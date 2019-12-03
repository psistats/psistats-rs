pub mod cpu;
pub mod mem;
pub mod ip_addrs;
pub mod os;
use std::{fmt, error};
use serde::{Deserialize, Serialize};
use toml::Value;
use toml::map::Map;


#[derive(Debug, Clone)]
pub struct ReporterError;

pub fn get_reporter(id: String) -> Option<ReporterCb> {
    match id.as_ref() {
        "cpu" => Some(cpu::cpu_reporter),
        "mem" => Some(mem::mem_reporter),
        "ip_addrs" => Some(ip_addrs::ip_addrs_reporter),
        "os" => Some(os::os_reporter),
        _ => None
    }
}

impl fmt::Display for ReporterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred with the reporter")
    }
}

impl error::Error for ReporterError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub type ReporterCb = fn(conf: &Map<String, Value>) -> Report;

#[derive(Deserialize, Serialize, Clone)]
pub struct Report {
    pub id: String,
    pub value: String
}

impl Report {
    fn new(id: String, value: String) -> Self {
        Report {
            id: id,
            value: value
        }
    }

    pub fn to_string(&self) -> String {
        return self.to_json();
    }

    pub fn to_json(&self) -> String {
        let json_value: String;
        if self.value.starts_with('{') == true || self.value.starts_with('[') == true {
            json_value = format!("{}", self.value);
        } else if self.value.parse::<u64>().is_ok() {
            json_value = format!("{}", self.value);
        } else {
            json_value = format!("\"{}\"", self.value);
        }

        let json = format!("{{\"id\": \"{id}\", \"value\": {val}}}", 
            id = self.id, 
            val = json_value
        );

        return json;
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_json())
    }
}
