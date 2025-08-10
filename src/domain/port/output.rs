use crate::domain::types::WeatherReport;

#[mockall::automock]
pub trait Presenter {
    fn display(&self, report: &WeatherReport);
}
