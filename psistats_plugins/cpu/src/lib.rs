#[macro_use]
use psistats_service::{ ReporterFunction, ReporterInitFunction, ReporterConfig };
use psistats_service::PluginRegistrar;
use psistats_service::PsistatsReport;
use psistats_service::PluginError;
use psistats_service::FunctionType;


mod cpu;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar>) {
    println!("psistats-cpu: register() called");
    registrar.register_plugin("cpu", FunctionType::ReporterInit(Box::new(Init)));
    registrar.register_plugin("cpu", FunctionType::Reporter(Box::new(Reporter)));
}
psistats_service::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl ReporterInitFunction for Init {
    fn call(&self, _: &ReporterConfig) -> Result<(), PluginError> {
        println!("CPU Plugin Init Function Called!");
        cpu::start_cpu_thread();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, _: &ReporterConfig) -> Result<PsistatsReport, PluginError> {
        println!("CPU Plugin report function called!");
        return cpu::get_report();
    }
}
