use libpsistats::{ ReporterFunction, PluginSettings, ReportValue };
use libpsistats::PluginRegistrar;
use libpsistats::PsistatsError;
use uptime_lib;

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
  fn call(&self, _: &PluginSettings) -> Result<ReportValue, PsistatsError> {
    match uptime_lib::get() {
      Ok(uptime) => {
        Ok(ReportValue::Integer(uptime.as_secs()))
      },
      Err(err) => {
        Err(PsistatsError::Runtime(format!("{}", err)))
      }
    }
  }
}

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
  registrar.register_reporter_fn("uptime", Box::new(Reporter));
}
libpsistats::export_plugin!(register);
