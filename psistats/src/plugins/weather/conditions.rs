use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Conditions {
  Clear,
  MainlyClear,
  PartlyCloudy,
  MostlyCloudy,
  Cloudy,
  LightRain,
  HeavyRain,
  Rain,
  Thunderstorm,
  Flurries,
  Squalls,
  LightSnow,
  HeavySnow,
  Snow,
  Drizzle,
  LightFreezingRain,
  FreezingRain,
  HeavyFreezingRain,
  Fog,
  Hail,
  Unknown
}

impl fmt::Display for Conditions {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
