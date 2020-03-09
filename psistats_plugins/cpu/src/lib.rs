#[macro_use]
use psistats_service::plugins::api::{
    PluginRegistrar, PsistatsInitFunction, 
    PsistatsReportFunction, PsistatsError, 
    PsistatsReport
};

mod cpu;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar>) {
    println!("psistats-cpu: register() called");
    registrar.register_init_fn("cpu", Box::new(Init));
    registrar.register_report_fn("cpu", Box::new(Reporter));
}
psistats_service::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl PsistatsInitFunction for Init {
    fn call(&self) -> Result<(), PsistatsError> {
        println!("CPU Plugin Init Function Called!");
        cpu::start_cpu_thread();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl PsistatsReportFunction for Reporter {
    fn call(&self) -> Result<PsistatsReport, PsistatsError> {
        println!("CPU Plugin report function called!");
        return cpu::get_report();
    }
}
