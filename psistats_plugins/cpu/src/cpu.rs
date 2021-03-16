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

pub fn start_cpu_thread() {
    thread::spawn(|| {

        let mut sys = sysinfo::System::new();

        loop {
            match SYS_CHANNEL.1.recv() {
                Ok(_) => {
                    sys.refresh_cpu();
                    let procs = sys.get_processors();

                    let msg: Vec<ReportValue> = procs.iter().map(|p| {
                        return ReportValue::Float(p.get_cpu_usage().into());
                    }).collect();

                    let pr = ReportValue::Array(msg);
                    REPORT_CHANNEL.0.send(pr).unwrap();
                },
                Err(_) => ()
            }
        }
    });
}

pub fn get_report() -> Result<ReportValue, PsistatsError> {
    SYS_CHANNEL.0.send("Foobar!".to_string()).unwrap();

    match REPORT_CHANNEL.1.recv() {
        Ok(report) => {
            return Ok(report);
        },
        Err(_) => {
            return Err(
              PsistatsError::Runtime("foo".to_string())
            );
        }
    };
}
