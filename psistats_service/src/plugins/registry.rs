use crate::PluginRegistrar;
use crate::{ ReporterFunction, ReporterInitFunction };
use crate::{ PublisherFunction, PublisherInitFunction };
use crate::PluginError;
use crate::FunctionType;

use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::rc::Rc;
use libloading::Library;


#[derive(Default)]
pub struct DefaultPluginRegistrar {
    
    reporter_init: HashMap<String, Box<dyn ReporterInitFunction>>,
    reporter: HashMap<String, Box<dyn ReporterFunction>>,
    publisher_init: HashMap<String, Box<dyn PublisherInitFunction>>,
    publisher: HashMap<String, Box<dyn PublisherFunction>>,

    libs: Vec<Rc<Library>>
}

impl DefaultPluginRegistrar {
    pub fn new() -> DefaultPluginRegistrar {
        DefaultPluginRegistrar::default()
    }
}

impl PluginRegistrar for DefaultPluginRegistrar {

    fn register_plugin(&mut self, name: &str, func: FunctionType) {
        match func {
            FunctionType::Publisher(f) => { self.publisher.insert(name.to_string(), f); },
            FunctionType::PublisherInit(f) => { self.publisher_init.insert(name.to_string(), f); },
            FunctionType::Reporter(f) => { self.reporter.insert(name.to_string(), f); },
            FunctionType::ReporterInit(f) => { self.reporter_init.insert(name.to_string(), f); }
        };
    }

    fn register_lib(&mut self, lib: Rc<libloading::Library>) {
        self.libs.push(lib);
    }

    fn get_reporter_init(&self, name: &str) -> Result<&Box<dyn ReporterInitFunction>, PluginError> {
        if self.reporter_init.contains_key(name) {
            Ok(self.reporter_init.get(name).unwrap())
        } else {
            Err(PluginError::FunctionNotFound { p: name.to_string(), fname: "Reporter Init Function".to_string() })
        }
    }

    fn get_reporter(&self, name: &str) -> Result<&Box<dyn ReporterFunction>, PluginError> {
        if self.reporter.contains_key(name) {
            Ok(self.reporter.get(name).unwrap())
        } else {
            Err(PluginError::FunctionNotFound { p: name.to_string(), fname: "Reporter Function".to_string() })
        }
    }

    fn get_publisher_init(&self, name: &str) -> Result<&Box<dyn PublisherInitFunction>, PluginError> {
        if self.publisher_init.contains_key(name) {
            Ok(self.publisher_init.get(name).unwrap())
        } else {
            Err(PluginError::FunctionNotFound { p: name.to_string(), fname: "Publisher init function".to_string() })
        }
    }

    fn get_publisher(&self, name: &str) -> Result<&Box<dyn PublisherFunction>, PluginError> {
        if self.publisher.contains_key(name) {
            Ok(self.publisher.get(name).unwrap())
        } else {
            Err(PluginError::FunctionNotFound { p: name.to_string(), fname: "Publisher function".to_string() })
        }
    }
}
