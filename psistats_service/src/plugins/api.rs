use std::rc::Rc;
use libloading;
use serde::{Serialize, Deserialize};
use std::fmt;

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
    fn call(&self, name: &str, fn_type: PsistatsFunctionTypes);
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
pub struct Report {
    pub id: String,
    pub value: String
}

impl Report {
    pub fn new(id: String, value: String) -> Self {
        Report {
            id: id,
            value: value
        }
    }

    pub fn to_string(&self) -> String {
        return self.to_json();
    }

    pub fn to_json(&self) -> String {
        let json_value: String;
        if self.value.starts_with('{') == true || self.value.starts_with('[') == true {
            json_value = format!("{}", self.value);
        } else if self.value.parse::<u64>().is_ok() {
            json_value = format!("{}", self.value);
        } else {
            json_value = format!("\"{}\"", self.value);
        }

        let json = format!("{{\"id\": \"{id}\", \"value\": {val}}}", 
            id = self.id, 
            val = json_value
        );

        return json;
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_json())
    }
}
