use crate::{ReporterFunction, InitFunction, PublisherFunction, CommandFunction };

use std::sync::Arc;
use std::collections::HashMap;

/// A plugin registrar is used to register plugins.
///
/// For most situations, [DefaultPluginRegistrar](libpsistats::DefaultPluginRegistrar) should
/// be a good enough implementation.
pub trait PluginRegistrar<'a> {

  /// Register an initialization function
  fn register_init_fn(&mut self, name: &str, func: Box<dyn InitFunction + Send + Sync>);

  /// Register a reporter funciton
  fn register_reporter_fn(&mut self, name: &str, func: Box<dyn ReporterFunction + Send + Sync>);

  /// Register a publisher function
  fn register_publisher_fn(&mut self, name: &str, func: Box<dyn PublisherFunction + Send + Sync>);

  /// Register a command function
  fn register_command_fn(&mut self, name: &str, func: Box<dyn CommandFunction + Send + Sync>);

  /// Register a plugin library. It's necessary to keep a reference
  /// of the plugin library active.
  fn register_lib(
      &mut self, lib: Arc<libloading::Library>
  );

  /// Get the plugin init function
  fn get_init_fn(&self, name: &str) -> Option<&Box<dyn InitFunction + Send + Sync>>;

  /// Get a reporter function
  fn get_reporter_fn(&self, name: &str) -> Option<&Box<dyn ReporterFunction + Send + Sync>>;

  /// Get a publisher function
  fn get_publisher_fn(&self, name: &str) -> Option<&Box<dyn PublisherFunction + Send + Sync>>;

  /// Get all initialization functions
  fn get_init_fns(&self) -> &HashMap<String, Box<dyn InitFunction + Send + Sync>>;

  /// Get all reporter functions
  fn get_reporter_fns(&self) -> &HashMap<String, Box<dyn ReporterFunction + Send + Sync>>;

  /// Get all publisher functions
  fn get_publisher_fns(&self) -> &HashMap<String, Box<dyn PublisherFunction + Send + Sync>>;

  /// Get all command functions
  fn get_command_fns(&self) -> &HashMap<String, Box<dyn CommandFunction + Send + Sync>>;
}