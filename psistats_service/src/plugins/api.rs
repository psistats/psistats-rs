use std::rc::Rc;
use libloading;

pub trait PsistatsFunction {
    fn call(&self, config: toml::Value);
}

pub enum PsistatsError {
    Other { msg: String },
}

pub enum PsistatsFunctionTypes {
    INIT,
    REPORT,
    PUBLISH,
}

pub trait PluginRegistrar {
    fn register_fn(
        &mut self,
        name: &str,
        fn_type: PsistatsFunctionTypes,
        cb: Box<dyn PsistatsFunction>,
    );
    fn register_lib(
        &mut self, lib: Rc<libloading::Library>
    );
    fn count_fn(&self, fn_type: PsistatsFunctionTypes) -> usize;
    fn count_libs(&self) -> usize;
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
