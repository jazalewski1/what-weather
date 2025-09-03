use crate::types::units::*;
use crate::types::weather::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CurrentFullReport {
    pub kind: Kind,
    pub temperature: Temperature,
    pub cloud_coverage: Percentage,
    pub humidity: Percentage,
    pub wind: Wind,
    pub pressure: Pressure,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CurrentPartialReport {
    pub coordinates: Coordinates,
    pub kind: Option<Kind>,
    pub temperature: Option<Temperature>,
    pub cloud_coverage: Option<Percentage>,
    pub humidity: Option<Percentage>,
    pub wind: Option<Wind>,
    pub pressure: Option<Pressure>,
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

#[derive(Clone, Debug, PartialEq)]
pub struct DailyFullData {
    pub date: Date,
    pub kind: Kind,
    pub temperature_range: TemperatureRange,
    pub cloud_coverage_range: PercentageRange,
    pub humidity_range: PercentageRange,
    pub wind: WindScope,
    pub pressure_range: PressureRange,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DailyFullReport {
    pub data: Vec<DailyFullData>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DailyPartialData {
    pub date: Date,
    pub kind: Option<Kind>,
    pub temperature_range: Option<TemperatureRange>,
    pub cloud_coverage_range: Option<PercentageRange>,
    pub humidity_range: Option<PercentageRange>,
    pub wind: Option<WindScope>,
    pub pressure_range: Option<PressureRange>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DailyPartialReport {
    pub coordinates: Coordinates,
    pub data: Vec<DailyPartialData>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Report {
    CurrentFull(CurrentFullReport),
    CurrentPartial(CurrentPartialReport),
    ForecastFull(DailyFullReport),
    ForecastPartial(DailyPartialReport),
}
