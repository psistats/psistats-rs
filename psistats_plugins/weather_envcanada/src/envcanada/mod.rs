use psistats::plugins::weather::Conditions;
use psistats::plugins::weather::CurrentConditions;
use reqwest;
use quick_xml::Reader;
use quick_xml::events::Event;

pub fn condition_mapper(condition: &str) -> Conditions {

  let mut val: Conditions;

  match condition {
    "Sunny" => val = Conditions::Clear,
    "Mainly Sunny" => val = Conditions::MainlyClear,
    "Partly Cloudy" => val = Conditions::PartlyCloudy,
    "Mostly Cloudy" => val = Conditions::MostlyCloudy,
    "Light Rain Shower" => val = Conditions::LightRain,
    "Light Rain Shower and Flurries" => val = Conditions::LightRain,
    "Light Flurries" => val = Conditions::Flurries,
    "Cloudy" => val = Conditions::Cloudy,
    "Precipitation" => val = Conditions::Rain,
    "Squalls" => val = Conditions::HeavySnow,
    "Light Precipitation" => val = Conditions::LightRain,
    "Heavy Precipitation" => val = Conditions::HeavyRain,
    "Rain Shower" => val = Conditions::Rain,
    "Light Rain and Drizzle" => val = Conditions::LightRain,
    "Light Rain" => val = Conditions::LightRain,
    "Rain" => val = Conditions::Rain,
    "Rain and Drizzle" => val = Conditions::Rain,
    "Heavy Rain and Drizzle" => val = Conditions::HeavyRain,
    "Heavy Rain Shower" => val = Conditions::HeavyRain,
    "Heavy Rain" => val = Conditions::HeavyRain,
    "Rain" => val = Conditions::Rain,
    "Light Freezing Drizzle" => val = Conditions::LightFreezingRain,
    "Light Freezing Rain" => val = Conditions::LightFreezingRain,
    "Heavy Freezing Drizzle" => val = Conditions::HeavyFreezingRain,
    "Heavy Freezing Rain" => val = Conditions::HeavyFreezingRain,
    "Freezing Drizzle" => val = Conditions::LightFreezingRain,
    "Freezing Rain" => val = Conditions::FreezingRain,
    "Rain and Flurries" => val = Conditions::Rain,
    "Rain and Snow" => val = Conditions::Snow,
    "Heavy Rain Shower and Flurries" => val = Conditions::HeavyRain,
    "Heavy Rain and Snow" => val = Conditions::HeavyRain,
    "Light Snow" => val = Conditions::LightSnow,
    "Snow" => val = Conditions::Snow,
    "Flurries" => val = Conditions::Flurries,
    "Heavy Flurries" => val = Conditions::Flurries,
    "Heavy Snow" => val = Conditions::HeavySnow,
    "Thunderstorm with Rain" => val = Conditions::Thunderstorm,
    "Thunderstorm with Heavy Rain" => val = Conditions::Thunderstorm,
    "Thunderstorm with Light Rain" => val = Conditions::Thunderstorm,
    "Thunderstorm with Rain" => val = Conditions::Thunderstorm,
    "Thunderstorm" => val = Conditions::Thunderstorm,
    "Heavy Thunderstorm" => val = Conditions::Thunderstorm,
    "Heavy Thunderstorm with Rain" => val = Conditions::Thunderstorm,
    "Haze" => val = Conditions::Fog,
    "Fog" => val = Conditions::Fog,
    "Ice Fog" => val = Conditions::Fog,
    "Fog Patches" => val = Conditions::Fog,
    "Shallow Fog" => val = Conditions::Fog,
    "Mist" => val = Conditions::Fog,
    "Drifting Snow" => val = Conditions::LightSnow,
    "Ice Crystals" => val = Conditions::LightSnow,
    "Snow Pellets" => val = Conditions::LightFreezingRain,
    "Ice Pellets" => val = Conditions::LightFreezingRain,
    "Hail" => val = Conditions::Hail,
    "Snow Grains" => val = Conditions::LightSnow,
    "Light Drizzle" => val = Conditions::Drizzle,
    "Heavy Drizzle" => val = Conditions::Drizzle,
    "Drizzle" => val = Conditions::Drizzle,
    "Clear" => val = Conditions::Clear,
    "Mainly Clear" => val = Conditions::MainlyClear,
    _ => val = Conditions::Unknown
  }

  return val;

}

pub fn parse(xml: &str) -> Result<CurrentConditions, ()> {

  let mut currentConditions = CurrentConditions::init();

  let mut reader = Reader::from_str(xml);
  let mut buf = Vec::new();

  let mut inCurrentConditions = false;
  let mut inTemp = false;
  let mut inDewpoint = false;
  let mut inPressure = false;
  let mut inWind = false;
  let mut inGusts = false;
  let mut inSpeed = false;
  let mut inWindDirection = false;
  let mut inCondition = false;

  loop {
    match reader.read_event(&mut buf) {
      Ok(Event::Start(ref e)) => {
        match e.name() {
          b"currentConditions" => inCurrentConditions = true,
          b"temperature" => inTemp = true,
          b"pressure" => inPressure = true,
          b"wind" => inWind = true,
          b"speed" => inSpeed = true,
          b"direction" => inWindDirection = true,
          b"dewpoint" => inDewpoint = true,
          b"condition" => inCondition = true,
          _ => ()
        }
      },
      Ok(Event::Text(e)) => {
        if (inCurrentConditions) {
          let sTemp = e.unescape_and_decode(&reader).unwrap().clone();
          if (inTemp) {
            currentConditions.temperature = sTemp.parse::<f32>().unwrap();
          } else if (inPressure) {
            currentConditions.pressure = sTemp.parse::<f32>().unwrap();
          } else if (inDewpoint) {
            currentConditions.dewpoint = sTemp.parse::<f32>().unwrap();
          } else if (inWind && inSpeed) {
            currentConditions.wind_speed = sTemp.parse::<f32>().unwrap();
          } else if (inWind && inGusts) {
            currentConditions.wind_gusts = sTemp.parse::<f32>().unwrap();
          } else if (inWind && inWindDirection) {
            currentConditions.wind_direction = sTemp.clone();
          } else if (inCondition) {
            currentConditions.conditions = condition_mapper(sTemp.as_ref());
          }
        }
      },
      Ok(Event::End(ref e)) => {
        match e.name() {
          b"currentConditions" => inCurrentConditions = false,
          b"temperature" => inTemp = false,
          b"pressure" => inPressure = false,
          b"wind" => inWind = false,
          b"speed" => inSpeed = false,
          b"direction" => inWindDirection = false,
          b"dewpoint" => inDewpoint = false,
          b"condition" => inCondition = false,
          _ => ()
        }
      },
      Ok(Event::Eof) => break,
      Err(e) => panic!("Error at position: {}: {:?}", reader.buffer_position(), e),
      _ => ()
    }
    buf.clear();
  }

  Ok(currentConditions)
}

pub fn get_data(province: &str, cityCode: &str) -> Result<String, ()> {

  //https://dd.weather.gc.ca/citypage_weather/xml/QC/s0000635_e.xml
  let mut url = String::from("https://dd.weather.gc.ca/citypage_weather/xml/");
  url.push_str(province);
  url.push_str("/");
  url.push_str(cityCode);
  url.push_str("_e.xml");

  let resp = reqwest::blocking::get(&url).unwrap();

  let xml = resp.text().unwrap();

  Ok(xml)

}