#[macro_use] extern crate log;

#[macro_use]
use psistats_service::{ ReporterFunction, ReporterInitFunction, ReporterConfig };
use psistats_service::PluginRegistrar;
use psistats_service::PsistatsReport;
use psistats_service::PluginError;
use psistats_service::FunctionType;


mod cpu;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send>) {
    debug!("Registering functions");
    registrar.register_plugin("cpu", FunctionType::ReporterInit(Box::new(Init)));
    registrar.register_plugin("cpu", FunctionType::Reporter(Box::new(Reporter)));
}
psistats_service::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl ReporterInitFunction for Init {
    fn call(&self, _: &ReporterConfig) -> Result<(), PluginError> {
        debug!("Initializing");
        cpu::start_cpu_thread();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, conf: &ReporterConfig) -> Result<PsistatsReport, PluginError> {
        let t = conf.get_config().get("show_total").unwrap().as_bool().unwrap();

        return cpu::get_report();
    }
}
