use crate::port::geolocation::GeolocationProvider;
use crate::port::weather::*;
use crate::types::attributes::*;
use crate::types::report::*;
use crate::types::units::Coordinates;

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

    pub fn fetch_all(&self, coordinates: &Option<Coordinates>) -> FullReport {
        let coordinates = self.fetch_coordinates(coordinates);
        let request = FullRequest { coordinates };
        let response = self.weather_provider.fetch_all(&request);
        FullReport { response }
    }

    pub fn fetch_selected(
        &self,
        attributes: &WeatherAttributeSet,
        coordinates: &Option<Coordinates>,
    ) -> PartialReport {
        let coordinates = self.fetch_coordinates(coordinates);
        let request = PartialRequest {
            coordinates,
            attributes: attributes.clone(),
        };
        let response = self.weather_provider.fetch_selected(&request);
        PartialReport {
            coordinates,
            response,
        }
    }

    fn fetch_coordinates(&self, coordinates: &Option<Coordinates>) -> Coordinates {
        if let Some(coords) = coordinates {
            *coords
        } else {
            self.geolocation_provider.get_current_coordinates()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::*;
    use crate::types::units::*;
    use crate::types::weather::*;
    use mockall::predicate::eq;

    #[test]
    fn fetches_coordinates_and_all_attributes() {
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
        let report = FullResponse {
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
            .with(eq(FullRequest { coordinates }))
            .times(1)
            .return_const(report);
        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let _report = sut.fetch_all(&None);
    }

    #[test]
    fn fetches_coordinates_and_selected_attributes() {
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
        let request = PartialRequest {
            coordinates,
            attributes: requested_attributes.clone(),
        };
        let report = PartialResponse {
            kind: Some(Kind::Clouds(Clouds::Light)),
            temperature: Some(Temperature::new_celsius(24.7)),
            cloud_coverage: None,
            humidity: Some(Percentage::from(60)),
            wind: None,
            pressure: Some(Hectopascal::from(1001.5)),
        };
        weather_provider
            .expect_fetch_selected()
            .with(eq(request))
            .times(1)
            .return_const(report);

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let _report = sut.fetch_selected(&requested_attributes, &None);
    }

    #[test]
    fn uses_specified_coordinates_to_fetch_all_attributes() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider
            .expect_get_current_coordinates()
            .never();

        let mut weather_provider = MockWeatherProvider::new();
        let report = FullResponse {
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
        let coordinates = Coordinates {
            latitude: 1.2,
            longitude: 3.4,
        };
        let request = FullRequest { coordinates };
        weather_provider
            .expect_fetch_all()
            .with(eq(request))
            .times(1)
            .return_const(report);

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);

        let _report = sut.fetch_all(&Some(coordinates));
    }

    #[test]
    fn uses_specified_coordinates_to_fetch_selected_attributes() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider
            .expect_get_current_coordinates()
            .never();

        let mut weather_provider = MockWeatherProvider::new();
        let coordinates = Coordinates {
            latitude: 1.2,
            longitude: 3.4,
        };
        let requested_attributes = WeatherAttributeSet::from([
            WeatherAttribute::WeatherKind,
            WeatherAttribute::Temperature,
            WeatherAttribute::Pressure,
            WeatherAttribute::Humidity,
        ]);
        let request = PartialRequest {
            coordinates,
            attributes: requested_attributes.clone(),
        };
        let report = PartialResponse {
            kind: Some(Kind::Clouds(Clouds::Light)),
            temperature: Some(Temperature::new_celsius(24.7)),
            cloud_coverage: None,
            humidity: Some(Percentage::from(60)),
            wind: None,
            pressure: Some(Hectopascal::from(1001.5)),
        };
        weather_provider
            .expect_fetch_selected()
            .with(eq(request))
            .times(1)
            .return_const(report);

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let _report = sut.fetch_selected(&requested_attributes, &Some(coordinates));
    }
}
