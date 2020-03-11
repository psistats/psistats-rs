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


