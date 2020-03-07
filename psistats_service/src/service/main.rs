use crate::plugins::api::PluginRegistrar;
use crate::plugins::api::PsistatsFunctionTypes;
use crate::plugins::loader::PluginLoader;
use crate::plugins::registry::DefaultPluginRegistrar;
use clap::{App, Arg};
use std::alloc::System;

#[global_allocator]
static ALLOCATOR: System = System;

pub fn main() {
    let matches = App::new("Psistats")
        .version("0.2.0")
        .author("Psikon.Org")
        .about("Psistats system monitoring")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Location of config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("plugins")
                .long("plugins")
                .value_name("DIR")
                .help("One or many paths to plugins")
                .multiple(true)
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap_or("psistats.toml");
    println!("Value for config is: {}", config);

    let plugins = matches.value_of("plugins").unwrap();
    println!("Plugins dir: {}", plugins);

    let mut registrar: Box<dyn PluginRegistrar> = Box::new(DefaultPluginRegistrar::new());

    let mut pl: PluginLoader = PluginLoader;
    unsafe {
        pl.load_plugins(plugins, &mut registrar).unwrap();
    }

    println!("Total init callbacks: {}", registrar.count_fn(PsistatsFunctionTypes::INIT));
    println!("Total report callbacks: {}", registrar.count_fn(PsistatsFunctionTypes::REPORT));
    println!("Total publish callbacks: {}", registrar.count_fn(PsistatsFunctionTypes::PUBLISH));
    println!("Total libraries: {}", registrar.count_libs());
}
