use libpsistats::PluginRegistrar;
use libpsistats::PluginLoader;
use libpsistats::DefaultPluginRegistrar;
use libpsistats::PluginSettings;
use clap::{App, Arg};
use std::alloc::System;
use std::sync::Arc;
use std::time::Duration;
use crate::service::config::PsistatsConfiguration;
use crate::service::workers::CommandWorker;
use crate::service::workers::PublisherWorker;
use crate::service::workers::ReporterWorker;
use crate::service::workers::ScheduleWorker;
use std::env;
// use std::thread;
extern crate pretty_env_logger;
use gethostname;
use crossbeam_utils::thread;
#[global_allocator]
static ALLOCATOR: System = System;


pub fn main(conf_file: String, plugin_dir: String) {

  let conf = Arc::new(load_configuration(&conf_file));
  let hostname = get_hostname(&conf);


  env::set_var("RUST_LOG", &conf.settings.loglevel);
  pretty_env_logger::init();

  info!("=[ PSISTATS ]=");
  info!("Hostname: {}", hostname);


  info!("Initializing plugins");
  let plugin_confs = &conf.plugin;
  let registrar = Arc::new(init_plugin_registrar(plugin_confs, &plugin_dir));


  let init_fns = registrar.get_init_fns();
  for (plugin_name, initfn) in init_fns.iter() {
    let plugin_conf = conf.get_plugin_config(plugin_name).unwrap();
    debug!("Calling init fn for {}", plugin_name);
    initfn.call(&hostname, plugin_conf).unwrap();
  }

  let (report_queue_tx, publisher_worker) = PublisherWorker::init(registrar.get_publisher_fns(), &conf);
  let (request_queue_tx, reporter_worker) = ReporterWorker::init(report_queue_tx.clone(), registrar.get_reporter_fns(), &conf);
  let (command_queue_tx, command_worker)  = CommandWorker::init(request_queue_tx.clone(), registrar.get_command_fns(), &conf);
  let schedule_worker = ScheduleWorker::init(request_queue_tx.clone(), registrar.get_reporter_fns(), &conf);


  thread::scope(|s| {
    debug!("Starting publisher worker");
    let publisher_handle = s.spawn(|_| {
      loop {
        trace!("publisher poll");
        publisher_worker.poll();
      }
    });


    debug!("Starting reporter worker");
    let reporter_handle = s.spawn(|_| {
      loop {
        trace!("reporter poll");
        reporter_worker.poll();
      }
    });

    debug!("Starting command worker");
    let command_handle = s.spawn(|_| {
      loop {
        trace!("command poll");
        command_worker.poll();
      }
    });

    debug!("Starting schedule worker");
    let schedule_handle = s.spawn(|_| {
      loop {
        trace!("schedule poll");
        schedule_worker.poll();
        std::thread::sleep(Duration::from_millis(1000));
      }
    });


    schedule_handle.join().unwrap();
    reporter_handle.join().unwrap();
    command_handle.join().unwrap();
    publisher_handle.join().unwrap();

  }).unwrap();



  // cmd_thread.join().unwrap();

  // cmd_thread.wait();

  /*

  loop {

    // Check if there are any commands pending in the command queue
    let cmd = cmd_rx.try_recv();
    match cmd {
      Ok(c) => {
        info!("{}", format!("Received command {:?}", c));
        match c {
          Commands::Report(r) => {
            let reporter_conf = conf.get_plugin_config(&r).unwrap().clone();
            let reporterfn = registrar.get_reporter_fn(&r).unwrap();
            let report_value = reporterfn.call(&reporter_conf).unwrap();

            let report = PsistatsReport::new(&r, &hostname, report_value);
            report_tx.send(report).unwrap();
          }
        }

      },
      _ => ()
    }

    // Iterate over reporter plugins for this interval
    for (interval, plugins) in &reporter_intervals {
      if interval > &0 && now.elapsed().as_secs() % interval == 0 {
        report_pool.scoped(|scoped| {

          for pluginname in plugins {
            let reporterfn = registrar.get_reporter_fn(pluginname).unwrap();
            let tx = report_tx.clone();
            let reporter_conf = conf.get_plugin_config(pluginname).unwrap().clone();
            let hostname = hostname.clone();


            scoped.execute(move || {
              let report_value = reporterfn.call(&reporter_conf).unwrap();
              let report = PsistatsReport::new(&pluginname, &hostname, report_value);
              match tx.send(report) {
                Ok(()) => (),
                Err(err) => error!("Failed to send msg: {:?}", err)
              }
            });
          }
        });
      }
    }


    loop {
      match report_rx.try_recv() {
        Ok(report) => {
          pub_pool.scoped(|scoped| {
            for (plugin, plugincb) in publisher_fns.iter() {
              let publisher_conf = conf.get_plugin_config(plugin).unwrap();
              let report = report.clone();
              scoped.execute(move || {
                plugincb.call(report, publisher_conf).unwrap();
              });
            }
          });
        }
        Err(_) => { break; }
      }
    }


    thread::sleep(Duration::from_millis(min_interval));
  }
  */
  loop {

    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("boop");
  }
}

fn get_hostname<'a>(conf: &'a PsistatsConfiguration) -> String {
  let hostname: String;

  if conf.settings.hostname == "" {
    hostname = gethostname::gethostname().into_string().unwrap().to_lowercase();
  } else {
    hostname = conf.settings.hostname.to_string();
  }

  return hostname;
}

fn load_configuration(conf_file: &str) -> PsistatsConfiguration {

  let conf: PsistatsConfiguration;

  let conf_content = std::fs::read_to_string(conf_file).unwrap();

  match toml::from_str::<PsistatsConfiguration>(&conf_content) {
    Ok(c) => conf = c,
    Err(e) => {
      println!("Failed to load the configuration file ({}):", conf_file);
      println!("{}", e);
      std::process::exit(1);
    }
  }

  return conf;

}

//fn init_plugin_registrar<'a>(conf: &'a PsistatsConfiguration, plugin_dir: &'a str) -> Box<dyn PluginRegistrar<'a> + Send + Sync> {
fn init_plugin_registrar<'a>(conf: &'a Vec<PluginSettings>, plugin_dir: &'a str) -> Box<dyn PluginRegistrar<'a> + Send + Sync> {
  let mut registrar: Box<dyn PluginRegistrar + Send + Sync> = Box::new(DefaultPluginRegistrar::new());
  let pl: PluginLoader = PluginLoader::new(plugin_dir.to_string());

  unsafe {
    for plugin_conf in conf.iter().filter(|c| c.is_enabled()) {
      let plugin_name = &plugin_conf.name;

      match pl.load_plugin(plugin_name, &mut registrar) {
        Ok(()) => {
          info!("Plugin {} loaded", plugin_name);
        },
        Err(err) => {
          error!("Failed to load plugin {}: {}", plugin_name, err);
        }
      }
    }
  }

  return registrar;

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


    let c = matches.value_of("config").unwrap_or("psistats.toml").to_string();
    let plugins = matches.value_of("plugins").unwrap().to_string();

    main(c, plugins);
}
