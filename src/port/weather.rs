use crate::types::attributes::WeatherAttributeSet;
use crate::types::error::FetchError;
use crate::types::report::*;
use crate::types::units::*;

#[derive(Debug, PartialEq, Eq)]
pub enum RequestKind {
    PastFull(DayCount),
    PastPartial(DayCount, WeatherAttributeSet),
    CurrentFull,
    CurrentPartial(WeatherAttributeSet),
    ForecastFull(DayCount),
    ForecastPartial(DayCount, WeatherAttributeSet),
}

#[derive(Debug, PartialEq)]
pub struct ReportRequest {
    pub coordinates: Coordinates,
    pub kind: RequestKind,
}

#[mockall::automock]
pub trait WeatherProvider {
    fn fetch(&self, request: &ReportRequest) -> Result<Report, FetchError>;
}
