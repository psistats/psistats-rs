#[macro_use]
use psistats::{ ReporterFunction, ReporterInitFunction, ReporterConfig };
use psistats::PluginRegistrar;
use psistats::PsistatsReport;
use psistats::PluginError;
use psistats::FunctionType;


mod memory;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send>) {
    registrar.register_plugin("memory", FunctionType::ReporterInit(Box::new(Init)));
    registrar.register_plugin("memory", FunctionType::Reporter(Box::new(Reporter)));
}
psistats::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl ReporterInitFunction for Init {
    fn call(&self, _: &ReporterConfig) -> Result<(), PluginError> {
        memory::start_mem_thread();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, _: &ReporterConfig) -> Result<PsistatsReport, PluginError> {
        return memory::get_report();
    }
}
