use serde_json::json;
use get_if_addrs::get_if_addrs;
use toml::Value;
use toml::map::Map;
use crate::reporters::Report;


pub fn ip_addrs_reporter(_: &Map<String, Value>) -> Report {

    let mut ips = vec![];

    for iface in get_if_addrs().unwrap() {
        let ip = (iface.name, iface.addr.ip());
        ips.push(ip);
    }

    return Report::new("ip_addrs".to_string(), json!(ips).to_string());
}
