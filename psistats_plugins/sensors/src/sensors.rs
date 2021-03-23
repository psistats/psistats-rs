#[cfg(not(target_os = "windows"))]
use sensors::Sensors;

use lazy_static::lazy_static;
use libpsistats::ReportValue;
use std::collections::HashMap;

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
