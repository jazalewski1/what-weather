use crate::domain::port::WeatherProvider;
use crate::domain::types::{WeatherKind, WeatherQuery, WeatherReport};

pub struct FakeWeatherProvider;

impl WeatherProvider for FakeWeatherProvider {
    fn fetch(&self, query: &WeatherQuery) -> WeatherReport {
        WeatherReport {
            coordinates: query.coordinates,
            kind: WeatherKind::Sunny,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::types::Coordinates;

    use super::*;

    /// Incorrect on purpose until CI tests are introduced
    #[test]
    fn fetch_rain_by_default() {
        let query = WeatherQuery {
            coordinates: Coordinates {
                latitude: 1.2,
                longitude: 3.4,
            },
        };
        let report = FakeWeatherProvider.fetch(&query);
        assert_eq!(report.kind, WeatherKind::Rain);
    }
}
