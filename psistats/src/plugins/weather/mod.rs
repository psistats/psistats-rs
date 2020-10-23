mod conditions;

pub use conditions::Conditions;
use std::collections::HashMap;
use super::super::plugins::api::ReportValue;


#[derive(Debug, Clone)]
pub struct CurrentConditions {
  pub temperature: f32,
  pub conditions: Conditions,
  pub relative_humidity: f32,
  pub dewpoint: f32,
  pub wind_speed: f32,
  pub wind_direction: String,
  pub wind_gusts: f32,
  pub pressure: f32,
}

impl CurrentConditions {
  pub fn init() -> Self {
    CurrentConditions {
      temperature: 0.0,
      conditions: Conditions::Unknown,
      relative_humidity: 0.0,
      dewpoint: 0.0,
      wind_speed: 0.0,
      wind_direction: "N".to_string(),
      wind_gusts: 0.0,
      pressure: 0.0
    }
  }

  pub fn to_report_value(&self) -> ReportValue {
    let mut map: HashMap<String, ReportValue> = HashMap::new();

    map.insert("temperature".to_string(), ReportValue::Float(self.temperature.into()));
    map.insert("conditions".to_string(), ReportValue::String(format!("{}", self.conditions)));
    map.insert("relativeHumidity".to_string(), ReportValue::Float(self.relative_humidity.into()));
    map.insert("dewpoint".to_string(), ReportValue::Float(self.dewpoint.into()));
    map.insert("windSpeed".to_string(), ReportValue::Float(self.wind_speed.into()));
    map.insert("windDirection".to_string(), ReportValue::String(self.wind_direction.clone()));
    map.insert("windGusts".to_string(), ReportValue::Float(self.wind_gusts.into()));
    map.insert("pressure".to_string(), ReportValue::Float(self.pressure.into()));

    return ReportValue::Object(map);
  }
}

pub fn c_to_f(c: f64) -> f64 {
  return (c * 9.0 / 5.0) + 32.0;
}

pub fn f_to_c(f: f64) -> f64 {
  return (f - 32.0) * 5.0 / 9.0;
}