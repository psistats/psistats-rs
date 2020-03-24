use crate::PluginRegistrar;
use crate::PluginLoader;
use crate::DefaultPluginRegistrar;
use clap::{App, Arg};
use std::alloc::System;
use std::sync::{Arc, Mutex};
use crate::config;
use std::path::PathBuf;
use std::env;
use crate::service::manager::start_reporters;
extern crate pretty_env_logger;

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
    let plugins = matches.value_of("plugins").unwrap();

    let mut registrar: Box<dyn PluginRegistrar + Send> = Box::new(DefaultPluginRegistrar::new());

    let pl: PluginLoader = PluginLoader::new(plugins.to_string());

    let conf = config::ServiceConfig::from_file(PathBuf::from(c));

    env::set_var("RUST_LOG", conf.logging.get_level());
    pretty_env_logger::init();

    info!("Psistats now starting ...");

    unsafe {
      for rconf in conf.get_reporter_configs() {

        let plugin_name = rconf.get_name();
        match pl.load_plugin(plugin_name, &mut registrar) {
            Ok(()) => {
                info!("Plugin {} loaded", plugin_name);
            },
            Err(err) => {
                error!("Plugin {} failed to load: {}", plugin_name, err);
            }
        }

        debug!("Trying to intialize {}", plugin_name);
        match registrar.get_reporter_init(rconf.get_name()) {
          Ok(plugin) => {
            plugin.call(rconf).unwrap();
          },
          Err(err) => {
            error!("{}", err);
          }
        }
      }
    }

    let registrarLock = Arc::new(Mutex::new(registrar));

    let reporter_thread = start_reporters(&conf, &registrarLock);

    reporter_thread.join().unwrap();
}
