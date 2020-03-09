use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub hostname: String,
    pub workers: u16,
    pub timer: u32
}


#[derive(Serialize, Deserialize)]
pub struct PluginConfig {
    enabled: bool,
    interval: u32,
    config: HashMap<String, String>
}
