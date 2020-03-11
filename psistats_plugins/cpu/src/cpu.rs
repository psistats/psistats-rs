use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use sysinfo::{ProcessorExt, SystemExt};
use sysinfo;
use psistats_service::plugins::api;
use std::thread;
use psistats_service::PsistatsReport;
use psistats_service::PluginError;

lazy_static! {
    static ref sys_channel: (Sender<String>, Receiver<String>) = unbounded();
    static ref report_channel: (Sender<PsistatsReport>, Receiver<PsistatsReport>) = unbounded();
}

pub fn start_cpu_thread() {
    thread::spawn(|| {

        let mut sys = sysinfo::System::new();

        loop {
            match sys_channel.1.recv() {
                Ok(_) => {
                    sys.refresh_cpu();
                    let procs = sys.get_processors();

                    let msg: Vec<api::ReportValue> = procs.iter().map(|p| {
                        return api::ReportValue::Float(p.get_cpu_usage().into());
                    }).collect();

                    let pr = api::PsistatsReport::new("cpu", api::ReportValue::Array(msg));
                    report_channel.0.send(pr).unwrap();
                },
                Err(_) => {
                    println!("sys_thread received error");
                }
            }
        }
    });    
}

pub fn get_report() -> Result<PsistatsReport, PluginError> {
    sys_channel.0.send("Foobar!".to_string()).unwrap();

    match report_channel.1.recv() {
        Ok(report) => {
            return Ok(report);
        },
        Err(_) => {
            return Err(
                PluginError::Runtime { p: "cpu".to_string(), msg: "Error getting report!".to_string() }
            );
        }
    };
}
