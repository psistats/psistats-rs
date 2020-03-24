#[macro_use] extern crate log;

#[macro_use]
use psistats_service::{ ReporterFunction, ReporterInitFunction, ReporterConfig };
use psistats_service::PluginRegistrar;
use psistats_service::PsistatsReport;
use psistats_service::PluginError;
use psistats_service::FunctionType;
use psistats_service::ReportValue;


extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send>) {
    registrar.register_plugin("foobar", FunctionType::ReporterInit(Box::new(Init)));
    registrar.register_plugin("foobar", FunctionType::Reporter(Box::new(Reporter)));
}
psistats_service::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl ReporterInitFunction for Init {
    fn call(&self, _: &ReporterConfig) -> Result<(), PluginError> {
      println!("Foobar init");
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, _: &ReporterConfig) -> Result<PsistatsReport, PluginError> {
      println!("Foobar reporter called!");

      let r = PsistatsReport::new("foobar", ReportValue::String("FoobaR!".to_string()));

      Ok(r)
    }
}
