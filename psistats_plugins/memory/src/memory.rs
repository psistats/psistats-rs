use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use sysinfo::{SystemExt};
use sysinfo;
use libpsistats::ReportValue;
use std::thread;
use libpsistats::PsistatsError;

lazy_static! {
    static ref SYS_CHANNEL: (Sender<String>, Receiver<String>) = unbounded();
    static ref REPORT_CHANNEL: (Sender<ReportValue>, Receiver<ReportValue>) = unbounded();
}

pub fn start_mem_thread() {
    thread::spawn(|| {

        let mut sys = sysinfo::System::new();

        loop {
            match SYS_CHANNEL.1.recv() {
                Ok(_) => {
                    sys.refresh_memory();
                    let mut msg: Vec<ReportValue> = Vec::new();

                    msg.push(ReportValue::Integer(sys.get_total_memory()));
                    msg.push(ReportValue::Integer(sys.get_free_memory()));
                    msg.push(ReportValue::Integer(sys.get_available_memory()));
                    msg.push(ReportValue::Integer(sys.get_used_memory()));

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
                PsistatsError::Runtime("Memory plugin unable to get report".to_string())
            );
        }
    };
}
