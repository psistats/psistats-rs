use libpsistats::{ ReporterFunction, InitFunction, PluginSettings };
use libpsistats::PluginRegistrar;
use libpsistats::PsistatsError;
use libpsistats::ReportValue;

#[cfg(target_os = "windows")]
mod ohmsensors;

#[cfg(not(target_os = "windows"))]
mod lmsensors;

mod sensors;



extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
  registrar.register_init_fn("sensors", Box::new(Init));
  registrar.register_reporter_fn("sensors", Box::new(Reporter));
}
libpsistats::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl InitFunction for Init {
    fn call(&self, _: &str, settings: &PluginSettings) -> Result<(), PsistatsError> {
        sensors::init(settings);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, config: &PluginSettings) -> Result<ReportValue, PsistatsError> {
        Ok(sensors::get_report(config))
    }
}
