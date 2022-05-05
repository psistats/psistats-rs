use libpsistats::{ReportValue, PluginSettings};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
  static ref INCLUDES: Mutex<Vec<String>> = Mutex::new(vec![]);
  static ref MAPPING: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[cfg(not(target_os = "windows"))]
use crate::lmsensors;

#[cfg(target_os = "windows")]
use crate::ohmsensors;

pub fn init(settings: &PluginSettings) {

  let mut includes = INCLUDES.lock().unwrap();

  if settings.get_config().contains_key("includes") {
    let includes_conf = settings.get_config().get("includes").unwrap().as_array().unwrap();

    for i in includes_conf {
      includes.push(i.as_str().unwrap().to_string());
    }
  }

  let mut mapping = MAPPING.lock().unwrap();

  if settings.get_config().contains_key("mapping") {
    let mapping_conf = settings.get_config().get("mapping").unwrap().as_array().unwrap();

    for i in mapping_conf {
      mapping.insert(i[0].as_str().unwrap().to_string(), i[1].as_str().unwrap().to_string());
    }
  }

}

pub fn get_report(_: &PluginSettings) -> ReportValue {

  let includes = INCLUDES.lock().unwrap();
  let mapping = MAPPING.lock().unwrap();

  #[cfg(not(target_os = "windows"))]
  return lmsensors::get_report(&includes, &mapping);

  #[cfg(target_os = "windows")]
  return ohmsensors::get_report(&INCLUDES.lock().unwrap());
}

/*
#[cfg(target_os = "windows")]
pub fn get_report() -> ReportValue {
  return ReportValue::String("Sensors are not available on windows".to_string());
}

#[cfg(not(target_os = "windows"))]
lazy_static! {
  static ref SENSORS: Sensors = Sensors::new();
}

#[cfg(not(target_os = "windows"))]
pub fn get_report() -> ReportValue {


  let mut sensor_data:HashMap<String, ReportValue> = HashMap::new();

  for chip in *SENSORS {

    let mut chip_data:HashMap<String, ReportValue> = HashMap::new();

    let chip_name = chip.get_name().unwrap();

    for feature in chip {

      let mut feature_data:HashMap<String, ReportValue> = HashMap::new();
      let feature_name = feature.name().to_string();


      for subfeature in feature {
        let subname = subfeature.name().clone().to_string();
        let value = subfeature.get_value().unwrap().clone();
        feature_data.insert(subname, ReportValue::Float(value));
      }

      chip_data.insert(feature_name, ReportValue::Object(feature_data));
    }

    sensor_data.insert(chip_name, ReportValue::Object(chip_data));
  }

  return ReportValue::Object(sensor_data);
}
*/