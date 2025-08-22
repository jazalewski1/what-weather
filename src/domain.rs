pub mod current_list;
pub mod current_summary;
pub mod forecast_summary;

mod common_format;

use crate::types::units::Coordinates;
use mockall::automock;

#[automock(type Report = String;)]
pub trait ReportStrategy {
    type Report;

    fn fetch(&self, coordinates: &Coordinates) -> Self::Report;

    fn format(&self, report: &Self::Report) -> String;
}
