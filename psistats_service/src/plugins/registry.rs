use crate::plugins::api::PsistatsFunction;
use crate::plugins::api::PluginRegistrar;
use std::collections::HashMap;


#[derive(Default)]
struct DefaultPluginRegistrar {
    init_fn: HashMap<String, Box<dyn PsistatsFunction>>,
    report_fn: HashMap<String, Box<dyn PsistatsFunction>>,
    publish_fn: HashMap<String, Box<dyn PsistatsFunction>>
}

impl DefaultPluginRegistrar {
    pub fn new() -> DefaultPluginRegistrar {
        DefaultPluginRegistrar::default()
    }
}

impl PluginRegistrar for DefaultPluginRegistrar {
    fn register_init_fn(&mut self, name: &str, cb: Box<dyn PsistatsFunction>) {
        self.init_fn.insert(name.to_string(), cb);
    }

    fn register_publish_fn(&mut self, name: &str, cb: Box<dyn PsistatsFunction>) {
        self.publish_fn.insert(name.to_string(), cb);
    }

    fn register_report_fn(&mut self, name: &str, cb: Box<dyn PsistatsFunction>) {
        self.report_fn.insert(name.to_string(), cb);
    }
}

