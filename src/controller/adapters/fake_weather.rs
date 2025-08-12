use crate::domain::port::WeatherProvider;
use crate::domain::types::*;

pub struct FakeWeatherProvider;

impl WeatherProvider for FakeWeatherProvider {
    fn fetch(&self, query: &WeatherQuery) -> WeatherReport {
        WeatherReport {
            coordinates: query.coordinates,
            kind: WeatherKind::Clouds(Clouds::Clear),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::types::Coordinates;

    use super::*;

    #[test]
    fn fetch_clear_sky_by_default() {
        let query = WeatherQuery {
            coordinates: Coordinates {
                latitude: 1.2,
                longitude: 3.4,
            },
        };
        let report = FakeWeatherProvider.fetch(&query);
        assert_eq!(report.kind, WeatherKind::Clouds(Clouds::Clear));
    }
}
