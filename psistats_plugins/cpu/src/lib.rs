#[macro_use]
use psistats_service::plugins::api::{PluginRegistrar, PsistatsFunction, PsistatsFunctionTypes, Report};
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessorExt, SystemExt};
use sysinfo;
use serde_json::{json};


use toml;

extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar>) {
    println!("psistats-cpu: register() called");
    registrar.register_fn("cpu", PsistatsFunctionTypes::INIT, Box::new(Init));
    registrar.register_fn("cpu", PsistatsFunctionTypes::REPORT, Box::new(Reporter));
}
psistats_service::export_plugin!(register);

lazy_static! {
    static ref sys_channel: (Sender<String>, Receiver<String>) = unbounded();
    static ref report_channel: (Sender<Report>, Receiver<Report>) = unbounded();
}

#[derive(Debug, Clone, PartialEq)]
struct Init;

impl PsistatsFunction for Init {
    fn call(&self, _: toml::Value) {
        println!("CPU Plugin Init Function Called!");
        thread::spawn(|| {

            let mut sys = sysinfo::System::new();

            loop {
                match sys_channel.1.recv() {
                    Ok(_) => {
                        sys.refresh_cpu();
                        let procs = sys.get_processors();

                        let msg: Vec<f32> = procs.iter().map(|p| {
                            return p.get_cpu_usage();
                        }).collect();
                    
                    
                        let r = Report::new("cpu".to_string(), json!(msg).to_string());
                        report_channel.0.send(r);
                        
                    },
                    Err(_) => {
                        println!("sys_thread received error");
                    }
                }
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Reporter;

impl PsistatsFunction for Reporter {
    fn call(&self, _: toml::Value) {
        println!("CPU Plugin report function called!");
        sys_channel.0.send("Foobar!".to_string()).unwrap();

        match report_channel.1.recv() {
            Ok(report) => {
                println!("Report received: {}", report.to_json());
            },
            Err(_) => {
                println!("Error getting report");
            }
        };
    }
}
