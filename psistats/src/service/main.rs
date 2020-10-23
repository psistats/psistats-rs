use crate::PluginRegistrar;
use crate::PluginLoader;
use crate::DefaultPluginRegistrar;
use clap::{App, Arg};
use std::alloc::System;
use std::sync::{Arc, Mutex};
use crate::config;
use std::path::PathBuf;
use std::env;
use crate::service::manager::reporters::ManagerBuilder as RManagerBuilder;
use crate::service::manager::publishers::ManagerBuilder as PManagerBuilder;
use std::thread;
extern crate pretty_env_logger;
use crossbeam_channel::{ unbounded };
use std::time::Duration;

#[global_allocator]
static ALLOCATOR: System = System;

pub fn main(conf_file: &str, plugin_dir: &str) {
  let mut registrar: Box<dyn PluginRegistrar + Send> = Box::new(DefaultPluginRegistrar::new());

  let pl: PluginLoader = PluginLoader::new(plugin_dir.to_string());

  let conf = config::ServiceConfig::from_file(PathBuf::from(conf_file));

  env::set_var("RUST_LOG", conf.logging.get_level());
  pretty_env_logger::init();

  info!("Psistats now starting ...");

  unsafe {
    for pconf in conf.get_publisher_configs().iter().filter(|c| c.is_enabled()) {
      let plugin_name = pconf.get_name();
      match pl.load_plugin(plugin_name, &mut registrar) {
        Ok(()) => {
          info!("Plugin {} loaded", plugin_name);
        },
        Err(err) => {
          error!("Plugin {} failed to load: {}", plugin_name, err);
        }
      }

      match registrar.get_publisher_init(pconf.get_name()) {
        Ok(plugin) => {
          plugin.call(pconf).unwrap();
        },
        Err(err) => {
          error!("{}", err);
        }
      }
    }

    for rconf in conf.get_reporter_configs().iter().filter(|c| c.is_enabled()) {

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
          warn!("{}", err);
        }
      }
    }
  }

  let registrar_lock = Arc::new(Mutex::new(registrar));

  let (r_send, r_recv) = unbounded();
  let (c_send, c_recv) = unbounded();

  let confclone = conf.clone();
  let reg =  registrar_lock.clone();

  let r_send_clone = r_send.clone();
  let c_recv_clone = c_recv.clone();

  let reporter_thread = thread::spawn(move || {

    let mut builder = RManagerBuilder::new();
    builder.set_reporter_channel(r_send_clone)
      .set_command_channel(c_recv_clone)
      .set_registrar(&reg);

    let rm = builder.build(&confclone);

    let max_count = rm.max_count();
    let mut counter = 0;

    loop {
      debug!("Reporter iteration");
      counter = counter + 1;

      if counter > *max_count {
        counter =  1;
      }

      rm.run_reporters(&counter);
      thread::sleep(Duration::from_secs(1));
    }
  });

  let confclone2 = conf.clone();
  let reg2 =  registrar_lock.clone();

  let r_recv_clone2 = r_recv.clone();
  let c_send_clone2 = c_send.clone();

  let publisher_thread = thread::spawn(move || {
    let mut builder = PManagerBuilder::new();
    builder.set_reporter_channel(r_recv_clone2)
      .set_command_channel(c_send_clone2)
      .set_registrar(&reg2);

      let pm = builder.build(&confclone2);

      loop {
        debug!("Publisher iteration");

        pm.run_publishers();
        thread::sleep(Duration::from_secs(1));
      }
  });

  reporter_thread.join().unwrap();
  publisher_thread.join().unwrap();
}

pub fn cli_main() {
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

    main(c, plugins);
}
