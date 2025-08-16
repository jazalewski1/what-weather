use std::fmt::Display;

use crate::types::units::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Clouds {
    Clear,
    Light,
    Moderate,
    Dense,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Fog {
    Normal,
    Rime,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrecipitationKind {
    Rain,
    Snow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrecipitationIntensity {
    Light,
    Moderate,
    Heavy,
    Shower,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrecipitationHeat {
    Freezing,
    Normal,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Precipitation {
    pub kind: PrecipitationKind,
    pub intensity: PrecipitationIntensity,
    pub heat: PrecipitationHeat,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Clouds(Clouds),
    Fog(Fog),
    Precipitation(Precipitation),
    Thunderstorm,
}

#[derive(Debug, Clone, Copy)]
pub enum Temperature {
    Celsius(Celsius),
}

impl Temperature {
    pub fn new_celsius(value: f32) -> Self {
        Self::Celsius(Celsius { value })
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(1);
        match self {
            Self::Celsius(celsius) => {
                write!(f, "{celsius:.precision$}")
            }
        }
    }
}

pub type CloudCoverage = i8;

pub type Humidity = i8;

#[derive(Clone, Debug)]
pub struct Wind {
    pub speed: f32,
    pub direction: f32,
}

pub type Pressure = f32;
