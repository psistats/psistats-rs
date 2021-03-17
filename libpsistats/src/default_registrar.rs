use crate::PluginRegistrar;
use crate::{PublisherFunction, ReporterFunction, InitFunction};


use libloading::Library;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct DefaultPluginRegistrar {
  init_fns: HashMap<String, Box<dyn InitFunction + Send + Sync>>,
  reporter_fns: HashMap<String, Box<dyn ReporterFunction + Send + Sync>>,
  publisher_fns: HashMap<String, Box<dyn PublisherFunction + Send + Sync>>,

  libs: Vec<Arc<Library>>,
}

impl<'a> DefaultPluginRegistrar {
  pub fn new() -> DefaultPluginRegistrar {
    DefaultPluginRegistrar::default()
  }


}

impl<'a> PluginRegistrar<'a> for DefaultPluginRegistrar {
  fn register_init_fn(&mut self, name: &str, func: Box<dyn InitFunction + Send + Sync>) {
    self.init_fns.insert(name.to_string(), func);
  }

  fn register_reporter_fn(&mut self, name: &str, func: Box<dyn ReporterFunction + Send + Sync>) {
    self.reporter_fns.insert(name.to_string(), func);
  }

  fn register_publisher_fn(&mut self, name: &str, func: Box<dyn PublisherFunction + Send + Sync>) {
    self.publisher_fns.insert(name.to_string(), func);
  }

  fn register_lib(&mut self, lib: Arc<libloading::Library>) {
    self.libs.push(lib);
  }

  fn get_init_fn(&self, name: &str) -> Option<&Box<dyn InitFunction + Send + Sync>> {
    if self.init_fns.contains_key(name) {
      Some(self.init_fns.get(name).unwrap())
    } else {
      None
    }
  }

  fn get_reporter_fn(&self, name: &str) -> Option<&Box<dyn ReporterFunction + Send + Sync>> {
    if self.reporter_fns.contains_key(name) {
      Some(self.reporter_fns.get(name).unwrap())
    } else {
      None
    }
  }

  fn get_publisher_fn(&self, name: &str) -> Option<&Box<dyn PublisherFunction + Send + Sync>> {
    if self.publisher_fns.contains_key(name) {
      Some(self.publisher_fns.get(name).unwrap())
    } else {
      None
    }
  }

  fn get_init_fns(&self) -> &HashMap<String, Box<dyn InitFunction + Send + Sync>> {
    return &self.init_fns;
  }

  fn get_reporter_fns(&self) -> &HashMap<String, Box<dyn ReporterFunction + Send + Sync>> {
    return &self.reporter_fns;
  }

  fn get_publisher_fns(&self) -> &HashMap<String, Box<dyn PublisherFunction + Send + Sync>> {
    return &self.publisher_fns;
  }
}
