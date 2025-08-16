use crate::types::units::*;
use crate::types::weather::*;

#[derive(Clone, Debug)]
pub struct FullReport {
    pub kind: Kind,
    pub temperature: Temperature,
    pub cloud_coverage: Percentage,
    pub humidity: Percentage,
    pub wind: Wind,
    pub pressure: Hectopascal,
}

#[derive(Clone, Debug)]
pub struct PartialReport {
    pub kind: Option<Kind>,
    pub temperature: Option<Temperature>,
    pub cloud_coverage: Option<Percentage>,
    pub humidity: Option<Percentage>,
    pub wind: Option<Wind>,
    pub pressure: Option<Hectopascal>,
}

#[derive(Clone, Debug)]
pub enum WeatherReport {
    Full(FullReport),
    Partial(PartialReport),
}
