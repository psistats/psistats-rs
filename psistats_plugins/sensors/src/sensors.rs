use sensors::Sensors;
use lazy_static::lazy_static;
use psistats::PsistatsReport;
use psistats::ReportValue;

lazy_static! {
  static ref SENSORS: Sensors = Sensors::new();
}

pub fn get_report() -> PsistatsReport {
  for chip in *SENSORS {
    println!(
      "{} (on {})",
      chip.get_name().unwrap(),
      chip.bus().get_adapter_name().unwrap()
    );

    for feature in chip {
      println!("  - {}", feature.get_label().unwrap());
      for subfeature in feature {
        println!(
          "    - {} = {}",
          subfeature.name(),
          subfeature.get_value().unwrap()
        );
      }
    }
  }

  return PsistatsReport::new("sensors", ReportValue::String("foobar".to_string()));
}
