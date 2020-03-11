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
pub struct Error {
    msg: String
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
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
            return Err(Error { msg: "Plugin loader does not support host OS".to_string()});
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
                println!("{}", err);
                return Err(Error { msg: format!("Error loading plugin file {}", &plugin_file.as_ref().to_str().unwrap()) });
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
                println!("{}", err);
                return Err(Error { msg: format!("Unable to load PSISTATS_PLUGIN for plugin {}", &plugin_file.as_ref().to_str().unwrap())});
            }
        }

        println!("load_plugin() -> Register?");
        (decl.register)(registrar);

        registrar.register_lib(lib_rc);

        Ok(())
    }

    pub unsafe fn load_plugins<P: AsRef<OsStr>>(
        &mut self,
        plugin_dir: P,
        registrar: &mut Box<dyn PluginRegistrar>,
    ) -> Result<(), String>
    {
        
        let pdir: String;

        match plugin_dir.as_ref().to_str() {
            None => pdir = ".".to_string(),
            Some(val) => pdir = val.to_string(),
        };

        let path = Path::new(&pdir);

        if path.exists() == false {
            return Err(format!("Plugin directory {} does not exist!", pdir).to_string());
        }

        let globptn = format!("{}/*.dll", pdir);

        let entries = glob(&globptn).unwrap();
        

        for entry in entries {
            let e = entry.unwrap();
            let entstr = OsStr::new(&e);
            println!("Loading plugin {:?}", entstr);
            match self.load_plugin(entstr, registrar) {
                Ok(_) => (),
                Err(err) => return Err(format!("Error loading plugin {:?}: {}", entstr, err))
            }
        }

        Ok(())
    }
}
