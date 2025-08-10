use crate::domain::types::{WeatherQuery, WeatherReport};

pub trait WeatherProvider {
    fn fetch(&self, query: &WeatherQuery) -> WeatherReport;
}
