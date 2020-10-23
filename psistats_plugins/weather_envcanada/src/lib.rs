#[macro_use]
use psistats::{ ReporterFunction, ReporterInitFunction, ReporterConfig };
use psistats::PluginRegistrar;
use psistats::PsistatsReport;
use psistats::PluginError;
use psistats::FunctionType;
use psistats::ReportValue;

mod envcanada;


extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send>) {
    registrar.register_plugin("weather_envcanada", FunctionType::Reporter(Box::new(Reporter)));
}
psistats::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl ReporterFunction for Reporter {
    fn call(&self, conf: &ReporterConfig) -> Result<PsistatsReport, PluginError> {
        // let t = conf.get_config().get("show_total").unwrap().as_bool().unwrap();
        //
        let province = conf.get_config().get("province").unwrap().as_str().unwrap();
        let cityCode = conf.get_config().get("cityCode").unwrap().as_str().unwrap();
        let xml = envcanada::get_data(province, cityCode).unwrap();
        let conditions = envcanada::parse(&xml).unwrap();

        let r = PsistatsReport::new("weather", conditions.to_report_value());
        Ok(r)
    }
}
