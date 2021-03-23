use std::path::Path;

pub fn get_config_location() -> Option<&str> {

  let locations = [
    "psistats.conf",
    "~/.psistats.conf",
    "/etc/psistats.conf"
  ];

  for l in locations.iter() {
    if (Path::new(l).exists()) {
      return Some(l);
    }
  }

  None

}