use crate::plugins::api::PluginRegistrar;
use crate::plugins::api::PsistatsFunction;
use crate::plugins::api::PsistatsFunctionTypes;
use std::collections::HashMap;
use std::rc::Rc;
use libloading::Library;

#[derive(Default)]
pub struct DefaultPluginRegistrar {
    init_fn: HashMap<String, Box<dyn PsistatsFunction>>,
    report_fn: HashMap<String, Box<dyn PsistatsFunction>>,
    publish_fn: HashMap<String, Box<dyn PsistatsFunction>>,
    libs: Vec<Rc<Library>>
}

impl DefaultPluginRegistrar {
    pub fn new() -> DefaultPluginRegistrar {
        DefaultPluginRegistrar::default()
    }
}

impl PluginRegistrar for DefaultPluginRegistrar {
    fn register_fn(
        &mut self,
        name: &str,
        fn_type: PsistatsFunctionTypes,
        cb: Box<dyn PsistatsFunction>,
    ) {
        match fn_type {
            PsistatsFunctionTypes::INIT => {
                self.init_fn.insert(name.to_string(), cb);
            }
            PsistatsFunctionTypes::PUBLISH => {
                self.publish_fn.insert(name.to_string(), cb);
            }
            PsistatsFunctionTypes::REPORT => {
                self.report_fn.insert(name.to_string(), cb);
            }
        }
    }

    fn register_lib(&mut self, lib: Rc<libloading::Library>) {
        self.libs.push(lib);
    }

    fn count(&self, fn_type: PsistatsFunctionTypes) -> usize {
        match fn_type {
            PsistatsFunctionTypes::INIT => self.init_fn.len(),
            PsistatsFunctionTypes::REPORT => self.report_fn.len(),
            PsistatsFunctionTypes::PUBLISH => self.publish_fn.len(),
        }
    }
}
