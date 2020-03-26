use std::sync::Arc;
use libloading;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::collections::HashMap;
use crate::{ ReporterConfig, PublisherConfig };

/// List of plugin function types
pub enum FunctionType {
    /// A publisher function is called every time a report has been
    /// generated
    Publisher(Box<dyn PublisherFunction + Send>),
    /// A publisher init function is called when psistats service starts
    PublisherInit(Box<dyn PublisherInitFunction + Send>),
    /// A reporter function is called on demand and/or on an interval
    /// It generates reports that can then be published
    Reporter(Box<dyn ReporterFunction + Send>),
    /// A reporter init function is called when psistats service starts
    ReporterInit(Box<dyn ReporterInitFunction + Send>)
}

/// A reporter function is called on demand and/or on an interval
/// It generates reports that can then be published
pub trait ReporterFunction {
    fn call(&self, config: &ReporterConfig) -> Result<PsistatsReport, PluginError>;
}

/// A reporter init function is called when psistats service starts
pub trait ReporterInitFunction {
    fn call(&self, config: &ReporterConfig) -> Result<(), PluginError>;
}

/// A publisher function is called every time a report has been
/// generated
pub trait PublisherFunction {
    fn call(&self, report: &PsistatsReport, config: &PublisherConfig) -> Result<(), PluginError>;
}

/// A publisher init function is called when psistats service starts
pub trait PublisherInitFunction {
    fn call(&self, config: &PublisherConfig) -> Result<(), PluginError>;
}

#[derive(Debug, Clone)]
pub enum PluginError {
    FunctionNotFound { p: String, fname: String },
    PluginFileNotFound { p: String },
    PluginDeclNotFound { p: String },
    Other { p: String, msg: String },
    Runtime { p: String,  msg: String },
}

impl fmt::Display for PluginError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PluginError::FunctionNotFound { p, fname } => { write!(f, "Plugin {} is lacking function {}", p, fname) },
            PluginError::PluginFileNotFound { p } => { write!(f, "Could not find plugin file for plugin {}", p) },
            PluginError::PluginDeclNotFound { p } => { write!(f, "Plugin declration not found for plugin {}", p) },
            PluginError::Other { p, msg } => { write!(f, "Error with plugin {}: {}", p, msg) },
            PluginError::Runtime { p, msg } => { write!(f, "Plugin {} failed to execute: {}", p, msg) }
        }
    }
}

impl std::error::Error for PluginError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// A plugin registrar is used to register plugins.
pub trait PluginRegistrar {

    /// Each plugin will need to call this method to register
    /// each function the plugin as available
    fn register_plugin(&mut self, name: &str, func: FunctionType);

    /// Register a plugin library. It's necessary to keep a reference
    /// of the plugin library active.
    fn register_lib(
        &mut self, lib: Arc<libloading::Library>
    );

    /// Get a reporter init function
    fn get_reporter_init(&self, name: &str) -> Result<&Box<dyn ReporterInitFunction + Send>, PluginError>;

    /// Get a reporter function
    fn get_reporter(&self, name: &str) -> Result<&Box<dyn ReporterFunction + Send>, PluginError>;

    /// Get a publisher init function
    fn get_publisher_init(&self, name: &str) -> Result<&Box<dyn PublisherInitFunction + Send>, PluginError>;

    /// Get a publisher function
    fn get_publisher(&self, name: &str) -> Result<&Box<dyn PublisherFunction + Send>, PluginError>;
}

/// Every plugin must expose (usually with the export_plugin! macro)
/// that defines a register method. This method is given a PluginRegistrar
/// so that the plugin can then register its functions with the given registrar
#[derive(Copy, Clone)]
pub struct PsistatsPlugin {
    pub register: unsafe extern "C" fn(&mut Box<dyn PluginRegistrar + 'static + Send>),
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static PSISTATS_PLUGIN: $crate::plugins::api::PsistatsPlugin =
            $crate::plugins::api::PsistatsPlugin {
                register: $register
            };
    };
}

/// A possible report value
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ReportValue {
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<ReportValue>),
    Object(HashMap<String, ReportValue>)
}

/// A PsistatsReport is what a reporter function should return
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PsistatsReport {
    pub id: String,
    pub value: ReportValue
}

impl PsistatsReport {
    pub fn new(id: &str, value: ReportValue) -> Self {
        PsistatsReport {
            id: id.to_string(),
            value: value
        }
    }

    pub fn get_id(&self) -> &String {
        return &self.id;
    }

    pub fn get_value(&self) -> &ReportValue {
        return &self.value;
    }
}
