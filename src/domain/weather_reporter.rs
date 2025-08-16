use crate::port::{GeolocationProvider, WeatherProvider};
use crate::types::{WeatherQuery, WeatherReport};

pub struct WeatherReporter<GP: GeolocationProvider, WP: WeatherProvider> {
    geolocation_provider: GP,
    weather_provider: WP,
}

impl<GP: GeolocationProvider, WP: WeatherProvider> WeatherReporter<GP, WP> {
    pub fn new(geolocation_provider: GP, weather_provider: WP) -> Self {
        Self {
            geolocation_provider,
            weather_provider,
        }
    }

    pub fn fetch(&self) -> WeatherReport {
        let coordinates = self.geolocation_provider.get_current_coordinates();
        let query = WeatherQuery { coordinates };
        self.weather_provider.fetch(&query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::*;
    use crate::types::units::*;
    use crate::types::weather::*;
    use crate::types::{Coordinates, WeatherReport};

    #[test]
    fn fetch_and_display_current_weather_report() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider
            .expect_get_current_coordinates()
            .times(1)
            .returning(|| Coordinates {
                latitude: 1.2,
                longitude: 3.4,
            });

        let mut weather_provider = MockWeatherProvider::new();
        weather_provider
            .expect_fetch()
            .times(1)
            .returning(|_| WeatherReport {
                coordinates: Coordinates {
                    latitude: 1.2,
                    longitude: 3.4,
                },
                kind: Kind::Clouds(Clouds::Light),
                temperature: Temperature::new_celsius(24.7),
                cloud_coverage: Percentage::from(47),
                humidity: Percentage::from(60),
                wind: Wind {
                    speed: Speed::new_meters_per_second(2.35),
                    direction: Azimuth::from(225.3),
                },
                pressure: 1001.5,
            });
        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let _report = sut.fetch();
    }
}
