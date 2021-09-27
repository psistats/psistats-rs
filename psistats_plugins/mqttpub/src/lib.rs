use libpsistats::{ PublisherFunction, InitFunction, PluginSettings };
use libpsistats::PluginRegistrar;
use libpsistats::PsistatsReport;
use libpsistats::PsistatsError;
use libpsistats::Commands;
use std::thread;
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use rumqttc::{MqttOptions, Client, QoS, Transport, Event, Packet };
use lazy_static::lazy_static;
use std::collections::HashMap;
use rustls::ClientConfig;
use rustls_native_certs;
use std::sync::mpsc;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
  registrar.register_init_fn("mqttpub", Box::new(Init));
  registrar.register_publisher_fn("mqttpub", Box::new(Publisher));
}
libpsistats::export_plugin!(register);

pub struct MqttWrapper {
  client: Client,
  topic_reports: String
}


impl MqttWrapper {

  pub fn init(hostname: &str, conf: &HashMap<String, toml::Value>, cmd_queue: mpsc::Sender<Commands>) -> Result<Self, PsistatsError> {
    let prefix = conf.get("topic_prefix").unwrap().as_str().unwrap();
    let topic_reports_suffix  = conf.get("topic_reports").unwrap().as_str().unwrap();
    let topic_commands_suffix = conf.get("topic_commands").unwrap().as_str().unwrap();

    let topic_reports = format!("{}/{}/{}", prefix, topic_reports_suffix, hostname);
    let topic_commands = format!("{}/{}", prefix, topic_commands_suffix);
    let topic_commands_reports = format!("{}/report", topic_commands);

    let mqttopts = || -> Result<MqttOptions, ()> {

      let mqtthost = conf.get("mqtthost").unwrap().as_str().unwrap();
      let mqttport = conf.get("mqttport").unwrap().as_str().unwrap();
      let username = conf.get("username").unwrap().as_str().unwrap();
      let password = conf.get("password").unwrap().as_str().unwrap();
      let client_id = format!("{}-{}", prefix, hostname);
      let usetls   = conf.get("usetls").unwrap().as_bool().unwrap();

      let mut mqttopts = MqttOptions::new(client_id, mqtthost, mqttport.parse::<u16>().unwrap());

      mqttopts.set_keep_alive(5);
      mqttopts.set_credentials(username, password);

      if usetls {
        let mut client_config = ClientConfig::new();
        client_config.root_store =
          rustls_native_certs::load_native_certs().expect("Failed to load platform certificates");

          mqttopts.set_transport(Transport::tls_with_config(client_config.into()));

      }
      return Ok(mqttopts);
    };

    let (mut client, mut connection) = Client::new(mqttopts().unwrap(), 10);

    client.subscribe(topic_commands_reports, QoS::AtMostOnce).unwrap();

    thread::spawn(move || {
      for (_i, notification) in connection.iter().enumerate() {
        let n = notification.unwrap();
        match n {
          Event::Incoming(incoming) => {
            match incoming {
              Packet::Publish(msg) => {
                cmd_queue.send(Commands::Report(String::from_utf8_lossy(&msg.payload).to_string()));
              },
              _ => ()
            }
          },
          Event::Outgoing(_) => ()
        }

      }
    });

    return Ok(MqttWrapper {
      client: client,
      topic_reports: String::from(topic_reports),
    });
  }

  pub fn send(&mut self, report: &PsistatsReport) -> Result<(), ()> {
    let json = report.as_json();
    let bytes = json.into_bytes();

    let topic = format!("{}/{}", self.topic_reports, report.reporter);

    self.client.publish(&topic, QoS::AtLeastOnce, false, bytes).unwrap();

    return Ok(());
  }

}

lazy_static! {
  pub static ref REPORT_CHANNEL: (Sender<PsistatsReport>, Receiver<PsistatsReport>) = unbounded();
}

pub fn start_mqtt_thread(hostname: &str, settings: &PluginSettings, cmd_queue: mpsc::Sender<Commands>) {
  let conf = settings.get_config().clone();

  let hn = String::from(hostname);

  thread::spawn(move || {
    let mut wrapper = MqttWrapper::init(&hn, &conf, cmd_queue).unwrap();
    loop {
      let report = REPORT_CHANNEL.1.recv().unwrap();
      wrapper.send(&report).unwrap();
    }
  });
}

#[derive(Debug, Clone, PartialEq)]
struct Init;

impl InitFunction for Init {
    fn call(&self, hostname: &str, settings: &PluginSettings, cmd_queue: mpsc::Sender<Commands>) -> Result<(), PsistatsError> {
      start_mqtt_thread(hostname, settings, cmd_queue);
      Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Commander;

impl Commander {
  fn call(&self, hostname: &str) {

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
