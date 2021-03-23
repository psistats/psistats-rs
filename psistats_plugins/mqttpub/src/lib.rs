use libpsistats::{ PublisherFunction, InitFunction, PluginSettings };
use libpsistats::PluginRegistrar;
use libpsistats::PsistatsReport;
use libpsistats::PsistatsError;
use std::thread;
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use rumqttc::{MqttOptions, Client, QoS };
use lazy_static::lazy_static;
use std::collections::HashMap;



extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
  registrar.register_init_fn("mqttpub", Box::new(Init));
  registrar.register_publisher_fn("mqttpub", Box::new(Publisher));
}
libpsistats::export_plugin!(register);

pub struct MqttWrapper {
  client: Client,
  path: String,
}


impl MqttWrapper {

  pub fn init(hostname: &str, conf: &HashMap<String, toml::Value>) -> Result<Self, PsistatsError> {
    let prefix = conf.get("prefix").unwrap().as_str().unwrap();

    let mqttopts = || -> Result<MqttOptions, ()> {

      let mqtthost = conf.get("mqtthost").unwrap().as_str().unwrap();
      let mqttport = conf.get("mqttport").unwrap().as_str().unwrap();
      let username = conf.get("username").unwrap().as_str().unwrap();
      let password = conf.get("password").unwrap().as_str().unwrap();
      let client_id = format!("{}-{}", prefix, hostname);

      let mut mqttopts = MqttOptions::new(client_id, mqtthost, mqttport.parse::<u16>().unwrap());

      mqttopts.set_keep_alive(5);
      mqttopts.set_credentials(username, password);
      return Ok(mqttopts);
    };

    let (client, mut connection) = Client::new(mqttopts().unwrap(), 10);

    let topic = format!("{}/{}", prefix, hostname);

    thread::spawn(move || loop {
      for _notification in connection.iter() {

      }
    });

    return Ok(MqttWrapper {
      client: client,
      path: String::from(topic),
    });
  }

  pub fn send(&mut self, report: &PsistatsReport) -> Result<(), ()> {
    let json = report.as_json();
    let bytes = json.into_bytes();

    match self.client.publish(&self.path, QoS::AtLeastOnce, false, bytes) {
      Ok(_) => (),
      Err(e) => {
         println!("mqtt ERROR: {:?}", e);
         return Err(());
      }
    }


    return Ok(());
  }
}

lazy_static! {
  pub static ref REPORT_CHANNEL: (Sender<PsistatsReport>, Receiver<PsistatsReport>) = unbounded();
}

pub fn start_mqtt_thread(hostname: &str, settings: &PluginSettings) {
  let conf = settings.get_config();
  //let hostname = conf.get("hostname").unwrap().as_str().unwrap();

  let mut wrapper = MqttWrapper::init(&hostname, conf).unwrap();

  thread::spawn(move || loop {
    let report = REPORT_CHANNEL.1.recv().unwrap();
    wrapper.send(&report).unwrap();
  });
}

#[derive(Debug, Clone, PartialEq)]
struct Init;

impl InitFunction for Init {
    fn call(&self, hostname: &str, settings: &PluginSettings) -> Result<(), PsistatsError> {
      start_mqtt_thread(hostname, settings);

      Ok(())
    }
}


#[derive(Debug, Clone, PartialEq)]
struct Publisher;

impl PublisherFunction for Publisher {
    fn call(&self, report: PsistatsReport, _: &PluginSettings) -> Result<(), PsistatsError> {
      match REPORT_CHANNEL.0.send(report) {
        Ok(_) => (),
        Err(e) => {
          return Err(
            PsistatsError::Runtime(format!("Failed to send report to crossbeam channel: {:?}", e))
          );
        }
      }
      Ok(())
    }
}
