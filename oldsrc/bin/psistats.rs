#![allow(dead_code)]
#![allow(unused_variables)]

use crossbeam_channel as channel;
// use psistats::config::Config;
use psistats::manager::{start_reporters, start_publishers};
use psistats::reporters::Report;
use std::fs;
use std::env;
use pretty_env_logger;
use toml;
use clap::{Arg, App};
#[macro_use] extern crate log;

fn main() {

    let matches = App::new("Psistats")
                    .version("0.1.0-beta")
                    .author("Alex Dow <adow@psikon.com>")
                    .about("System metrics reporting tool")
                    .arg(Arg::with_name("conf")
                      .short("c")
                      .long("config")
                      .value_name("CONF")
                      .help("Set location of configuration file")
                      .takes_value(true)
                    )
                    .arg(Arg::with_name("verbose")
                      .long("verbose")
                      .help("Toggle verbose output")
                    )
                    .get_matches();

    let mut loglevel = "info";
    if matches.is_present("verbose") == true {

      loglevel = "debug";
    }

    let conffile = matches.value_of("conf").unwrap_or("psistats.toml");


    env::set_var("RUST_LOG", loglevel);
    pretty_env_logger::init();

    info!("Starting psistats with config: {}", conffile);

    let conf_contents = fs::read_to_string(conffile)
        .expect(format!("Error reading {}", conffile).as_ref());

    let config = conf_contents.parse::<toml::Value>().unwrap();


    let (reports_sender, reports_receiver) = channel::unbounded::<Report>();
    let (commands_sender, commands_receiver) = channel::unbounded::<String>();

    let reporter_proc = start_reporters(config.clone(), reports_sender.clone(), commands_receiver.clone());
    let publisher_proc = start_publishers(config.clone(), reports_receiver.clone(), commands_sender.clone());

    let _ = reporter_proc.join();
    let _ = publisher_proc.join();
}
