use crate::types::query::*;
use crate::types::report::*;

#[mockall::automock]
pub trait WeatherProvider {
    fn fetch_all(&self, query: &FullQuery) -> FullReport;

    fn fetch_selected(&self, query: &PartialQuery) -> PartialReport;
}
