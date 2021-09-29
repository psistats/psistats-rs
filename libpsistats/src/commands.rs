/// List of available commands
///
/// Currently only one command is supported, which is
/// to trigger a report by the plugin name
#[derive(Debug)]
pub enum Commands {
  Report(String)
}