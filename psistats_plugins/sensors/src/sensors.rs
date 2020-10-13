#[cfg(not(target_os = "windows"))]
use sensors::Sensors;

use lazy_static::lazy_static;
use psistats::PsistatsReport;
use psistats::ReportValue;
use std::collections::HashMap;

#[cfg(target_os = "windows")]
pub fn get_report() -> PsistatsReport {
  return PsistatsReport::new("sensors", ReportValue::String("Sensors are not available on windows"));
}

#[cfg(not(target_os = "windows"))]
lazy_static! {
  static ref SENSORS: Sensors = Sensors::new();
}

#[cfg(not(target_os = "windows"))]
pub fn get_report() -> PsistatsReport {


  let mut sensorData:HashMap<String, ReportValue> = HashMap::new();

  for chip in *SENSORS {

    let mut chipData:HashMap<String, ReportValue> = HashMap::new();

    let chipName = chip.get_name().unwrap();

    for feature in chip {

      let mut featureData:HashMap<String, ReportValue> = HashMap::new();
      let featureName = feature.name().to_string();


      for subfeature in feature {
        let subname = subfeature.name().clone().to_string();
        let value = subfeature.get_value().unwrap().clone();
        featureData.insert(subname, ReportValue::Float(value));
      }

      chipData.insert(featureName, ReportValue::Object(featureData));
    }

    sensorData.insert(chipName, ReportValue::Object(chipData));
  }

  return PsistatsReport::new("sensors", ReportValue::Object(sensorData));
}
