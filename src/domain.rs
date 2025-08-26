mod common;
mod common_format;
mod current;
mod forecast;

pub mod strategies {
    pub use super::current::{list::CurrentList, summary::CurrentSummary};
    pub use super::forecast::{
        daily_list::DailyForecastList, daily_summary::DailyForecastSummary,
        today_list::TodayForecastList, today_summary::TodayForecastSummary,
    };
}

use crate::types::units::Coordinates;
use mockall::automock;

#[automock(type Report = String;)]
pub trait ReportStrategy {
    type Report;

    fn fetch(&self, coordinates: &Coordinates) -> Self::Report;

    fn format(&self, report: &Self::Report) -> String;
}
