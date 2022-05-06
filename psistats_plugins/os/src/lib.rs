use libpsistats::{ ReporterFunction, InitFunction, PluginSettings };
use libpsistats::PluginRegistrar;
use libpsistats::PsistatsError;
use libpsistats::ReportValue;
use std::env;
use std::collections::HashMap;
use os_info;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
  registrar.register_init_fn("os", Box::new(Init));
  registrar.register_reporter_fn("os", Box::new(Reporter));
}
libpsistats::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl InitFunction for Init {
    fn call(&self, _: &str, _: &PluginSettings) -> Result<(), PsistatsError> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, _: &PluginSettings) -> Result<ReportValue, PsistatsError> {

        let os = os_info::get();

        let os_type = format!("{}", os.os_type());
        let os_version = format!("{}", os.version());
        let os_codename = format!("{}", os.codename().unwrap_or(""));
        let os_edition = format!("{}", os.edition().unwrap_or(""));
        let os_bitness = format!("{}", os.bitness());

        

        let os_report: HashMap<String, ReportValue> = [
            ("type".to_string(), ReportValue::String(os_type)),
            ("version".to_string(), ReportValue::String(os_version)),
            ("codename".to_string(), ReportValue::String(os_codename)),
            ("edition".to_string(), ReportValue::String(os_edition)),
            ("bitness".to_string(), ReportValue::String(os_bitness))
        ].iter().cloned().collect();

        



        Ok(ReportValue::Object(os_report))
        // return memory::get_report();
    }
}
