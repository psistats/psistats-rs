use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use sysinfo::{ProcessorExt, SystemExt};
use sysinfo;
use psistats::plugins::api;
use std::thread;
use psistats::PsistatsReport;
use psistats::PluginError;

lazy_static! {
    static ref SYS_CHANNEL: (Sender<String>, Receiver<String>) = unbounded();
    static ref REPORT_CHANNEL: (Sender<PsistatsReport>, Receiver<PsistatsReport>) = unbounded();
}

pub fn start_mem_thread() {
    thread::spawn(|| {

        let mut sys = sysinfo::System::new();

        loop {
            match SYS_CHANNEL.1.recv() {
                Ok(_) => {
                    sys.refresh_memory();
                    let mut msg: Vec<api::ReportValue> = Vec::new();
                    msg.push(api::ReportValue::Integer(sys.get_total_memory()));
                    msg.push(api::ReportValue::Integer(sys.get_free_memory()));

                    let pr = api::PsistatsReport::new("memory", api::ReportValue::Array(msg));
                    REPORT_CHANNEL.0.send(pr).unwrap();
                },
                Err(_) => ()
            }
        }
    });
}

pub fn get_report() -> Result<PsistatsReport, PluginError> {
    SYS_CHANNEL.0.send("Foobar!".to_string()).unwrap();

    match REPORT_CHANNEL.1.recv() {
        Ok(report) => {
            return Ok(report);
        },
        Err(_) => {
            return Err(
                PluginError::Runtime { p: "memory".to_string(), msg: "Error getting report!".to_string() }
            );
        }
    };
}
