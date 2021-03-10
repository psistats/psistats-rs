#[macro_use]
use psistats::{ ReporterFunction, ReporterInitFunction, ReporterConfig };
use psistats::PluginRegistrar;
use psistats::PsistatsReport;
use psistats::PluginError;
use psistats::FunctionType;
use psistats::ReportValue;


mod cpu;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send>) {
  registrar.register_reporter_init("cpu", Box::new(Init));
  registrar.register_reporter("cpu", Box::new(Reporter));
}
psistats::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl ReporterInitFunction for Init {
    fn call(&self, _: &ReporterConfig) -> Result<(), PluginError> {
        cpu::start_cpu_thread();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, _: &ReporterConfig) -> Result<ReportValue, PluginError> {
        // let t = conf.get_config().get("show_total").unwrap().as_bool().unwrap();

        return cpu::get_report();
    }
}
