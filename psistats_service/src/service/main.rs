use clap::{Arg, App, SubCommand};
use crate::plugins::loader::{PluginLoader};


pub fn main() {

    let matches = App::new("Psistats")
        .version("0.2.0")
        .author("Psikon.Org")
        .about("Psistats system monitoring")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Location of config file")
            .takes_value(true))
        .arg(Arg::with_name("plugins")
            .long("plugins")
            .value_name("DIR")
            .help("One or many paths to plugins")
            .multiple(true)
            .takes_value(true))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("psistats.toml");
    println!("Value for config is: {}", config);

    let plugins = matches.value_of("plugins").unwrap();
    println!("Plugins dir: {}", plugins);

    let mut pl: PluginLoader = PluginLoader;
    unsafe {
        pl.loadPlugins(plugins).unwrap();
    }
}
