use libpsistats::PluginRegistrar;
use libpsistats::PluginLoader;
use libpsistats::DefaultPluginRegistrar;
use libpsistats::PsistatsReport;
use clap::{App, Arg};
use std::alloc::System;
use crate::service::config::PsistatsConfiguration;
use std::env;
use std::thread;
extern crate pretty_env_logger;
use std::sync::mpsc;
use scoped_threadpool::Pool;
use gethostname;
use std::time::{Duration, Instant};
use std::collections::HashMap;

#[global_allocator]
static ALLOCATOR: System = System;

pub fn main(conf_file: &str, plugin_dir: &str) {
  let mut registrar: Box<dyn PluginRegistrar + Send + Sync> = Box::new(DefaultPluginRegistrar::new());

  let pl: PluginLoader = PluginLoader::new(plugin_dir.to_string());
  let conf: PsistatsConfiguration;

  let conf_content = std::fs::read_to_string(conf_file).unwrap();

  match toml::from_str::<PsistatsConfiguration>(&conf_content) {
    Ok(c) => conf = c,
    Err(e) => {
      println!("{}", e);
      std::process::exit(1);
    }
  }

  let hostname: String;

  if conf.settings.hostname == "" {
    hostname = gethostname::gethostname().into_string().unwrap().to_lowercase();
  } else {
    hostname = conf.settings.hostname.to_string();
  }

  env::set_var("RUST_LOG", &conf.settings.loglevel);
  pretty_env_logger::init();

  info!("=[ PSISTATS ]=");
  info!("Hostname: {}", hostname);

  unsafe {
    for plugin_conf in conf.plugin.iter().filter(|c| c.is_enabled()) {
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

  let init_fns = registrar.get_init_fns();
  for (plugin_name, initfn) in init_fns.iter() {
    let plugin_conf = conf.get_plugin_config(plugin_name).unwrap();
    debug!("Calling init fn for {}", plugin_name);
    initfn.call(&conf.settings.hostname, plugin_conf).unwrap();
  }

  let reporter_fns = registrar.get_reporter_fns();
  let publisher_fns = registrar.get_publisher_fns();

  let mut reporter_intervals: HashMap<u64, Vec<&str>> = HashMap::new();
  let now = Instant::now();

  let max_counter = reporter_intervals.keys().max();


  for (pluginname, _) in reporter_fns {
    let pconf = conf.get_plugin_config(pluginname).unwrap();
    let interval = pconf.get_interval();

    if !reporter_intervals.contains_key(&interval) {
      reporter_intervals.insert(interval, vec![pluginname]);
    } else {
      reporter_intervals.get_mut(&interval).unwrap().push(pluginname);
    }
  }

  let (tx, rx) = mpsc::channel();

  let r_workers = conf.advanced.r_workers;
  let p_workers = conf.advanced.p_workers;
  let min_interval = conf.advanced.min_interval;

  let mut report_pool = Pool::new(r_workers);
  let mut pub_pool = Pool::new(p_workers);

  loop {
    for (interval, plugins) in &reporter_intervals {
      if now.elapsed().as_secs() % interval == 0 {
        report_pool.scoped(|scoped| {

          for pluginname in plugins {
            let reporterfn = registrar.get_reporter_fn(pluginname).unwrap();
            let tx = tx.clone();
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


      loop {
        match rx.try_recv() {
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
    }

    thread::sleep(Duration::from_millis(min_interval));
  }

  /*
  loop {
    for (plugin, reporterfn) in *reporter_fns {
      let tx = tx.clone();
      let reporter_conf = conf.get_plugin_config(plugin).unwrap();
      pool.execute(move || {
        let report_value = reporterfn.call(reporter_conf).unwrap();
        let report = PsistatsReport::new(&hostname, report_value);
        tx.send(report).unwrap();
      });
    }
    thread::sleep_ms(1000);
  }
  */

  /*
  let r = Arc::new(registrar);

  // let publisher_plugins = r.clone().get_publisher_fns();
  let reporter_plugins  = r.get_reporter_fns();

  for (plugin_name, initfn) in registrar.get_init_fns() {
    let plugin_conf = conf.get_plugin_config(plugin_name).unwrap();
    initfn.call(conf.hostname(), plugin_conf).unwrap();
  }

  //let r = Arc::new(registrar);



  let n_workers = 4;
  let pool = ThreadPool::new(n_workers);

  let (tx, rx) = unbounded();

  loop {
    for (plugin, reporterfn) in reporter_plugins {
      let tx = tx.clone();
      let reporter_conf = conf.get_plugin_config(plugin).unwrap();
      pool.execute(move || {
        let report_value = reporterfn.call(reporter_conf).unwrap();
        let report = PsistatsReport::new(conf.hostname(), report_value);
        tx.send(report).unwrap();
      });
    }
    thread::sleep_ms(1000);
  }

    /*
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
          plugin.call(pconf, &conf.settings).unwrap();
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
          error!("{}", err);
        }
      }
    }



  /*
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
  */
*/
*/
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
