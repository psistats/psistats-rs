use glob::glob;
use libloading::Library;
use std::rc::Rc;
use std::{ffi::OsStr, io, path::Path, fmt};
use std;
use crate::PluginRegistrar;
use crate::PsistatsPlugin;
use crate::PluginError;

pub struct PluginLoader {
    plugin_dir: String
}

#[derive(Debug, Clone)]
pub enum Error {
  DeclError(String),
  LibError(String),
  Other(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        Error::DeclError(msg) => write!(f, "{}", msg),
        Error::LibError(msg) => write!(f, "{}", msg),
        Error::Other(msg) => write!(f, "{}", msg)
      }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl PluginLoader {
    pub fn new(plugin_dir: String) -> Self {
        PluginLoader {
            plugin_dir
        }
    }

    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&self, plugin_name: P, registrar: &mut Box<dyn PluginRegistrar>) -> Result<(), Error> {
        let plugin_file: String;

        if cfg!(target_os = "windows") {
            plugin_file = format!("{}\\plugin_{}.dll", self.plugin_dir, plugin_name.as_ref().to_str().unwrap());
        } else {
            return Err(Error::Other("Plugin loader does not support host OS".to_string()));
        }

        return self.load_plugin_file(plugin_file, registrar);
    }

    unsafe fn load_plugin_file<P: AsRef<OsStr>>(
        &self,
        plugin_file: P,
        registrar: &mut Box<dyn PluginRegistrar>,
    ) -> Result<(), Error>
    {
        println!("load_plugin() -> Loading lib");



        let lib = Library::new(&plugin_file);
        let lib_rc: Rc<Library>;
        match lib {
            Ok(l) => {
                lib_rc = Rc::new(l);
            },
            Err(err) => {
                return Err(Error::LibError(format!("Error loading library: {}", err)));
            }
        }

        println!("load_plugin() -> getting PSISTATS_PLUGIN decl");

        let decl_ref = lib_rc.get::<*mut PsistatsPlugin>(b"PSISTATS_PLUGIN\0");
        let mut decl: PsistatsPlugin;
        match decl_ref {
            Ok(dref) => {
                decl = dref.read();
            }
            Err(err) => {
                return Err(Error::DeclError(format!("Plugin declaration error: {}", err)));
            }
        }

        (decl.register)(registrar);

        registrar.register_lib(lib_rc);

        Ok(())
    }
}
