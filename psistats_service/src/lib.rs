use toml;

pub mod plugins;
pub mod service;
// use crate::plugins::registry;/

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

