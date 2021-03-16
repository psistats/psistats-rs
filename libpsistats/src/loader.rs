use crate::PluginRegistrar;
use crate::PsistatsPlugin;
use crate::errors::PsistatsError;
use libloading::Library;
use std;
use std::sync::Arc;
use std::{ffi::OsStr, fmt};

/// Plugin Loader
///
/// The plugin loader will load plugins from a specified
/// directory and use [PluginRegistrar](libpsistats::PluginRegistrar) to
/// register the plugins.
///
/// Plugins in the directory are expected to be named libplugin_[pluginName].so
/// on *nix systems, and plugin_[name].dll on Windows.
///
/// ```
/// let pl = PluginLoader::new("/path/to/plugins");
/// let mut registrar = DefaultPluginRegistrar::new();
///
/// pl.load_plugin(&registrar, "foo"); // Loads /path/to/plugins/libplugin_foo.so
/// ```
pub struct PluginLoader {
  plugin_dir: String,
}

impl PluginLoader {
  pub fn new(plugin_dir: String) -> Self {
    PluginLoader { plugin_dir }
  }

  pub unsafe fn load_plugin<P: AsRef<OsStr>>(
    &self,
    plugin_name: P,
    registrar: &mut Box<dyn PluginRegistrar + Send + Sync>,
  ) -> Result<(), PsistatsError> {
    let plugin_file: String;

    if cfg!(target_os = "windows") {
      plugin_file = format!(
        "{}\\plugin_{}.dll",
        self.plugin_dir,
        plugin_name.as_ref().to_str().unwrap()
      );
    } else if cfg!(target_os = "linux") {
      plugin_file = format!(
        "{}/libplugin_{}.so",
        self.plugin_dir,
        plugin_name.as_ref().to_str().unwrap()
      );
    } else {
      return Err(PsistatsError::Runtime(
        "Plugin loader does not support host OS".to_string(),
      ));
    }

    return self.load_plugin_file(plugin_file, registrar);
  }

  unsafe fn load_plugin_file<P: AsRef<OsStr>>(
    &self,
    plugin_file: P,
    registrar: &mut Box<dyn PluginRegistrar + Send + Sync>,
  ) -> Result<(), PsistatsError> {
    let lib = Library::new(&plugin_file);
    let lib_rc: Arc<Library>;
    match lib {
      Ok(l) => {
        lib_rc = Arc::new(l);
      }
      Err(err) => {
        return Err(PsistatsError::PluginLibError(format!("{}", err)));
      }
    }

    let decl_ref = lib_rc.get::<*mut PsistatsPlugin>(b"PSISTATS_PLUGIN\0");
    let decl: PsistatsPlugin;
    match decl_ref {
      Ok(dref) => {
        decl = dref.read();
      }
      Err(err) => {
        return Err(PsistatsError::PluginDeclError(format!("{}", err)));
      }
    }

    (decl.register)(registrar);

    registrar.register_lib(lib_rc);

    Ok(())
  }
}
