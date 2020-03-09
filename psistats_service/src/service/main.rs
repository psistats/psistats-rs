use crate::plugins::api::PluginRegistrar;
use crate::plugins::api::PsistatsFunctionTypes;
use crate::plugins::loader::PluginLoader;
use crate::plugins::registry::DefaultPluginRegistrar;
use clap::{App, Arg};
use std::alloc::System;
use std::thread;
use std::time::Duration;
use toml;
use std::fs::read_to_string;
use crate::config;
use std::collections::HashMap;

#[global_allocator]
static ALLOCATOR: System = System;

pub fn main() {
    let matches = App::new("Psistats")
        .version("0.2.0")
        .author("Psikon.Org")
        .about("Psistats system monitoring")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Location of config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("plugins")
                .long("plugins")
                .value_name("DIR")
                .help("One or many paths to plugins")
                .multiple(true)
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let c = matches.value_of("config").unwrap_or("psistats.toml");
    println!("Value for config is: {}", c);

    let plugins = matches.value_of("plugins").unwrap();
    println!("Plugins dir: {}", plugins);

    let mut registrar: Box<dyn PluginRegistrar> = Box::new(DefaultPluginRegistrar::new());

    let mut pl: PluginLoader = PluginLoader;
    unsafe {
        pl.load_plugins(plugins, &mut registrar).unwrap();
    }

    let conf: HashMap<String, HashMap<String, toml::Value>> = toml::from_str(&read_to_string(c).unwrap()).unwrap();
    let raw_settings = conf.get("settings").unwrap();
    let settings = config::Settings {
        hostname: raw_settings.get("hostname").unwrap().as_str().unwrap().to_string(),
        workers: raw_settings.get("workers").unwrap().as_integer().unwrap() as u16,
        timer: raw_settings.get("timer").unwrap().as_integer().unwrap() as u32
    };

    println!("Settings hostname: {}", settings.hostname);

}
