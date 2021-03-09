extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[macro_use]
use psistats::{ PublisherFunction, PublisherInitFunction, PublisherConfig };
use psistats::PluginRegistrar;
use psistats::PsistatsReport;
use psistats::PluginError;
use psistats::FunctionType;
use psistats::PsistatsSettings;

use std::rc::Rc;


extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send>) {
    registrar.register_plugin("logger", FunctionType::PublisherInit(Box::new(Init)));
    registrar.register_plugin("logger", FunctionType::Publisher(Box::new(Publisher)));
}
psistats::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl PublisherInitFunction for Init {
    fn call(&self, _: &PublisherConfig, _: &PsistatsSettings) -> Result<(), PluginError> {
      pretty_env_logger::init();
      Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Publisher;

impl PublisherFunction for Publisher {
    fn call(&self, report: PsistatsReport, _: &PublisherConfig, _: &PsistatsSettings) -> Result<(), PluginError> {
      info!("{:?}", report);

      // let r = PsistatsReport::new("foobar", ReportValue::String("FoobaR!".to_string()));

      Ok(())
    }
}
