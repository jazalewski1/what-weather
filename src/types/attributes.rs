use std::collections::HashSet;
use strum::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, Hash)]
pub enum WeatherAttribute {
    WeatherKind,
    Temperature,
    CloudCoverage,
    Humidity,
    Wind,
    Pressure,
}

pub type WeatherAttributeSet = HashSet<WeatherAttribute>;
