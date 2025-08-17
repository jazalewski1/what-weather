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
    pub parameters: WeatherParameterSet,
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, Hash)]
pub enum WeatherParameter {
    WeatherKind,
    Temperature,
    CloudCoverage,
    Humidity,
    Wind,
    Pressure,
}

pub type WeatherParameterSet = HashSet<WeatherParameter>;
