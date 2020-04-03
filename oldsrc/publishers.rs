// pub mod log;
pub mod mqtt;
pub mod log;
use crate::reporters::Report;
use toml::map::Map;
use toml::Value;

pub type Publisher = fn(conf: &Map<String, Value>, report: &Report);
pub type Commander = fn(conf: &Map<String, Value>) -> Option<String>;

pub fn get_commander(pname: String) -> Option<Commander> {
    match pname.as_ref() {
        "mqtt" => Some(mqtt::commander),
        _ => None
    }
}

pub fn get_publisher(pname: String) -> Option<Publisher> {
    match pname.as_ref() {
        "mqtt" => Some(mqtt::publish),
        "log" => Some(log::publish),
        _ => None
    }
}
