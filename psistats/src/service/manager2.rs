use std::thread;
use crate::ServiceConfig;
use crate::PluginRegistrar;
use crate::ReporterConfig;
use crate::PublisherConfig;
use crate::PsistatsReport;
use std::collections::HashMap;
use std::sync::{Mutex,Arc};
use std::time::Duration;
use crossbeam_channel::{Receiver, Sender, unbounded};

pub enum PluginCommands {
  FORCE_REPORT,
  REPORT(PsistatsReport)
}

pub fn start_reporters(conf: &ServiceConfig, registrar: &Arc<Mutex<Box<dyn PluginRegistrar + Send>>>) -> (Sender<PsistatsReport>,  Receiver<PsistatsReport>) {

  let (r_send, r_recv) = unbounded();
  let (c_send, c_recv) = unbounded();


  let confclone = conf.clone();
  let reg =  registrar.clone();
  let r_send_clone = r_send.clone();
  let r_recv_clone = r_recv.clone();
  let c_send_clone = c_send.clone();
  let c_recv_clone = c_recv.clone();

  thread::spawn(move || {

    let mut builder = ManagerBuilder::new();
    builder.set_reporter_channel(r_send_clone)
      .set_command_channel(c_recv_clone)
      .set_registrar(&reg);

    let rm = builder.build(&confclone);

    let max_count = rm.max_count();
    let mut counter = 0;

    loop {
      debug!("Reporter iteration");
      counter = counter + 1;

      if counter > *max_count {
        counter =  1;
      }

      rm.run_reporters(&counter);
      thread::sleep(Duration::from_secs(1));
    }
  });

  return (r_send, report_r);
}

pub struct ManagerBuilder<'a> {
  r_recv: Option<Receiver<PsistatsReport>>,
  r_send: Option<Sender<PsistatsReport>>,
  c_recv: Option<Receiver<PluginCommands>>,
  c_send: Option<Sender<PluginCommands>>,
  registrar: Option<&'a Arc<Mutex<Box<dyn PluginRegistrar + Send>>>>
}

impl<'a> ManagerBuilder<'a> {
    fn new() -> Self {
      Self {
        r_recv: None,
        r_send: None,
        c_recv: None,
        c_send: None,
        registrar: None
      }
    }

    pub fn set_reporter_channels(&mut self, r: Receiver<PsistatsReport>, s: Sender<PsistatsReport>) -> &mut Self {
      self.r_recv = Some(r);
      self.r_send = Some(s);
      self
    }

    pub fn set_command_channels(&mut self, r: Receiver<PluginCommands>, s: Sender<PluginCommands>) -> &mut Self {
      self.c_recv = Some(r);
      self.c_send = Some(s);
      self
    }

    pub fn set_registrar(&mut self, registrar: &'a Arc<Mutex<Box<dyn PluginRegistrar + Send>>>) -> &mut Self {
      self.registrar = Some(registrar);
      self
    }

    pub fn build(&self, conf: &'a ServiceConfig) -> ReporterManager {

      let (reporter_map, reporter_intervals) = get_reporter_intervals(conf.get_reporter_configs());

      let mut reporter_config_map = HashMap::new();
      let rconfs = conf.get_reporter_configs();

      for rconf in rconfs.iter() {
        reporter_config_map.insert(rconf.get_name().to_string(), rconf);
      }

      let rm = ReporterManager {
        report_recv: self.r_recv.as_ref().unwrap().clone(),
        report_send: self.r_send.as_ref().unwrap().clone(),
        commands_recv: self.c_recv.as_ref().unwrap().clone(),
        commands_send: self.c_send.as_ref().unwrap().clone(),
        registrar: &self.registrar.as_ref().unwrap().clone(),
        reporter_interval_map: reporter_map,
        reporter_intervals: reporter_intervals,
        reporter_configs: reporter_config_map
      };

      return rm;
    }
}

pub struct ReporterManager<'a> {
  report_recv: Receiver<PsistatsReport>,
  report_send: Sender<PsistatsReport>,
  commands_recv: Receiver<PluginCommands>,
  commands_send: Sender<PluginCommands>,
  reporter_interval_map: HashMap<u32, Vec<String>>,
  reporter_intervals: Vec<u32>,
  reporter_configs: HashMap<String, &'a ReporterConfig>,
  registrar: &'a Arc<Mutex<Box<dyn PluginRegistrar + Send>>>
}

impl<'a> ReporterManager<'a> {

  pub fn get_report_recv(&'a self) -> Receiver<PsistatsReport> {
    return self.report_recv.clone();
  }

  pub fn get_report_sender(&'a self) -> Sender<PsistatsReport> {
    return self.report_send.clone();
  }

  pub fn build(conf: &'a ServiceConfig, registrar: &'a Arc<Mutex<Box<dyn PluginRegistrar + Send>>>) -> Self {
    let (report_s, report_r) = unbounded();
    let (cmd_s, cmd_r) = unbounded();

    let (reporter_map, reporter_intervals) = get_reporter_intervals(conf.get_reporter_configs());

    let mut reporter_config_map = HashMap::new();
    let rconfs = conf.get_reporter_configs();

    for rconf in rconfs.iter() {
      reporter_config_map.insert(rconf.get_name().to_string(), rconf);
    }



    ReporterManager {
      report_recv: report_r,
      report_send: report_s,
      commands_recv: cmd_r,
      commands_send: cmd_s,
      reporter_interval_map: reporter_map,
      reporter_intervals: reporter_intervals,
      reporter_configs: reporter_config_map,
      registrar: registrar
    }
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
          let report = reporter.call(rconf).unwrap();
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

