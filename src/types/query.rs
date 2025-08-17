use crate::types::units::Coordinates;
use std::collections::HashSet;
use strum::EnumIter;

#[derive(Clone, Debug, PartialEq)]
pub struct FullQuery {
    pub coordinates: Coordinates,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartialQuery {
    pub coordinates: Coordinates,
    pub attributes: WeatherAttributeSet,
}

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
