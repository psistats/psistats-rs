use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use toml::Value;
use toml;
use std::path::{Path, PathBuf};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub hostname: String,
    pub workers: u16,
    pub timer: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReporterConfig {
    name: String,
    enabled: bool,
    interval: u32,

    #[serde(default)]
    config: HashMap<String, toml::Value>
}

impl ReporterConfig {
    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn is_enabled(&self) -> bool {
        return self.enabled;
    }
    
    pub fn get_config(&self) -> &HashMap<String, toml::Value> {
        return &self.config;
    }
    
    pub fn get_interval(&self) -> u32 {
        return self.interval;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublisherConfig {
    name: String,
    enabled: bool,
    
    #[serde(default)]
    config: HashMap<String, toml::Value>
}

impl PublisherConfig {
    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn is_enabled(&self) -> bool {
        return self.enabled;
    }
    
    pub fn get_config(&self) -> &HashMap<String, toml::Value> {
        return &self.config;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceConfig {
    pub settings: Settings,

    #[serde(default)]
    pub r_plugin: Vec<ReporterConfig>,

    #[serde(default)]
    pub p_plugin: Vec<PublisherConfig>
}

impl ServiceConfig {
    pub fn get_publisher_config(&self, name: String) -> Option<&PublisherConfig> {
        let conf: Vec<&PublisherConfig> = self.p_plugin.iter().filter(|pc| pc.name == name).collect();
        if conf.len() > 0 {
            Some(conf[0])
        } else {
            None
        }
    }

    pub fn get_reporter_config(&self, name: String) -> Option<&ReporterConfig> {
        let conf: Vec<&ReporterConfig> = self.r_plugin.iter().filter(|pc| pc.name == name).collect();
        if conf.len() > 0 {
            Some(conf[0])
        } else {
            None
        }
    }

    pub fn get_reporter_configs(&self) -> &Vec<ReporterConfig> { 
        return &self.r_plugin;
    }

    pub fn get_publisher_configs(&self) -> &Vec<PublisherConfig> {
        return &self.p_plugin;
    }

    pub fn from_file(p: PathBuf) -> Self {
        let confstr = &read_to_string(&p).unwrap();
        let conf: ServiceConfig = toml::from_str(confstr).unwrap();
        conf
    }
}
