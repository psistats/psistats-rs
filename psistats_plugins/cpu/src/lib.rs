use libpsistats::export_plugin;

use libpsistats::{ ReporterFunction, InitFunction, PluginSettings };
use libpsistats::PluginRegistrar;
use libpsistats::PsistatsError;
use libpsistats::ReportValue;


mod cpu;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
  registrar.register_init_fn("cpu", Box::new(Init));
  registrar.register_reporter_fn("cpu", Box::new(Reporter));
}
export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl InitFunction for Init {
    fn call(&self, _: &str, settings: &PluginSettings) -> Result<(), PsistatsError> {
      let mut combined = false;
      if settings.get_config().contains_key("combined") {
        combined = settings.get_config().get("combined").unwrap().as_bool().unwrap();
      }

      cpu::start_cpu_thread(combined);
      Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, _: &PluginSettings) -> Result<ReportValue, PsistatsError> {
        // let t = conf.get_config().get("show_total").unwrap().as_bool().unwrap();

        return cpu::get_report();
    }
}
