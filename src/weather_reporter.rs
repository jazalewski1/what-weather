use crate::port::geolocation::GeolocationProvider;
use crate::port::weather::{ReportRequest, RequestKind, WeatherProvider};
use crate::types::report::*;
use crate::types::units::Coordinates;

pub struct Parameters {
    pub coordinates: Option<Coordinates>,
    pub request_kind: RequestKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    FetchingCoordinates,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::FetchingCoordinates => "Failed to fetch current coordinates",
        };
        write!(f, "{message}")
    }
}

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

    pub fn run(&self, parameters: Parameters) -> Result<Report, Error> {
        let coordinates = self.get_coordinates(parameters.coordinates)?;
        let request = ReportRequest {
            coordinates,
            kind: parameters.request_kind,
        };
        let report = self.weather_provider.fetch(&request);
        Ok(report)
    }

    fn get_coordinates(&self, coordinates: Option<Coordinates>) -> Result<Coordinates, Error> {
        if let Some(coords) = coordinates {
            return Ok(coords);
        }
        const MAX_NUMBER_OF_ATTEMPTS: usize = 3;
        for _ in 0..MAX_NUMBER_OF_ATTEMPTS {
            match self.geolocation_provider.fetch() {
                Ok(coords) => return Ok(coords),
                Err(error) => println!("Error: {error}"),
            }
        }
        Err(Error::FetchingCoordinates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::{MockGeolocationProvider, MockWeatherProvider};
    use crate::port::weather::ReportRequest;
    use crate::types::error::FetchError;
    use crate::types::units::*;
    use crate::types::weather::*;

    fn make_dummy_report() -> Report {
        Report::CurrentFull(CurrentFullReport {
            kind: Kind::Thunderstorm,
            temperature: Temperature::new_celsius(23.4),
            cloud_coverage: Percentage::from(50),
            humidity: Percentage::from(60),
            wind: Wind {
                speed: Speed::new_meters_per_second(1.23),
                direction: Azimuth::from(90.0),
            },
            pressure: Pressure::new_hpa(1001.23),
        })
    }

    #[test]
    fn fetches_coordinates_if_not_provided() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        geolocation_provider
            .expect_fetch()
            .once()
            .return_const(Ok(coordinates));

        let mut weather_provider = MockWeatherProvider::new();
        let matching_coordinates =
            move |request: &ReportRequest| request.coordinates == coordinates;
        weather_provider
            .expect_fetch()
            .withf(matching_coordinates)
            .return_const(make_dummy_report());

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let parameters = Parameters {
            coordinates: None,
            request_kind: RequestKind::CurrentFull,
        };
        let result = sut.run(parameters);
        assert!(result.is_ok());
    }

    #[test]
    fn fetches_coordinates_after_failure() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        geolocation_provider
            .expect_fetch()
            .times(2)
            .return_const(Err(FetchError::ConnectionFailure));
        geolocation_provider
            .expect_fetch()
            .once()
            .return_const(Ok(coordinates));

        let mut weather_provider = MockWeatherProvider::new();
        let matching_coordinates =
            move |request: &ReportRequest| request.coordinates == coordinates;
        weather_provider
            .expect_fetch()
            .withf(matching_coordinates)
            .return_const(make_dummy_report());

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let parameters = Parameters {
            coordinates: None,
            request_kind: RequestKind::CurrentFull,
        };
        let result = sut.run(parameters);
        assert!(result.is_ok());
    }

    #[test]
    fn retries_to_fetch_coordinates_and_fails() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        geolocation_provider
            .expect_fetch()
            .times(3)
            .return_const(Err(FetchError::ConnectionFailure));

        let mut weather_provider = MockWeatherProvider::new();
        let matching_coordinates =
            move |request: &ReportRequest| request.coordinates == coordinates;
        weather_provider
            .expect_fetch()
            .withf(matching_coordinates)
            .return_const(make_dummy_report());

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let parameters = Parameters {
            coordinates: None,
            request_kind: RequestKind::CurrentFull,
        };
        let result = sut.run(parameters);
        assert!(result.is_err());
    }

    #[test]
    fn uses_provided_coordinates() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider.expect_fetch().never();

        let mut weather_provider = MockWeatherProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        let matching_coordinates =
            move |request: &ReportRequest| request.coordinates == coordinates;
        weather_provider
            .expect_fetch()
            .withf(matching_coordinates)
            .return_const(make_dummy_report());

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let parameters = Parameters {
            coordinates: Some(coordinates),
            request_kind: RequestKind::CurrentFull,
        };
        let result = sut.run(parameters);
        assert!(result.is_ok());
    }

    #[test]
    fn returns_fetched_report() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider.expect_fetch().never();

        let mut weather_provider = MockWeatherProvider::new();
        let report = make_dummy_report();
        weather_provider.expect_fetch().return_const(report.clone());

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let coordinates = Coordinates::new(1.23, 45.67);
        let parameters = Parameters {
            coordinates: Some(coordinates),
            request_kind: RequestKind::CurrentFull,
        };
        let actual_report = sut.run(parameters);
        assert_eq!(actual_report, Ok(report));
    }
}
