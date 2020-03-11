pub mod plugins;
pub mod service;
pub mod config;

pub use config::{PublisherConfig, ReporterConfig, ServiceConfig, Settings};

pub use plugins::api::{
    PublisherInitFunction, PublisherFunction, ReporterInitFunction, ReporterFunction,
    PluginRegistrar, PsistatsReport,  PsistatsPlugin, PluginError
};

pub use plugins::registry::DefaultPluginRegistrar;

pub use plugins::loader::PluginLoader;
pub use plugins::api::FunctionType;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");
