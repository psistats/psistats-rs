use rumqtt::{MqttClient, MqttOptions, QoS, Notification, SecurityOptions};
use crossbeam_channel::Receiver;
use crate::reporters::Report;
use toml::Value;
use toml::map::Map;
use std::sync::Mutex;
use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

lazy_static! {
    pub static ref CLIENT: Mutex<Mqtt> = Mutex::new(Mqtt::new());
}

pub struct Mqtt {
    client: Option<MqttClient>,
    queue: Option<Receiver<Notification>>,
    suffix: Option<String>
}

impl Mqtt {
    fn new() -> Mqtt {
        Mqtt {
            client: Option::None,
            queue: Option::None,
            suffix: Option::None
        }
    }

    fn get_client(&mut self, conf: &Map<String, Value>) -> Option<MqttClient> {
        match &self.client {
            Some(client) => {
                return Some(client.clone());
            },
            None => {
                let settings  = conf.get("settings").unwrap().as_table().unwrap();
                let hostname  = settings.get("hostname").unwrap().as_str().unwrap();

                let mqtt_conf = conf.get("p_mqtt").unwrap().as_table().unwrap();
                let suffix    = mqtt_conf.get("suffix").unwrap().as_str().unwrap();
                let publish_queue = format!("{}/{}", suffix, hostname);

                let mqtt_host = mqtt_conf.get("hostname").unwrap().as_str().unwrap();
                let mqtt_port = mqtt_conf.get("port").unwrap().as_integer().unwrap();
                let mqtt_user = mqtt_conf.get("username").unwrap().as_str().unwrap();
                let mqtt_pass = mqtt_conf.get("password").unwrap().as_str().unwrap();
                let client_id_prefix = mqtt_conf.get("client_id").unwrap().as_str().unwrap();
                let mut rng = thread_rng();
                let client_id_rnd: String = iter::repeat(())
                  .map(|()| rng.sample(Alphanumeric))
                  .take(8)
                  .collect();

                let client_id = format!("{}-{}", client_id_prefix, client_id_rnd);

                info!("Starting MQTT Connection: {}:{} w/ client id {}", mqtt_host, mqtt_port, client_id);

                let mut mqtt_options = MqttOptions::new(client_id, mqtt_host, mqtt_port as u16);

                if mqtt_user.to_string() != "" {
                    mqtt_options = mqtt_options.set_security_opts(SecurityOptions::UsernamePassword(mqtt_user.to_string(), mqtt_pass.to_string()));
                }

                match MqttClient::start(mqtt_options) {
                    Ok(ok) => {
                        let mut mqtt_client = ok.0;
                        let notifications = ok.1;

                        let commands_topic = format!("{}/commands", publish_queue);
                        mqtt_client.subscribe(commands_topic, QoS::AtLeastOnce).unwrap();

                        self.client = Option::from(mqtt_client.clone());
                        self.queue  = Option::from(notifications);
                        self.suffix = Option::from(publish_queue);

                        return Some(mqtt_client);
                    },
                    Err(err) => {
                        error!("{:?}", err);
                        return None;
                    }
                }

                /*
                let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();

                let commands_topic = format!("{}/commands", publish_queue);
                mqtt_client.subscribe(commands_topic, QoS::AtLeastOnce).unwrap();

                self.client = Option::from(mqtt_client.clone());
                self.queue  = Option::from(notifications);
                self.suffix = Option::from(publish_queue);

                return mqtt_client;
                */
            }
        }
    }
}


pub fn publish(conf: &Map<String, Value>, report: &Report) {
    let mut mqtt = CLIENT.lock().unwrap();
    let client   = mqtt.get_client(conf);


    match client {
        Some(mut c) => {
            let queue = format!("{}/reports/{}", mqtt.suffix.as_ref().unwrap(), report.id);
            c.publish(queue, QoS::AtLeastOnce, false, report.to_json()).unwrap();
        },
        None => ()
    }

    // client.publish(queue, QoS::AtLeastOnce, false, report.to_json()).unwrap();
}

pub fn commander(_conf: &Map<String, Value>) -> Option<String> {
    let mqtt = CLIENT.lock().unwrap();

    match &mqtt.queue {
        Some(queue) => {
            match queue.try_recv() {
                Ok(notification) => {
                    match notification {
                        Notification::Publish(msg) => {
                            let command = std::str::from_utf8(&msg.payload).unwrap();
                            Some(command.to_string())
                        }
                        _ => {
                            None
                        }
                    }
                },
                Err(_) => {
                    None
                }
            }
        },
        None => {
            None
        }
    }
}
