/// Macro to set the plugin entry point
///
/// All plugins must use this macro to define the plugin entry point.
///
/// The plugin entry point is a register function that is called by psistats. The register
/// function should use the given plugin registrar to register all its callback functions.
///
/// The register function must be compatible with [`libpsistats::register`]. Example usage:
///
/// ```
/// extern "C" fn register(registrar: &mut Box<dyn PluginRegistrar + Send + Sync>) {
///   registrar.register_reporter_fn("myplugin", Box::new(Reporter)));
/// }
///
/// libpsistats::export_plugin!(register);
/// ```
#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static PSISTATS_PLUGIN: $crate::PsistatsPlugin =
            $crate::PsistatsPlugin {
                register: $register
            };
    };
}