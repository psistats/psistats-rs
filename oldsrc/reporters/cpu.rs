use serde_json::json;
use sysinfo::{System, SystemExt, ProcessorExt};
use std::sync::Mutex;
use std::sync::Arc;
use toml::Value;
use toml::map::Map;


use crate::reporters::Report;

lazy_static! {

    static ref CPU_SYSTEM: Arc<Mutex<System>> = {
        let sys = System::new();
        Arc::new(Mutex::new(sys))
    };
}


pub fn cpu_reporter(_: &Map<String, Value>) -> Report {
    let mut sys = CPU_SYSTEM.lock().unwrap();
    sys.refresh_cpu();

    let procs = sys.get_processors();

    let msg: Vec<f32> = procs.iter().map(|p| {
        return p.get_cpu_usage();
    }).collect();


    return Report::new("cpu".to_string(), json!(msg).to_string());
}
