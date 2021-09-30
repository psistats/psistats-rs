use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use sysinfo::{ProcessorExt, SystemExt};
use sysinfo;
use std::thread;
use libpsistats::{ ReportValue, PsistatsError };
lazy_static! {
    static ref SYS_CHANNEL: (Sender<String>, Receiver<String>) = unbounded();
    static ref REPORT_CHANNEL: (Sender<ReportValue>, Receiver<ReportValue>) = unbounded();
}

pub fn start_cpu_thread(combined: bool) {
    thread::spawn(move || {

        let mut sys = sysinfo::System::new();

        loop {
            match SYS_CHANNEL.1.recv() {
                Ok(_) => {
                    sys.refresh_cpu();
                    let procs = sys.get_processors();
                    let pr: ReportValue;

                    let cpu_cores: Vec<ReportValue> = procs.iter().map(|p| {
                        return ReportValue::Float(p.get_cpu_usage().into());
                    }).collect();

                    if combined {
                      let cpu_sum: f64 = cpu_cores.iter().map(|x| {
                        if let ReportValue::Float(cpu_usage) = x {
                          return cpu_usage;
                        } else {
                          return &0.0;
                        }
                      }).sum();

                      let total_cpus = cpu_cores.len();

                      let cpu_total: f64 = cpu_sum / total_cpus as f64;

                      pr = ReportValue::Float(cpu_total);
                    } else {
                      pr = ReportValue::Array(cpu_cores);
                    }


                    REPORT_CHANNEL.0.send(pr).unwrap();
                },
                Err(_) => ()
            }
        }
    });
}

pub fn get_report() -> Result<ReportValue, PsistatsError> {
  // FIXME What is this again?
    SYS_CHANNEL.0.send("Foobar!".to_string()).unwrap();

    match REPORT_CHANNEL.1.recv() {
        Ok(report) => {
            return Ok(report);
        },
        Err(err) => {
            return Err(
              PsistatsError::Runtime(format!("{}", err))
            );
        }
    };
}
