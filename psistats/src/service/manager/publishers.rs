use crate::PsistatsReport;
use crate::PluginRegistrar;
use crate::service::manager::PluginCommands;
use crate::{ ServiceConfig, PublisherConfig, PsistatsSettings };
use std::sync::{Mutex, Arc};
use crossbeam_channel::{Receiver, Sender};
use std::rc::Rc;

pub struct ManagerBuilder<'a> {
  r_recv: Option<Receiver<PsistatsReport>>,
  c_send: Option<Sender<PluginCommands>>,
  registrar: Option<&'a Arc<Mutex<Box<dyn PluginRegistrar + Send>>>>
}

impl<'a> ManagerBuilder<'a> {
  pub fn new() -> Self {
    Self {
      r_recv: None,
      c_send: None,
      registrar: None
    }
  }

  pub fn set_reporter_channel(&mut self, r: Receiver<PsistatsReport>) -> &mut Self {
    self.r_recv = Some(r);
    self
  }

  pub fn set_command_channel(&mut self, s: Sender<PluginCommands>) -> &mut Self {
    self.c_send = Some(s);
    self
  }

  pub fn set_registrar(&mut self, registrar: &'a Arc<Mutex<Box<dyn PluginRegistrar + Send>>>) -> &mut Self {
    self.registrar = Some(registrar);
    self
  }

  pub fn build(&self, conf: &'a ServiceConfig) -> Manager {

    let publisher_configs: Vec<&PublisherConfig> = conf.get_publisher_configs().iter().filter(|conf| conf.is_enabled()).collect();

    let rm = Manager {
      r_recv: self.r_recv.as_ref().unwrap().clone(),
      c_send: self.c_send.as_ref().unwrap().clone(),
      registrar: &self.registrar.as_ref().unwrap().clone(),
      publisher_configs: publisher_configs,
      settings: &conf.settings
    };

    return rm;
  }
}

pub struct Manager<'a> {
  r_recv: Receiver<PsistatsReport>,
  c_send: Sender<PluginCommands>,
  registrar: &'a Arc<Mutex<Box<dyn PluginRegistrar + Send>>>,
  publisher_configs: Vec<&'a PublisherConfig>,
  settings: &'a PsistatsSettings
}

impl<'a> Manager<'a> {

  pub fn run_publishers(&'a self) {
    match self.r_recv.recv() {
      Ok(report) => {

        //let r = Rc::new(report);

        for pconf in self.publisher_configs.iter() {
          let reg = self.registrar.lock().unwrap();
          let publisher = reg.get_publisher(pconf.get_name()).unwrap();
          publisher.call(report.clone(), pconf, self.settings).unwrap();
        }
      },
      Err(_) => {
        error!("Error getting received");
      }
    }
  }
}
