use crate::plugins::api::{PluginRegistrar, PsistatsPlugin};
use glob::glob;
use libloading::Library;
use std::rc::Rc;
use std::{ffi::OsStr, io, path::Path};

pub struct PluginLoader;

impl PluginLoader {
    pub unsafe fn load_plugin<P: AsRef<OsStr>>(
        &mut self,
        plugin_file: P,
        registrar: &mut Box<dyn PluginRegistrar>,
    ) -> io::Result<PsistatsPlugin>
    {
        println!("load_plugin() -> Loading lib");
        let lib = Rc::new(Library::new(plugin_file)?);

        println!("load_plugin() -> getting __PSISTATS_PLUGIN decl");
        let decl = lib
            .get::<*mut PsistatsPlugin>(b"PSISTATS_PLUGIN\0")?
            .read();

        println!("load_plugin() -> Register?");
        (decl.register)(registrar);

        registrar.register_lib(lib);

        Ok(decl)
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
