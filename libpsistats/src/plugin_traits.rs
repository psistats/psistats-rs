//! Plugin Traits
//!
//! These traits define the callbacks a plugin can define. Plugins can
//! define an [`InitFunction`], [`ReporterFunction`], or [`PublisherFunction`].
//!
//! There is no teardown / shutdown callback for now.
use crate::PsistatsError;
use crate::PluginSettings;
use crate::PsistatsReport;
use crate::ReportValue;


/// An init function is called when psistats is first loaded. Init functions
/// can do things like start additional threads or set some initial state.
///
/// Example:
/// ```
/// use libpsistats::{InitFunction, PsistatsError, PluginSettings};
/// use std::sync::Mutex;
///
/// struct MyPluginState {
///   pub counter: u32
/// }
///
/// impl MyPluginState {
///   new() -> Self { MyPluginState { counter: 0 }}
/// }
///
/// lazy_static! {
///   static ref STATE: Mutex<MyPluginState> = Mutex::new(MyPluginState::new());
/// }
///
/// struct MyPluginInit;
/// impl InitFunction(hostname: &str, settings: &PluginSettings) for MyPluginInit -> Result<(), PsistatsError> {
///   let starting_point = settings.get_config().get("startingPoint").unwrap();
///   let locked_state = STATE.lock().unwrap();
///
///   locked_state.counter = starting_point;
///
///   Ok(())
/// }
/// ```
pub trait InitFunction {
  fn call(&self, hostname: &str, settings: &PluginSettings) -> Result<(), PsistatsError>;
}

/// Reporter functions generate [`ReportValue`]s. They are usually called
/// at configured intervals, though can be configured to idle until manually
/// triggered.
///
/// Example
///
/// ```
/// struct MyPluginReporter;
/// impl ReporterFunction for MyPluginReporter {
///   call(&self, _: &PluginSettings) -> Result<ReportValue, PsistatsError> {
///     Ok(ReportValue::String("report!")))
///   }
/// }
/// ```
pub trait ReporterFunction {
  fn call(&self, settings: &PluginSettings) -> Result<ReportValue, PsistatsError>;
}

/// Publisher functions are called for every report that is generated.
///
/// Example
///
/// ```
/// use libpsistats::{PublisherFunciton, PsistatsReport, PsistatsError, PluginSettings}
///
/// struct MyPluginPublisher;
/// impl PublisherFunction for MyPluginPublisher -> Result<(), PsistatsError> {
///   fn call(&self, report: &PsistatsReport, _: &PluginSettings) -> Result<(), PsistatsError> {
///     println!("report: {}", report.as_json());
///     Ok(())
///   }
/// }
/// ```
pub trait PublisherFunction {
  fn call(&self, report: PsistatsReport, settings: &PluginSettings) -> Result<(), PsistatsError>;
}
