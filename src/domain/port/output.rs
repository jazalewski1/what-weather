use crate::domain::types::WeatherReport;

pub trait Presenter {
    fn display(&self, report: &WeatherReport);
}
