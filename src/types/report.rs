use crate::types::units::*;
use crate::types::weather::*;

#[derive(Clone, Debug)]
pub struct CurrentFullReport {
    pub kind: Kind,
    pub temperature: Temperature,
    pub cloud_coverage: Percentage,
    pub humidity: Percentage,
    pub wind: Wind,
    pub pressure: Hectopascal,
}

#[derive(Clone, Debug)]
pub struct CurrentPartialReport {
    pub coordinates: Coordinates,
    pub kind: Option<Kind>,
    pub temperature: Option<Temperature>,
    pub cloud_coverage: Option<Percentage>,
    pub humidity: Option<Percentage>,
    pub wind: Option<Wind>,
    pub pressure: Option<Hectopascal>,
}

impl CurrentPartialReport {
    pub fn new_empty(coordinates: Coordinates) -> Self {
        Self {
            coordinates,
            kind: None,
            temperature: None,
            cloud_coverage: None,
            humidity: None,
            wind: None,
            pressure: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ForecastFullReport {
    pub kind: Kind,
    pub temperature_range: TemperatureRange,
    pub cloud_coverage_range: PercentageRange,
}
