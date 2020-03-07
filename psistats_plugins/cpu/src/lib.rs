#[macro_use]
use psistats_service::plugins::api::{PluginRegistrar, PsistatsFunction, PsistatsFunctionTypes};

use toml;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar>) {
    println!("psistats-cpu: register() called");
    registrar.register_fn("cpu", PsistatsFunctionTypes::INIT, Box::new(Init));
    registrar.register_fn("cpu", PsistatsFunctionTypes::REPORT, Box::new(Report));
}
psistats_service::export_plugin!(register);

#[derive(Debug, Clone, PartialEq)]
struct Init;

impl PsistatsFunction for Init {
    fn call(&self, _: toml::Value) {
        println!("CPU Plugin Init Function Called!");
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Report;

impl PsistatsFunction for Report {
    fn call(&self, _: toml::Value) {
        println!("CPU Plugin report function called!");
    }
}
