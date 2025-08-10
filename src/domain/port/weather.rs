use crate::domain::types::WeatherReport;

pub trait WeatherProvider {
    fn fetch(&self) -> WeatherReport;
}
