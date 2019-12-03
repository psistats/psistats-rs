use toml::Value;
use toml::map::Map;
use crate::reporters::Report;

pub fn publish(_conf: &Map<String, Value>, report: &Report) {
    info!("{}", report);
}
