use crate::types::Coordinates;
use crate::types::units::*;
use crate::types::weather::*;

#[derive(Clone, Debug)]
pub struct WeatherReport {
    pub coordinates: Coordinates,
    pub kind: Kind,
    pub temperature: Temperature,
    pub cloud_coverage: Percentage,
    pub humidity: Percentage,
    pub wind: Wind,
    pub pressure: Pressure,
}
