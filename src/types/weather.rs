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

#[derive(Clone, Debug)]
pub struct Wind {
    pub speed: Speed,
    pub direction: Azimuth,
}
