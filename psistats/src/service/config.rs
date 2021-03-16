use libpsistats::PluginSettings;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PsistatsConfiguration {
  pub settings: Settings,
  pub advanced: Advanced,


  #[serde(default)]
  pub plugin: Vec<PluginSettings>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
  pub hostname: String,
  pub loglevel: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Advanced {
  pub min_interval: u64,
  pub r_workers: u32,
  pub p_workers: u32
}

impl PsistatsConfiguration {

  pub fn get_plugin_config(&self, plugin_name: &str) -> Option<&PluginSettings> {
    return self.plugin.iter().find(|p| p.name == plugin_name);
  }
}
