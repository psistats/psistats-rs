#[macro_use]
use psistats_service::plugins::api::{PluginRegistrar, PsistatsFunction};

use toml;

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_init_fn("cpu", Box::new(Init));
}
psistats_service::export_plugin!(register);


#[derive(Debug, Clone, PartialEq)]
struct Init;

impl PsistatsFunction for Init {
    fn call(&self, _: toml::Value) {
        println!("CPU Plugin Init Function Called!");
    }
}
