use std::{collections::HashMap, env, ffi::OsStr, io, path::PathBuf};
use std::{rc::Rc};
use crate::plugins::api::{PsistatsPlugin, PluginRegistrar};
use libloading::Library;
use glob::glob;

pub struct PluginLoader;

impl PluginLoader {

    pub unsafe fn loadPlugin<P: AsRef<OsStr>>(&mut self, plugin_file: P) -> io::Result<PsistatsPlugin> {
        let lib = Rc::new(Library::new(plugin_file)?);
        let decl = lib.get::<*mut PsistatsPlugin>(b"__PSISTATS_PLUGIN\0")?.read();

        Ok(decl)
    }

    pub unsafe fn loadPlugins<P: AsRef<OsStr>>(&mut self, pluginDir: P) -> io::Result<()> {

        let mut pdir = "";

        match pluginDir.as_ref().to_str() {
            None => pdir = ".",
            Some(val) => pdir = val
        };

        let globptn = format!("{}/*.dll", pdir);

        for entry in glob(&globptn).unwrap() {
            println!("Plugin file: {:?}", entry);
            self.loadPlugin(OsStr::new(&entry.unwrap())).unwrap();
            
        }

        Ok(())
    }
}
