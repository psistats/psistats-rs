use crate::PluginRegistrar;
use crate::PluginLoader;
use crate::DefaultPluginRegistrar;
use clap::{App, Arg};
use std::alloc::System;
use crate::config;
use std::path::PathBuf;

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

    let pl: PluginLoader = PluginLoader::new(plugins.to_string());

    let conf = config::ServiceConfig::from_file(PathBuf::from(c));

    unsafe {
        for rconf in conf.get_reporter_configs() {

            let plugin_name = rconf.get_name();
            match pl.load_plugin(plugin_name, &mut registrar) {
                Ok(()) => {
                    println!("Plugin {} loaded", plugin_name);
                },
                Err(err) => {
                    println!("Plugin failed to load: {}", err);
                }
            }

            match registrar.get_reporter_init(rconf.get_name()) {
              Ok(plugin) => {
                plugin.call(rconf).unwrap();
              },
              Err(err) => {
                println!("{}", err);
              }
            }
        }
    }
}
