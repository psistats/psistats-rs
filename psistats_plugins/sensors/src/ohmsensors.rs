use std::sync::Mutex;
use std::rc::Rc;
use std::thread;
use wmi::*;
use libpsistats::ReportValue;
use libpsistats::PsistatsError;
use serde::Deserialize;
use lazy_static::lazy_static;
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};

#[allow(non_snake_case)]
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Sensor {
    Name: String,
    Identifier: String,
    SensorType: String,
    Parent: String,
    Value: f64,
    Min: f64,
    Max: f64,
    Index: i64
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Hardware {
    Name: String,
    Identifier: String,
    HardwareType: String,
    Parent: String
}

lazy_static! {

    static ref CONNECTED: Mutex<bool> = Mutex::new(false);

    static ref WMI_CHANNEL: (Sender<bool>, Receiver<bool>) = unbounded();
    static ref REPORT_CHANNEL: (Sender<ReportValue>, Receiver<ReportValue>) = unbounded();    
}   

pub fn start_ohm_thread() {
    thread::spawn(move || {

        let comlib  = COMLibrary::new().unwrap();
        let wmi_con = WMIConnection::with_namespace_path("ROOT\\OpenHardwareMonitor", Rc::new(comlib)).unwrap();

        loop {
            match WMI_CHANNEL.1.recv() {
                Ok(v) => {
                    if v == true {
                        let results: Vec<Sensor> = wmi_con.query().unwrap();
                        
                        let report_values = results.iter().map(|r| {
                            return ReportValue::Array([
                                ReportValue::String(r.Identifier.clone()),
                                ReportValue::Float(r.Value)
                            ].to_vec());
                        }).collect();

                        REPORT_CHANNEL.0.send(ReportValue::Array(report_values)).unwrap();
                    }
                },
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    });
}

pub fn get_report() -> Result<ReportValue, PsistatsError> {

    let mut connected = CONNECTED.lock().unwrap();

    if *connected == false {
        start_ohm_thread();
        *connected = true;
    }

    // let results: Vec<Sensor> = wmi_con.filtered_query(&filters)?;

    WMI_CHANNEL.0.send(true).unwrap();

    match REPORT_CHANNEL.1.recv() {
        Ok(report) => {
            return Ok(report);
        }
        Err(err) => {
            return Err(PsistatsError::Runtime(format!("{}", err)));
        }
    }
}
