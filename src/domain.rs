mod common_forecast;
mod common_format;
mod current_list;
mod current_summary;
mod daily_forecast_summary;
mod today_forecast_summary;

pub mod strategies {
    pub use super::current_list::CurrentList;
    pub use super::current_summary::CurrentSummary;
    pub use super::daily_forecast_summary::DailyForecastSummary;
    pub use super::today_forecast_summary::TodayForecastSummary;
}

use crate::types::units::Coordinates;
use mockall::automock;

#[automock(type Report = String;)]
pub trait ReportStrategy {
    type Report;

    fn fetch(&self, coordinates: &Coordinates) -> Self::Report;

    fn format(&self, report: &Self::Report) -> String;
}
