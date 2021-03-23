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
//! An example plugin that simply reports a counter:
//! ```
//! #[macro_use]
//! extern crate lazy_static;
//!
//! use libpsistats::PluginRegistrar;
//! use libpsistats::PsistatsError;
//! use libpsistats::{ ReporterFunction, PluginSettings, ReportValue };
//! use std::sync::Mutex;
//!
//! // Step 1: Create a state struct for the plugin.
//! struct CounterState {
//!   pub counter: u64
//! }
//!
//! impl CounterState {
//!   pub fn new() -> Self {
//!     CounterState { counter: 0 }
//!   }
//! }
//!
//! // Step 2: Expose it as a static reference inside a Mutex for thread safety.
//! lazy_static! {
//!   static ref STATE: Mutex<CounterState> = Mutex::new(CounterState::new());
//! }
//!
//!
//! // Step 3: Setup the reporter callback
//! #[derive(Debug, Clone, PartialEq)]
//! struct CounterReporter;
//! impl ReporterFunction for CounterReporter {
//!   fn call(&self, _: &PluginSettings) -> Result<ReportValue, PsistatsError> {
//!     let mut state = STATE.lock().unwrap();
//!     state.counter = state.counter + 1;
//!
//!     Ok(ReportValue::Integer(state.counter))
//!   }
//! }
//!
//! // Step 4: Setup the plugin entry point
//! extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
//!   registrar.register_reporter_fn("counter", Box::new(CounterReporter));
//! }
//! libpsistats::export_plugin!(register);
//!
//! ```
//!
//! Plugins should be built with the crate type `cdylib`. The library name should be prefixed with `plugin_`.
//!
//! ```
//! # Cargo.toml
//! [package]
//! name = "example_counter_plugin"
//! version = "0.0.1"
//! edition = "2018"
//!
//! [lib]
//! crate-type = ["cdylib"]
//! name = "plugin_counter"
//!
//! [dependencies]
//! libpsistats = { git = "https://github.com/psistats/psistats-rs" }
//! lazy_static = "1.4"
//! ```
//!
//! It can then be added to the psistats configuration;
//!
//! ```
//! # psistats.toml
//! [[plugin]]
//! name="counter"
//! enabled=true
//! interval=1
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
