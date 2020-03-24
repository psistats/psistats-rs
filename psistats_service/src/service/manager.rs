use std::thread;
use crate::ServiceConfig;
use crate::PluginRegistrar;
use crate::ReporterConfig;
use std::collections::HashMap;
use std::sync::{Mutex,Arc};
use std::time::Duration;

fn get_reporter_intervals(rconfs: &Vec<ReporterConfig>) -> (HashMap<u32, Vec<&str>>, Vec<u32>) {
  let mut reporters: HashMap<u32, Vec<&str>> = HashMap::new();

  for rconf in rconfs {

    if rconf.is_enabled() {

      let interval = rconf.get_interval();

      if !reporters.contains_key(&interval) {
        reporters.insert(interval, Vec::new());
      }

      reporters.get_mut(&interval).unwrap().push(rconf.get_name());
    }
  }


  let intervals: Vec<u32> = reporters.keys().cloned().filter(|i| {
    *i > 0
  }).map(|i| i.clone()).collect();

  return (reporters, intervals);
}

pub fn start_reporters(conf: &ServiceConfig, registrar: &Arc<Mutex<Box<dyn PluginRegistrar + Send>>>) -> thread::JoinHandle<()> {

  let registrar_clone = registrar.clone();
  let conf_clone = conf.clone();

  return thread::spawn(move || {
    let rconfs = conf_clone.get_reporter_configs();
    let (reporters, intervals) = get_reporter_intervals(&rconfs);

    let max_counter = intervals.iter().max().unwrap();
    let mut counter = 1;

    loop {
      counter = counter + 1;
      if counter > *max_counter {
        counter = 1;
      }

      for interval in &intervals {
        if counter % interval == 0 {
          let rlist = reporters.get(&interval).unwrap();
          for reporter_name in rlist {
            let reg = registrar_clone.lock().unwrap();
            let reporter = reg.get_reporter(reporter_name).unwrap();
            let rconf = conf_clone.get_reporter_config(reporter_name).unwrap();
            let report = reporter.call(rconf).unwrap();
            debug!("Report: {:?}", report);
          }
        }
      }

      thread::sleep(Duration::from_secs(1));
    }
  });
}