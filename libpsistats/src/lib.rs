//! libpsistats - library for authoring psistats plugins
//!
//! Psistats plugins must implement specific traits and define
//! certain symbols in order to use them with the psistats reporting tool.
//!
//! Every plugin must define at least one callback that implements
//! [`ReporterFunction`], [`PublisherFunction`], or [`InitFunction`]. As well, the plugin
//! needs to export a symbol called `PSISTATS_PLUGIN` which implements [`PsistatsPlugin`]
//! This can be made easier using the [`export_plugin`] macro.
//!
//! For a plugin to be loaded, it'll need to be part of psistats' configuration.
//!
//! An example plugin that simply reports a counter, and publishes by just
//! printing the received report to stdout.
//! ```
//! #[macro_use]
//! extern crate lazy_static;
//!
//! use libpsistats::PluginRegistrar;
//! use libpsistats::FunctionType;
//! use libpsistats::PluginError;
//! use libpsistats::{ ReporterFunction, PublisherFunction, PluginSettings, ReportValue };
//! use std::sync::Mutex;
//!
//! // Create some static state for our plugin
//! struct CounterState {
//!   pub counter: u64
//! }
//!
//! impl CounterState {
//!  pub fn new() -> Self {
//!    CounterState { counter: 0 }
//!  }
//!
//!  pub fn inc(&mut self) {
//!    self.counter = self.counter + 1;
//!  }
//! }
//!
//! lazy_static! {
//!  static ref COUNTER: Mutex<CounterState> = Mutex::new(CounterState::new());
//! }
//!
//! // define the reporter
//!
//! #[derive(Debug, Clone, PartialEq)]
//! struct CounterReporter;
//!
//! impl ReporterFunction for CounterReporter {
//!   fn call(&self, _: &PluginSettings) -> Result<ReportValue, PsistatsError> {
//!     let mut counter = COUNTER.lock().unwrap();
//!     counter.inc();
//!
//!     Ok(ReportValue::Integer(counter.counter))
//!   }
//! }
//!
//! struct SimplePublisher;
//! impl PublisherFunction for SimplePublisher {
//!   fn call(&self, report: &PsistatsReport, _: &PluginSettings) -> Result<(), PsistatsError> {
//!     println!("{}", report.as_json());
//!   }
//! }
//!
//! extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send>) {
//!   registrar.register_reporter_fn("example", Box::new(CounterReporter));
//!   registrar.register_publisher_fn("example", Box::new(SimplePublisher));
//! }
//!
//! libpsistats::export_plugin!(register);
//! ```
//!
//! Plugins should be built with the crate type `cdylib`. The library name should be prefixed with `plugin_`.
//!
//! ```
//! # Cargo.toml
//! [package]
//! name = "example_plugin"
//! version = "0.0.1"
//! edition = "2018"
//!
//! [lib]
//! crate-type = ["cdylib"]
//! name = "plugin_example"
//!
//! [dependencies]
//! libpsistats = { git = "https://github.com/psistats/psistats-rs" }
//! lazy_static = "1.4"
//! ```
mod plugin_traits;
mod reports;
mod registrar;
mod default_registrar;
mod errors;
mod config;
mod macros;
mod loader;

pub use registrar::PluginRegistrar;
pub use default_registrar::DefaultPluginRegistrar;
pub use plugin_traits::ReporterFunction;
pub use plugin_traits::InitFunction;
pub use plugin_traits::PublisherFunction;
pub use reports::ReportValue;
pub use reports::PsistatsReport;
pub use errors::PsistatsError;
pub use config::PluginSettings;
pub use loader::PluginLoader;


/// Plugin Entry Point
///
/// This is the plugin entry point. Every plugin is expected to export a symbol
/// called `PSISTATS_PLUGIN` which implements the register function in this struct.
///
/// This is low level, and you should typically use the [`export_plugin`] macro instead.
#[derive(Copy, Clone)]
pub struct PsistatsPlugin {
    pub register: unsafe extern "C" fn(&mut Box<dyn PluginRegistrar + 'static + Send + Sync>)
}
