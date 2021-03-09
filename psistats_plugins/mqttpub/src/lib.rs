use psistats::{ PublisherFunction, PublisherInitFunction, PublisherConfig };
use psistats::PluginRegistrar;
use psistats::PsistatsReport;
use psistats::PluginError;
use psistats::FunctionType;
use psistats::PsistatsSettings;
use std::thread;
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use rumqttc::{MqttOptions, Client, QoS };
use lazy_static::lazy_static;
use std::collections::HashMap;



extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send>) {
  registrar.register_plugin("mqttpub", FunctionType::PublisherInit(Box::new(Init)));
  registrar.register_plugin("mqttpub", FunctionType::Publisher(Box::new(Publisher)));
}
psistats::export_plugin!(register);

pub struct MqttWrapper {
  client: Client,
  path: String,
}


impl MqttWrapper {

  pub fn init(hostname: &str, conf: &HashMap<String, toml::Value>) -> Result<Self, PluginError> {
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

pub fn start_mqtt_thread(conf: &PublisherConfig, settings: &PsistatsSettings) {
  let mut wrapper = MqttWrapper::init(&settings.hostname, conf.get_config()).unwrap();

  thread::spawn(move || loop {
    let report = REPORT_CHANNEL.1.recv().unwrap();
    wrapper.send(&report).unwrap();
  });
}

#[derive(Debug, Clone, PartialEq)]
struct Init;

impl PublisherInitFunction for Init {
    fn call(&self, conf: &PublisherConfig, settings: &PsistatsSettings) -> Result<(), PluginError> {
      start_mqtt_thread(conf, settings);

      Ok(())
    }
}


#[derive(Debug, Clone, PartialEq)]
struct Publisher;

impl PublisherFunction for Publisher {
    fn call(&self, report: PsistatsReport, _: &PublisherConfig, _: &PsistatsSettings) -> Result<(), PluginError> {
      match REPORT_CHANNEL.0.send(report) {
        Ok(_) => (),
        Err(e) => {
          println!("report channel error: {:?}", e);
        }
      }
      Ok(())
    }
}
