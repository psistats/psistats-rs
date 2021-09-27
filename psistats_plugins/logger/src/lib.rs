extern crate pretty_env_logger;
#[macro_use] extern crate log;

use libpsistats::{ PublisherFunction, InitFunction, PluginSettings, Commands };
use libpsistats::PluginRegistrar;
use libpsistats::PsistatsReport;
use libpsistats::PsistatsError;
use std::sync::mpsc::Sender;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
  registrar.register_init_fn("logger", Box::new(Init));
  registrar.register_publisher_fn("logger", Box::new(Publisher));
}
libpsistats::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl InitFunction for Init {
    fn call(&self, _: &str, _: &PluginSettings, _: Sender<Commands>) -> Result<(), PsistatsError> {
      pretty_env_logger::init();
      Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Publisher;

impl PublisherFunction for Publisher {
    fn call(&self, report: PsistatsReport, _: &PluginSettings) -> Result<(), PsistatsError> {
      info!("{:?}", report);
      Ok(())
    }
}
