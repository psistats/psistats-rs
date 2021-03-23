use get_if_addrs::get_if_addrs;
use libpsistats::{ ReporterFunction, PluginSettings, ReportValue };
use libpsistats::PluginRegistrar;
use libpsistats::PsistatsError;


#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, _: &PluginSettings) -> Result<ReportValue, PsistatsError> {
        let mut ips = vec![];

        for iface in get_if_addrs().unwrap() {
            let ip = (iface.name, iface.addr.ip());
            ips.push(ReportValue::String(format!("{:#?}", ip)));
        }

        Ok(ReportValue::Array(ips))

    }
}

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
  registrar.register_reporter_fn("ipaddrs", Box::new(Reporter));
}
libpsistats::export_plugin!(register);
