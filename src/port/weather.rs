use crate::types::attributes::WeatherAttributeSet;
use crate::types::error::FetchError;
use crate::types::report::*;
use crate::types::units::*;

#[derive(Debug, PartialEq, Eq)]
pub enum RequestKind {
    CurrentFull,
    CurrentPartial(WeatherAttributeSet),
    TodayForecastFull,
    TodayForecastPartial(WeatherAttributeSet),
    DailyForecastFull(DayCount),
    DailyForecastPartial(WeatherAttributeSet, DayCount),
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
