use std::collections::HashSet;
use strum::{EnumIter, VariantArray};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, VariantArray)]
pub enum WeatherAttribute {
    WeatherKind,
    Temperature,
    CloudCoverage,
    Humidity,
    Wind,
    Pressure,
}

pub type WeatherAttributeSet = HashSet<WeatherAttribute>;
