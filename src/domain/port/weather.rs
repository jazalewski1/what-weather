use crate::domain::types::{WeatherQuery, WeatherReport};

#[mockall::automock]
pub trait WeatherProvider {
    fn fetch(&self, query: &WeatherQuery) -> WeatherReport;
}
