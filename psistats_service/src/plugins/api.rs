pub trait PsistatsFunction {
    fn call(&self, config: toml::Value);
}

pub enum PsistatsError {
    Other { msg: String }
}

pub enum PsistatsFunctionTypes {
    INIT,
    REPORT,
    PUBLISH
}

pub trait PluginRegistrar {
    fn register_init_fn(&mut self, name: &str, cb: Box<dyn PsistatsFunction>);
    fn register_report_fn(&mut self, name: &str, cb: Box<dyn PsistatsFunction>);
    fn register_publish_fn(&mut self, name: &str, cb: Box<dyn PsistatsFunction>);
}

#[derive(Copy, Clone)]
pub struct PsistatsPlugin {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar)
}


#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static __PSISTATS_PLUGIN: $crate::plugins::api::PsistatsPlugin = $crate::plugins::api::PsistatsPlugin {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: $register
        };
    };
}
