use libpsistats::{ ReporterFunction, InitFunction, PluginSettings, Commands };
use libpsistats::PluginRegistrar;
use libpsistats::PsistatsError;
use libpsistats::ReportValue;
use std::sync::mpsc::Sender;

mod memory;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
  registrar.register_init_fn("memory", Box::new(Init));
  registrar.register_reporter_fn("memory", Box::new(Reporter));
}
libpsistats::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl InitFunction for Init {
    fn call(&self, _: &str, _: &PluginSettings, _: Sender<Commands>) -> Result<(), PsistatsError> {
        memory::start_mem_thread();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, _: &PluginSettings) -> Result<ReportValue, PsistatsError> {
        return memory::get_report();
    }
}
