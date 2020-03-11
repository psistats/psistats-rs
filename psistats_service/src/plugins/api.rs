use std::rc::Rc;
use libloading;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::collections::HashMap;
use crate::{ ReporterConfig, PublisherConfig };

pub enum FunctionType {
    Publisher(Box<dyn PublisherFunction>),
    PublisherInit(Box<dyn PublisherInitFunction>),
    Reporter(Box<dyn ReporterFunction>),
    ReporterInit(Box<dyn ReporterInitFunction>)
}

pub trait ReporterFunction {
    fn call(&self, config: &ReporterConfig) -> Result<PsistatsReport, PluginError>;
}

pub trait ReporterInitFunction {
    fn call(&self, config: &ReporterConfig) -> Result<(), PluginError>;
}

pub trait PublisherFunction {
    fn call(&self, config: &PublisherConfig) -> Result<(), PluginError>;
}

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

pub trait PluginRegistrar {

    fn register_plugin(&mut self, name: &str, func: FunctionType);
 
    fn register_lib(
        &mut self, lib: Rc<libloading::Library>
    );

    fn get_reporter_init(&self, name: &str) -> Result<&Box<dyn ReporterInitFunction>, PluginError>;
    fn get_reporter(&self, name: &str) -> Result<&Box<dyn ReporterFunction>, PluginError>;

    fn get_publisher_init(&self, name: &str) -> Result<&Box<dyn PublisherInitFunction>, PluginError>;
    fn get_publisher(&self, name: &str) -> Result<&Box<dyn PublisherFunction>, PluginError>;
}

#[derive(Copy, Clone)]
pub struct PsistatsPlugin {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut Box<dyn PluginRegistrar + 'static>),
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static PSISTATS_PLUGIN: $crate::plugins::api::PsistatsPlugin =
            $crate::plugins::api::PsistatsPlugin {
                rustc_version: $crate::RUSTC_VERSION,
                core_version: $crate::CORE_VERSION,
                register: $register,
            };
    };
}

#[derive(Deserialize, Serialize, Clone)]
pub enum ReportValue {
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<ReportValue>),
    Object(HashMap<String, ReportValue>)
}

#[derive(Deserialize, Serialize, Clone)]
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
