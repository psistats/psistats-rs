use sensors::Sensors;
use libpsistats::{ReportValue};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
  static ref SENSORS: Sensors = Sensors::new();
}

pub fn get_report(includes: &Vec<String>, mapping: &HashMap<String, String>) -> ReportValue {


  let mut sensor_data: Vec<ReportValue> = vec!();

  for chip in *SENSORS {

    let chip_name = chip.get_name().unwrap();

    for feature in chip {

      let feature_name = feature.name().to_string();


      for subfeature in feature {
        let subname = subfeature.name().clone().to_string();
        let value = subfeature.get_value().unwrap().clone();
        let mut sensor_name = format!("/{}/{}/{}", chip_name, feature_name, subname);

        if mapping.contains_key(&sensor_name) {
          sensor_name = mapping.get(&sensor_name).unwrap().clone();
        }



        if includes.len() == 0 || includes.contains(&sensor_name) {
          let report_value = ReportValue::Array([
            ReportValue::String(sensor_name),
            ReportValue::Float(value)
          ].to_vec());

          sensor_data.push(report_value);
        }
      }
    }
  }

  return ReportValue::Array(sensor_data);
}
