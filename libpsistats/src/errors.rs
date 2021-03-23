use thiserror::Error;

/// List of errors libpsistats will generate
#[derive(Error, Debug, Clone)]
pub enum PsistatsError {

  /// Raised when the required PSISTATS_PLUGIN exported symbol is not found
  /// See [export_plugin!](libpsistats::export_plugin) for more information
  #[error("Plugin register function not found for plugin {0}")]
  RegisterFuncNotFound(String),

  /// Raised when a plugin file is not found
  #[error("Plugin file not found for plugin {0}")]
  PluginFileNotFound(String),

  /// Raised when a plugin configuration can't be found
  #[error("Configuration for {0} not found")]
  PluginConfigNotFound(String),

  /// Raised if the plugin loader can't find PSISTATS_PLUGIN symbol in the
  /// plugin file. See [export_plugin!](libpsistats::export_plugin) for more
  /// information.
  #[error("Error with the plugin declaration: {0}")]
  PluginDeclError(String),

  /// Raised if there was a problem loading the plugin file
  #[error("Error with the plugin lib: {0}")]
  PluginLibError(String),

  /// Catchall error for other errors coming out of libpsistats
  #[error("Runtime error {0}")]
  Runtime(String)
}
