use libpsistats::{CommandFunction, InitFunction, PluginSettings, PublisherFunction};
use libpsistats::PluginRegistrar;
use libpsistats::PsistatsReport;
use libpsistats::PsistatsError;
use libpsistats::Commands;
use std::time::Duration;
use std::thread;
use crossbeam_channel::{unbounded, bounded};

use crossbeam_channel::{Receiver, Sender};
use crossbeam::thread as cbthread;
use rumqttc::{Client, ClientError, Event, MqttOptions, Packet, QoS, Transport, TrySendError};
use lazy_static::lazy_static;
use rustls::ClientConfig;
use rustls_native_certs;


lazy_static! {
  pub static ref REPORT_CHANNEL: (Sender<PsistatsReport>, Receiver<PsistatsReport>) = unbounded();
  pub static ref COMMAND_CHANNEL: (Sender<Commands>, Receiver<Commands>) = unbounded();
  pub static ref SUBSCRIBE_CHANNEL: (Sender<String>, Receiver<String>) = bounded(5);
}

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
  registrar.register_init_fn("mqttpub", Box::new(Init));
  registrar.register_publisher_fn("mqttpub", Box::new(Publisher));
  registrar.register_command_fn("mqttpub", Box::new(Command));
}
libpsistats::export_plugin!(register);


pub fn get_mqtt_opts(hostname: &str, settings: &PluginSettings) -> Option<MqttOptions> {
  let conf = settings.get_config();

  let prefix = conf.get("topic_prefix").unwrap().as_str().unwrap();
  let mqtthost = conf.get("mqtthost").unwrap().as_str().unwrap();
  let mqttport = conf.get("mqttport").unwrap().as_str().unwrap();
  let username = conf.get("username").unwrap().as_str().unwrap();
  let password = conf.get("password").unwrap().as_str().unwrap();
  let client_id = format!("{}-{}", prefix, hostname);
  let usetls   = conf.get("usetls").unwrap().as_bool().unwrap();

  let mut mqttopts = MqttOptions::new(client_id, mqtthost, mqttport.parse::<u16>().unwrap());

  mqttopts.set_keep_alive(15);
  mqttopts.set_credentials(username, password);
  mqttopts.set_clean_session(true);

  if usetls {
    let mut client_config = ClientConfig::new();
    client_config.root_store =
      rustls_native_certs::load_native_certs().expect("Failed to load platform certificates");

      mqttopts.set_transport(Transport::tls_with_config(client_config.into()));

  }

  return Some(mqttopts);
}


pub fn start_mqtt_thread(hostname: &str, settings: &PluginSettings) {

  let host = String::from(hostname);
  let mqttopts = get_mqtt_opts(hostname, settings).unwrap();
  let conf = settings.get_config().clone();

  thread::spawn(move || {
    cbthread::scope(|s| {

      let (mut client, mut connection) = Client::new(mqttopts, 10);

      let prefix = conf.get("topic_prefix").unwrap().as_str().unwrap();
      let topic_reports_suffix  = conf.get("topic_reports").unwrap().as_str().unwrap();
      let topic_reports = format!("{}/{}/{}", prefix, topic_reports_suffix, host);

      let topic_commands_suffix = conf.get("topic_commands").unwrap().as_str().unwrap();
      let topic_commands = format!("{}/{}/{}", prefix, topic_commands_suffix, host);
      let topic_commands_reports = format!("{}/report", topic_commands);

      let mut c1 = client.clone();

      s.spawn(move |_| {
        loop {
          let topic = SUBSCRIBE_CHANNEL.1.recv().unwrap();
          match c1.try_subscribe(&topic, QoS::AtLeastOnce) {
            Ok(_) => {
              println!("mqtt: Subscribed to {}", topic);
            },
            Err(err) => {
              println!("mqtt: failed to subscribe to {} - {:?}", topic, err);
              std::thread::sleep(Duration::from_millis(1000));
              SUBSCRIBE_CHANNEL.0.send(topic.clone()).unwrap();
            }
          }
        }
      });

      s.spawn(move |_| {

        for (_i, notification) in connection.iter().enumerate() {
          match notification {
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
              SUBSCRIBE_CHANNEL.0.send(topic_commands_reports.clone()).unwrap();
            }
            Ok(Event::Incoming(Packet::Publish(msg))) => {

              let cmdstr = String::from_utf8_lossy(&msg.payload).to_string();
              let cmd = Commands::Report(cmdstr);

              COMMAND_CHANNEL.0.send(cmd).unwrap();
            }
            Ok(Event::Incoming(_)) => (),
            Ok(Event::Outgoing(_)) => (),
            Err(err) => {
              println!("mqtt: connection error: {:?}", err);
              std::thread::sleep(Duration::from_millis(1000));
            }
          }
        }
      });

      s.spawn(move |_| {

        loop {
          let report = REPORT_CHANNEL.1.recv().unwrap();

          let json = report.as_json();
          let bytes = json.into_bytes();

          let topic = format!("{}/{}", topic_reports, report.reporter);

          let res = client.try_publish(&topic, QoS::AtLeastOnce, false, bytes);
          match res {
            Ok(_) => (),
            Err(ClientError::TryRequest(TrySendError::Full(_))) => (),
            Err(err) => {
              println!("mqtt publish err: {:?}", err);
            }
          }
        }
      });
    }).unwrap();
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
struct Command;

impl CommandFunction for Command {
  #[allow(dead_code)]
  fn call(&self, hostname: &str, settings: &PluginSettings) -> Option<Commands> {
    match COMMAND_CHANNEL.1.try_recv() {
      Ok(c) => {
        return Some(c);
      },
      Err(_) => {
        return None;
      }
    }
  }
}


#[derive(Debug, Clone, PartialEq)]
struct Publisher;

impl PublisherFunction for Publisher {
  fn call(&self, report: PsistatsReport, _: &PluginSettings) -> Result<(), PsistatsError> {
    REPORT_CHANNEL.0.send(report).unwrap();
    Ok(())
  }
}
