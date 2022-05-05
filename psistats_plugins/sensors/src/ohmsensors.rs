use std::sync::Mutex;
use std::rc::Rc;
use std::thread;
use std::collections::HashMap;
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
    static ref REPORT_CHANNEL: (Sender<Vec<Sensor>>, Receiver<Vec<Sensor>>) = unbounded();    
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
                        
                        

                        REPORT_CHANNEL.0.send(results).unwrap();
                    }
                },
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    });
}

pub fn get_report(includes: &Vec<String>, mapping: &HashMap<String, String>) -> Result<ReportValue, PsistatsError> {

    let mut connected = CONNECTED.lock().unwrap();

    if *connected == false {
        start_ohm_thread();
        *connected = true;
    }

    // let results: Vec<Sensor> = wmi_con.filtered_query(&filters)?;

    WMI_CHANNEL.0.send(true).unwrap();

    match REPORT_CHANNEL.1.recv() {
        Ok(sensors) => {

            let mut reports: Vec<ReportValue> = vec![];

            for sensor in sensors {

                let mut sensor_name = sensor.Identifier;

                if mapping.contains_key(&sensor_name) {
                    sensor_name = mapping.get(&sensor_name).unwrap().to_string();
                }

                if includes.len() == 0 || includes.contains(&sensor_name) {
                    let report_value = ReportValue::Array([
                        ReportValue::String(sensor_name),
                        ReportValue::Float(sensor.Value)
                    ].to_vec());

                    reports.push(report_value);
                }
            }

            return Ok(ReportValue::Array(reports));
        }
        Err(err) => {
            return Err(PsistatsError::Runtime(format!("{}", err)));
        }
    }
}
