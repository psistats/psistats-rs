use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

/// Report Value
///
/// Reporter functions should generate a ReportValue using one of the five
/// base types.
///
/// Before being sent to publisher functions, the ReportValue will be wrapped by
/// [PsistatsReport](libpsistats::PsistatsReport) and will include some additional
/// metadata such as the plugin name, and hostname.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ReportValue {
    Integer(u64),
    Float(f64),
    String(String),
    Array(Vec<ReportValue>),
    Object(HashMap<String, ReportValue>)
}

/// The PsistatsReport is generated internally from the [ReportValue](libpsistats::ReportValue) generated
/// by a reporter.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PsistatsReport {
  pub reporter: String,
  pub hostname: String,
  pub value: ReportValue
}

impl PsistatsReport {
    pub fn new(reporter: &str, hostname: &str, value: ReportValue) -> Self {
        PsistatsReport {
            reporter: reporter.to_string(),
            hostname: hostname.to_string(),
            value: value
        }
    }

    /// Convert the report to JSON
    pub fn as_json(&self) -> String {
      return serde_json::to_string(self).unwrap();
    }
}
