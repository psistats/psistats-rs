use crate::reporters::Report;
use crate::reporters::ReporterCb;
use crate::reporters::get_reporter;
use crate::publishers::{get_publisher, get_commander, Publisher, Commander};

use std::thread;
use crossbeam_channel::{Receiver, Sender};
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::time::Duration;
use toml;
use toml::Value;
use std::str;
use scoped_threadpool::Pool;

pub fn start_publishers(conf: Value, reports_receiver: Receiver<Report>, commands_sender: Sender<String>) -> JoinHandle<()> {
    return thread::spawn(move || {

        let mut publishers: Vec<Publisher> = Vec::new();
        let mut commanders: Vec<Commander> = Vec::new();
        let c = conf.as_table().unwrap();

        for conf_section in c.keys() {
            if conf_section.starts_with("p_") {

                let publisher_id = str::replace(conf_section, "p_", "");
                let enabled = c[conf_section]["enabled"].as_bool().unwrap();

                if enabled == true {

                  match get_publisher(publisher_id.clone()) {
                      Some(publisher) => {
                          info!("Enabled publisher: {}", publisher_id);
                          publishers.push(publisher);
                      }
                      None => ()
                  }
                  match get_commander(publisher_id.clone()) {
                      Some(publisher) => {
                          info!("Enabled commander: {}", publisher_id);
                          commanders.push(publisher);
                      }
                      None => ()
                  }
                }
            }
        }

        loop {
            for commander in &commanders {
                match commander(&c) {
                    Some(cmd) => {
                        commands_sender.send(cmd).unwrap();
                    }
                    None => ()
                }
            }

            match reports_receiver.try_recv() {
                Ok(report) => {
                    for publisher in &publishers {
                        publisher(&c, &report);
                    }
                },
                Err(_) => ()
            }

            thread::sleep(Duration::from_millis(100));
        }
    });
}

pub fn start_reporters(conf: Value, reports_sender: Sender<Report>, commands_receiver: Receiver<String>) -> JoinHandle<()> {

    return thread::spawn(move || {

        let c = conf.as_table().unwrap();
        let settings = c["settings"].as_table().unwrap();

        let workers = settings["workers"].as_integer().unwrap() as u64;
        let timer   = settings["timer"].as_integer().unwrap() as u64;

        let mut reporters: HashMap<u64, Vec<ReporterCb>> = HashMap::new();

        let mut max_counter = 1;

        for conf_section in c.keys() {
            if conf_section.starts_with("r_") {
                let reporter_id = str::replace(conf_section, "r_", "");
                let enabled = c[conf_section]["enabled"].as_bool().unwrap();

                if enabled == true {
                  match get_reporter(reporter_id.clone()) {
                      Some(reporter) => {
                          info!("Enabled reporter: {}", reporter_id);
                          let interval = c[conf_section]["interval"].as_integer().unwrap() as u64;
                          if interval > max_counter {
                              max_counter = interval;
                          }

                          if reporters.contains_key(&interval) {
                              reporters.get_mut(&interval).unwrap().push(reporter);
                          } else {
                              reporters.insert(interval, Vec::new());
                              reporters.get_mut(&interval).unwrap().push(reporter);
                          }
                      },
                      None => println!("No reporter found for {}", reporter_id)
                  }
                }
            }
        }

        let mut pool = Pool::new(workers as u32);
        let mut counter = 1;
        let intervals: Vec<u64> = reporters.keys().cloned().filter(|i| {
            *i > 0 // Only schedule reporters that have an interval greater than 0
        }).map(|i| i.clone()).collect();

        pool.scoped(|scoped| {

            loop {

                match commands_receiver.try_recv() {
                    Ok(cmd) => {
                        println!("Reporters thread received {}", cmd);
                        match get_reporter(cmd.clone()) {
                            Some(reporter) => {
                                let report = reporter(c);
                                reports_sender.send(report).unwrap();
                            }
                            None => println!("{} reporter not found", cmd)
                        }
                    },
                    Err(_) => ()
                };

                counter = counter + 1;

                if counter > max_counter {
                    counter = 1;
                }

                for interval in &intervals {
                    if counter % interval == 0 {

                        let rlist = reporters.get(&interval).unwrap();

                        for reporter in rlist {
                            let rs_clone = reports_sender.clone();

                            scoped.execute(move || {
                                let report = reporter(c);
                                rs_clone.send(report).unwrap();
                            });

                        }
                    }
                }
                thread::sleep(Duration::from_millis(timer));
            }
        });
    });
}
