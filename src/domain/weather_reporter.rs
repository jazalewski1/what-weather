use crate::port::{GeolocationProvider, WeatherProvider};
use crate::types::query::*;
use crate::types::report::*;

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

    pub fn fetch_all(&self) -> FullReport {
        let coordinates = self.geolocation_provider.get_current_coordinates();
        let query = FullQuery { coordinates };
        self.weather_provider.fetch_all(&query)
    }

    pub fn fetch_selected(&self, attributes: &WeatherAttributeSet) -> PartialReport {
        let coordinates = self.geolocation_provider.get_current_coordinates();
        let query = PartialQuery {
            coordinates,
            attributes: attributes.clone(),
        };
        self.weather_provider.fetch_selected(&query)
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::port::mocks::*;
    use crate::types::units::*;
    use crate::types::weather::*;

    #[test]
    fn fetch_coordinates_and_all_attributes() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        let coordinates = Coordinates {
            latitude: 1.2,
            longitude: 3.4,
        };
        geolocation_provider
            .expect_get_current_coordinates()
            .times(1)
            .return_const(coordinates.clone());

        let mut weather_provider = MockWeatherProvider::new();
        let report = FullReport {
            kind: Kind::Clouds(Clouds::Light),
            temperature: Temperature::new_celsius(24.7),
            cloud_coverage: Percentage::from(47),
            humidity: Percentage::from(60),
            wind: Wind {
                speed: Speed::new_meters_per_second(2.35),
                direction: Azimuth::from(225.3),
            },
            pressure: Hectopascal::from(1001.5),
        };
        weather_provider
            .expect_fetch_all()
            .with(eq(FullQuery { coordinates }))
            .times(1)
            .return_const(report);
        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let _report = sut.fetch_all();
    }

    #[test]
    fn fetch_coordinates_and_selected_attributes() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        let coordinates = Coordinates {
            latitude: 1.2,
            longitude: 3.4,
        };
        geolocation_provider
            .expect_get_current_coordinates()
            .times(1)
            .return_const(coordinates.clone());

        let mut weather_provider = MockWeatherProvider::new();
        let requested_attributes = WeatherAttributeSet::from([
            WeatherAttribute::WeatherKind,
            WeatherAttribute::Temperature,
            WeatherAttribute::Pressure,
            WeatherAttribute::Humidity,
        ]);
        let query = PartialQuery {
            coordinates,
            attributes: requested_attributes.clone(),
        };
        let report = PartialReport {
            kind: Some(Kind::Clouds(Clouds::Light)),
            temperature: Some(Temperature::new_celsius(24.7)),
            cloud_coverage: None,
            humidity: Some(Percentage::from(60)),
            wind: None,
            pressure: Some(Hectopascal::from(1001.5)),
        };
        weather_provider
            .expect_fetch_selected()
            .with(eq(query))
            .times(1)
            .return_const(report);

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let _report = sut.fetch_selected(&requested_attributes);
    }
}
