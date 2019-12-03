use serde_json::json;
use sysinfo::{System, SystemExt};
use std::sync::Mutex;
use toml::Value;
use toml::map::Map;


use crate::reporters::Report;

lazy_static! {
    static ref MEM_SYSTEM: Mutex<System> = {
        let sys = System::new();
        Mutex::new(sys)
    };
}

pub fn mem_reporter(_: &Map<String, Value>) -> Report {

    let mut sys = MEM_SYSTEM.lock().unwrap();
    sys.refresh_system();

    let msg = vec![sys.get_total_memory(), sys.get_free_memory()];

    return Report::new("mem".to_string(), json!(msg).to_string());
}


