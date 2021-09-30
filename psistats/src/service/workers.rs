use std::{collections::HashMap, time::{Duration, Instant}};

use crossbeam_channel::{Receiver, Sender, unbounded};
use libpsistats::{CommandFunction, Commands, PsistatsReport, PublisherFunction, ReporterFunction};

use super::config::PsistatsConfiguration;
/**
theory of operation:


loop {



}


*/
// This worker listens for reports and sends them to publishers
pub struct PublisherWorker<'a> {
  report_queue: Receiver<PsistatsReport>,
  publisher_fns: &'a HashMap<String, Box<dyn PublisherFunction + Send + Sync>>,
  config: &'a PsistatsConfiguration
}

impl<'a> PublisherWorker<'a> {
  pub fn init(publisher_fns: &'a HashMap<String, Box<dyn PublisherFunction + Send + Sync>>,
          config: &'a PsistatsConfiguration) -> (Sender<PsistatsReport>, PublisherWorker<'a>) {
    let (report_tx, report_rx) = unbounded();

    return (report_tx, PublisherWorker {
      report_queue: report_rx,
      publisher_fns: publisher_fns,
      config: config
    });
  }
  pub fn poll(&self) {
    let report = self.report_queue.recv();

    match report {
      Ok(r) => {
        for (publisher_name, publisher_fn) in self.publisher_fns {
          let settings = self.config.get_plugin_config(publisher_name).unwrap();
          publisher_fn.call(r.clone(), &settings).unwrap();
        }
        //publisherFn.call(r, settings).unwrap();
      },
      _ => ()

    }
  }
}

// This worker listens for report requests, and gets report values for the publisher queue
pub struct ReporterWorker<'a> {
  request_queue: Receiver<String>,
  report_queue: Sender<PsistatsReport>,
  reporter_fns: &'a HashMap<String, Box<dyn ReporterFunction + Send + Sync>>,
  config: &'a PsistatsConfiguration
}

impl<'a> ReporterWorker<'a> {
  pub fn init(report_queue: Sender<PsistatsReport>,
    reporter_fns: &'a HashMap<String, Box<dyn ReporterFunction + Send + Sync>>,
    config: &'a PsistatsConfiguration) -> (Sender<String>, ReporterWorker<'a>) {

    let (request_tx, request_rx) = unbounded();

    return (request_tx, ReporterWorker {
      request_queue: request_rx,
      report_queue: report_queue,
      reporter_fns: reporter_fns,
      config: config
    });
  }

  pub fn poll(&self) {
    let report_request = self.request_queue.recv();
    match report_request {
      Ok(reporter_name) => {

        let reporter_fn = self.reporter_fns.get(&reporter_name).unwrap();
        let settings = self.config.get_plugin_config(&reporter_name).unwrap();
        let report_value = reporter_fn.call(&settings).unwrap();
        let report = PsistatsReport::new(&reporter_name, &self.config.settings.hostname, report_value);
        self.report_queue.send(report).unwrap();
      },
      _ => ()
    }
  }
}

// This worker will request reports based on their configured intervals
pub struct ScheduleWorker<'a> {
  intervals: HashMap<u64, Vec<&'a str>>,

  #[allow(dead_code)]
  config: &'a PsistatsConfiguration,
  now: Instant,
  request_queue: Sender<String>
}

impl<'a> ScheduleWorker<'a> {
  pub fn init(request_queue: Sender<String>,
    reporter_fns: &'a HashMap<String, Box<dyn ReporterFunction + Send + Sync>>,
    config: &'a PsistatsConfiguration) -> ScheduleWorker<'a> {

    let mut reporter_intervals: HashMap<u64, Vec<&str>> = HashMap::new();
    let now = Instant::now();



    for (pluginname, _) in reporter_fns {
      let pconf = config.get_plugin_config(pluginname).unwrap();
      let interval = pconf.get_interval();

      if !reporter_intervals.contains_key(&interval) {
        reporter_intervals.insert(interval, vec![pluginname]);
      } else {
        reporter_intervals.get_mut(&interval).unwrap().push(pluginname);
      }
    }

    return ScheduleWorker {
      intervals: reporter_intervals,
      config: config,
      now: now,
      request_queue: request_queue
    }
  }

  pub fn poll(&self) {
    for (interval, plugins) in &self.intervals {
      let elapsed_secs = self.now.elapsed().as_secs();
      if elapsed_secs > 0 && interval > &0 && elapsed_secs % interval == 0 {
        for plugin_name in plugins {
          self.request_queue.send(plugin_name.to_string()).unwrap();
        }
      }
    }
  }
}

// This worker sees if any plugins have issued any commands and deals with them
pub struct CommandWorker<'a> {
  command_queue: Receiver<Commands>,
  request_queue: Sender<String>,

  #[allow(dead_code)]
  config: &'a PsistatsConfiguration,

  #[allow(dead_code)]
  command_fns: &'a HashMap<String, Box<dyn CommandFunction + Send + Sync>>,
}

impl<'a> CommandWorker<'a> {
  pub fn init(request_queue: Sender<String>,
    command_fns: &'a HashMap<String, Box<dyn CommandFunction + Send + Sync>>,
    config: &'a PsistatsConfiguration) -> (Sender<Commands>, CommandWorker<'a>) {

    let (command_tx, command_rx) = unbounded();

    return (command_tx, CommandWorker {
      request_queue: request_queue,
      command_queue: command_rx,
      command_fns: command_fns,
      config: config
    });
  }

  pub fn poll(&self) {
    for (plugin_name, cmd_fn) in self.command_fns {
      let command = cmd_fn.call(&self.config.settings.hostname, &self.config.get_plugin_config(plugin_name).unwrap());

      match command {
        Some(cmd) => {
          info!("Got command: {:?}", cmd);
          match cmd {
            Commands::Report(report_name) => {
              self.request_queue.send(report_name).unwrap();
            }
          }
        },
        None => ()
      }
    }
    std::thread::sleep(Duration::from_millis(1000));
  }
}