use os_info;
use toml::map::Map;
use toml::Value;
use crate::reporters::Report;
use serde_json::json;
// use std::sync::Mutex;

lazy_static! {
    static ref OS_INFO: os_info::Info = {
        os_info::get()
    };
}

pub fn os_reporter(_: &Map<String, Value>) -> Report {
    let msg = vec![OS_INFO.os_type().to_string(), OS_INFO.version().to_string()];
    return Report::new("os".to_string(), json!(msg).to_string());
}
