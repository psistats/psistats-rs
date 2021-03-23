#[macro_use]
use psistats::{ ReporterFunction, ReporterInitFunction, ReporterConfig };
use psistats::PluginRegistrar;
use psistats::PsistatsReport;
use psistats::FunctionType;
use psistats::PluginError;
use psistats::ReportValue;

// Define our initialization function

#[derive(Debug, Clone, PartialEq)]
struct ExampleInit;

static mut counter: u64 = 0;

impl ReporterInitFunction for ExampleInit {
    fn call(&self, conf: &ReporterConfig) -> Result<(), PluginError> {
        // Here is where one would initialise their plugin, if necessary
        unsafe {
            counter = 10;
        }
        Ok(())
    }
}



// Define our reporting function
#[derive(Debug, Clone, PartialEq)]
struct ExampleReporter;
impl ReporterFunction for ExampleReporter {
    fn call(&self, conf: &ReporterConfig) -> Result<PsistatsReport, PluginError> {
        unsafe {
            counter = counter + 1;
            Ok(PsistatsReport::new("example-reporter", ReportValue::Integer(counter)))
        }
    }
}


// Now expose a register function for our plugin. It will be called with the plugin registry
// where we can register our plugin at runtime.
extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send>) {
    registrar.register_plugin("example-reporter", FunctionType::ReporterInit(Box::new(ExampleInit)));
    registrar.register_plugin("example-reporter", FunctionType::Reporter(Box::new(ExampleReporter)));
}
psistats::export_plugin!(register);
