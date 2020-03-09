use crate::plugins::api::PluginRegistrar;
use crate::plugins::api::PsistatsFunction;
use crate::plugins::api::{PsistatsReportFunction, PsistatsInitFunction};
use std::collections::HashMap;
use std::rc::Rc;
use libloading::Library;

#[derive(Default)]
pub struct DefaultPluginRegistrar {
    init_fn: HashMap<String, Box<dyn PsistatsInitFunction>>,
    report_fn: HashMap<String, Box<dyn PsistatsReportFunction>>,
    publish_fn: HashMap<String, Box<dyn PsistatsFunction>>,
    libs: Vec<Rc<Library>>
}

impl DefaultPluginRegistrar {
    pub fn new() -> DefaultPluginRegistrar {
        DefaultPluginRegistrar::default()
    }
}

impl PluginRegistrar for DefaultPluginRegistrar {

    fn register_init_fn(&mut self, name: &str, func: Box<dyn PsistatsInitFunction>) {
        self.init_fn.insert(name.to_string(), func);
    }

    fn register_report_fn(&mut self, name: &str, func: Box<dyn PsistatsReportFunction>) {
        self.report_fn.insert(name.to_string(), func);
    }

    fn register_lib(&mut self, lib: Rc<libloading::Library>) {
        self.libs.push(lib);
    }

    fn get_init_fn(&self, name: &str) -> Result<&Box<dyn PsistatsInitFunction>, String> {
        if self.init_fn.contains_key(name) {
            Ok(self.init_fn.get(name).unwrap())
        } else {
            Err(format!("Init function '{}' not found", name))
        }
    }

    fn get_report_fn(&self, name: &str) -> Result<&Box<dyn PsistatsReportFunction>, String> {
        if self.report_fn.contains_key(name) {
            Ok(self.report_fn.get(name).unwrap())
        } else {
            Err(format!("Report function '{}' not found", name))
        }
    }    
}
