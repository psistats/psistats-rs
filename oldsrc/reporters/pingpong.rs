use toml::Value;
use toml::map::Map;
use crate::reporters::Report;

pub fn pingpong_reporter(_: &Map<String, Value>) -> Report {

    return Report::new("pingpong".to_string(), "\"pong\"".to_string());
}
