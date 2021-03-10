use crate::PsistatsReport;
use crate::PluginRegistrar;
use crate::service::manager::PluginCommands;
use crate::{ ServiceConfig, ReporterConfig };

use std::sync::{Mutex, Arc};
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;

pub struct ManagerBuilder<'a> {
  r_send: Option<Sender<PsistatsReport>>,
  c_recv: Option<Receiver<PluginCommands>>,
  registrar: Option<&'a Arc<Mutex<Box<dyn PluginRegistrar + Send>>>>
}

impl<'a> ManagerBuilder<'a> {
  pub fn new() -> Self {
    Self {
      r_send: None,
      c_recv: None,
      registrar: None
    }
  }

  pub fn set_reporter_channel(&mut self, s: Sender<PsistatsReport>) -> &mut Self {
    self.r_send = Some(s);
    self
  }

  pub fn set_command_channel(&mut self, r: Receiver<PluginCommands>) -> &mut Self {
    self.c_recv = Some(r);
    self
  }

  pub fn set_registrar(&mut self, registrar: &'a Arc<Mutex<Box<dyn PluginRegistrar + Send>>>) -> &mut Self {
    self.registrar = Some(registrar);
    self
  }

  pub fn build(&self, conf: &'a ServiceConfig) -> Manager {

    let (reporter_map, reporter_intervals) = get_reporter_intervals(conf.get_reporter_configs());

    let mut reporter_config_map = HashMap::new();
    let rconfs = conf.get_reporter_configs();

    for rconf in rconfs.iter() {
      reporter_config_map.insert(rconf.get_name().to_string(), rconf);
    }

    let rm = Manager {
      report_send: self.r_send.as_ref().unwrap().clone(),
      commands_recv: self.c_recv.as_ref().unwrap().clone(),
      registrar: &self.registrar.as_ref().unwrap().clone(),
      reporter_interval_map: reporter_map,
      reporter_intervals: reporter_intervals,
      reporter_configs: reporter_config_map
    };

    return rm;
  }
}

pub struct Manager<'a> {
  report_send: Sender<PsistatsReport>,
  commands_recv: Receiver<PluginCommands>,
  reporter_interval_map: HashMap<u32, Vec<String>>,
  reporter_intervals: Vec<u32>,
  reporter_configs: HashMap<String, &'a ReporterConfig>,
  registrar: &'a Arc<Mutex<Box<dyn PluginRegistrar + Send>>>
}

impl<'a> Manager<'a> {

  pub fn get_report_sender(&'a self) -> Sender<PsistatsReport> {
    return self.report_send.clone();
  }

  pub fn max_count(&'a self) -> &u32 {
    return self.reporter_intervals.iter().max().unwrap();
  }

  pub fn run_reporters(&'a self, counter: &u32) {
    for interval in &self.reporter_intervals {
      if counter % interval == 0 {
        let rlist = self.reporter_interval_map.get(&interval).unwrap();
        for reporter_name in rlist {
          let reg = self.registrar.lock().unwrap();
          let reporter = reg.get_reporter(reporter_name).unwrap();
          let rconf = self.reporter_configs.get(reporter_name).unwrap();
          let output = reporter.call(rconf).unwrap();

          let report = PsistatsReport::new(reporter_name, output);
          self.report_send.send(report).unwrap();
        }
      }
    }
  }
}


fn get_reporter_intervals(rconfs: &Vec<ReporterConfig>) -> (HashMap<u32, Vec<String>>, Vec<u32>) {
  let mut reporters: HashMap<u32, Vec<String>> = HashMap::new();

  for rconf in rconfs {

    if rconf.is_enabled() {

      let interval = rconf.get_interval();

      if !reporters.contains_key(&interval) {
        reporters.insert(interval, Vec::new());
      }

      reporters.get_mut(&interval).unwrap().push(rconf.get_name().to_string());
    }
  }


  let intervals: Vec<u32> = reporters.keys().cloned().filter(|i| {
    *i > 0
  }).map(|i| i.clone()).collect();

  return (reporters, intervals);
}

