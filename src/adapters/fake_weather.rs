use crate::domain::port::WeatherProvider;
use crate::domain::types::{WeatherKind, WeatherReport};

pub struct FakeWeatherProvider;

impl WeatherProvider for FakeWeatherProvider {
    fn fetch(&self) -> WeatherReport {
        WeatherReport {
            kind: WeatherKind::Sunny,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_rain_by_default() {
        let expected = WeatherReport { kind: WeatherKind::Rain };
        assert_eq!(FakeWeatherProvider.fetch(), expected);
    }
}
