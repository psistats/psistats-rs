use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Plugin Settings
///
/// This will be passed to all registered plugin functions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PluginSettings {
  /// Plugin name
  pub name: String,

  enabled: bool,


  interval: Option<u64>,


  #[serde(default)]
  config: HashMap<String, toml::Value>
}

impl PluginSettings {
  /// Denotes whether or not the plugin is enabled. If plugin is disabled
  /// it will not be loaded.
  pub fn is_enabled(&self) -> bool {
    return self.enabled;
  }

  /// Typically used by reporter plugins. Sets the interval that a plugin
  /// will generate a report. If left unset or set to 0 but the plugin
  /// is still enabled, then the plugin will be loaded but generate no
  /// reports automatically. Useful for on demand reports.
  pub fn get_interval(&self) -> u64 {
    match self.interval {
      Some(val) => val,
      None => 0
    }
  }

  /// List of key/value pairs for arbitrary configuration values for a plugin
  pub fn get_config(&self) -> &HashMap<String, toml::Value> {
    return &self.config;
  }

  /// Get the plugin's name
  pub fn get_name(&self) -> &str {
    return &self.name;
  }
}