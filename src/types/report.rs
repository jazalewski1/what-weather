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
pub struct TodayForecastFullReport {
    pub kind: Kind,
    pub temperature_range: TemperatureRange,
    pub cloud_coverage_range: PercentageRange,
    pub humidity_range: PercentageRange,
    pub wind: WindScope,
    pub pressure_range: PressureRange,
}

#[derive(Clone, Debug)]
pub struct DailyFullData {
    pub date: Date,
    pub kind: Kind,
    pub temperature_range: TemperatureRange,
    pub cloud_coverage_range: PercentageRange,
    pub humidity_range: PercentageRange,
    pub wind: WindScope,
    pub pressure_range: PressureRange,
}

#[derive(Clone, Debug)]
pub struct DailyForecastFullReport {
    pub data: Vec<DailyFullData>,
}

#[derive(Clone, Debug)]
pub struct TodayForecastPartialReport {
    pub coordinates: Coordinates,
    pub kind: Option<Kind>,
    pub temperature_range: Option<TemperatureRange>,
    pub cloud_coverage_range: Option<PercentageRange>,
    pub humidity_range: Option<PercentageRange>,
    pub wind: Option<WindScope>,
    pub pressure_range: Option<PressureRange>,
}
