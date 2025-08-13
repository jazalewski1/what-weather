use crate::domain::types::Coordinates;
use crate::domain::types::weather::*;

#[derive(Clone, Debug)]
pub struct WeatherReport {
    pub coordinates: Coordinates,
    pub kind: WeatherKind,
    pub temperature: Temperature,
    pub cloud_coverage: CloudCoverage,
    pub humidity: Humidity,
    pub wind: Wind,
    pub pressure: Pressure,
}
