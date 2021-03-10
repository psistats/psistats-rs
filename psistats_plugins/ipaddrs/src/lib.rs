use get_if_addrs::get_if_addrs;
use psistats::{ ReporterFunction, ReporterConfig, ReportValue };
use psistats::PluginRegistrar;
use psistats::PsistatsReport;
use psistats::PluginError;
use psistats::FunctionType;

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, _: &ReporterConfig) -> Result<ReportValue, PluginError> {
        let mut ips = vec![];

        for iface in get_if_addrs().unwrap() {
            let ip = (iface.name, iface.addr.ip());
            ips.push(ReportValue::String(format!("{:#?}", ip)));
        }

        Ok(ReportValue::Array(ips))

    }
}

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send>) {
    registrar.register_plugin("ipaddrs", FunctionType::Reporter(Box::new(Reporter)));
}
psistats::export_plugin!(register);
