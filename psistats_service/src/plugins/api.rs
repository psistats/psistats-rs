use std::rc::Rc;
use libloading;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::collections::HashMap;

pub trait PsistatsFunction {
    fn call(&self, config: toml::Value);
}

pub trait PsistatsReportFunction {
    fn call(&self) -> Result<PsistatsReport, PsistatsError>;
}

pub trait PsistatsInitFunction {
    fn call(&self) -> Result<(), PsistatsError>;
}

pub enum PsistatsError {
    Other(String),
}

pub enum PsistatsFunctionTypes {
    INIT,
    REPORT,
    PUBLISH,
}

pub trait PluginRegistrar {

    fn register_init_fn(
        &mut self,
        name: &str,
        func: Box<dyn PsistatsInitFunction>
    );

    fn register_report_fn(
        &mut self,
        name: &str,
        func: Box<dyn PsistatsReportFunction>
    );

    fn register_lib(
        &mut self, lib: Rc<libloading::Library>
    );

    fn get_init_fn(&self, name: &str) -> Result<&Box<dyn PsistatsInitFunction>, String>;
    fn get_report_fn(&self, name: &str) -> Result<&Box<dyn PsistatsReportFunction>, String>;

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

pub enum ReportValue {
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<ReportValue>),
    Object(HashMap<String, ReportValue>)
}

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
