use crate::port::geolocation::GeolocationProvider;
use crate::port::weather::{ReportRequest, RequestKind, WeatherProvider};
use crate::types::units::Coordinates;
use crate::types::report::*;

pub struct Parameters {
    pub coordinates: Option<Coordinates>,
    pub request_kind: RequestKind,
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

    pub fn run(&self, parameters: Parameters) -> Report {
        let coordinates = if let Some(coords) = parameters.coordinates {
            coords
        } else {
            self.geolocation_provider.get_current_coordinates()
        };
        let request = ReportRequest {
            coordinates,
            kind: parameters.request_kind,
        };
        self.weather_provider.fetch(&request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::{MockGeolocationProvider, MockWeatherProvider};
    use crate::types::units::*;
    use crate::types::weather::*;
    use crate::port::weather::ReportRequest;

    fn make_dummy_report() -> Report {
        Report::CurrentFull(CurrentFullReport{
            kind: Kind::Thunderstorm,
            temperature: Temperature::new_celsius(23.4),
            cloud_coverage: Percentage::from(50),
            humidity: Percentage::from(60),
            wind: Wind { speed: Speed::new_meters_per_second(1.23), direction: Azimuth::from(90.0) },
            pressure: Pressure::new_hpa(1001.23),
        })
    }

    #[test]
    fn fetches_coordinates_if_not_provided() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        geolocation_provider
            .expect_get_current_coordinates()
            .once()
            .return_const(coordinates);

        let mut weather_provider = MockWeatherProvider::new();
        let matching_coordinates = move |request: &ReportRequest| request.coordinates == coordinates;
        weather_provider.expect_fetch().withf(matching_coordinates).return_const(make_dummy_report());

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let parameters = Parameters { coordinates: None, request_kind: RequestKind::CurrentFull };
        sut.run(parameters);
    }

    #[test]
    fn uses_provided_coordinates() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider
            .expect_get_current_coordinates()
            .never();

        let mut weather_provider = MockWeatherProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        let matching_coordinates = move |request: &ReportRequest| request.coordinates == coordinates;
        weather_provider.expect_fetch().withf(matching_coordinates).return_const(make_dummy_report());

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let parameters = Parameters { coordinates: Some(coordinates), request_kind: RequestKind::CurrentFull };
        sut.run(parameters);
    }

    #[test]
    fn returns_fetched_report() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider
            .expect_get_current_coordinates()
            .never();

        let mut weather_provider = MockWeatherProvider::new();
        let report = make_dummy_report();
        weather_provider.expect_fetch().return_const(report.clone());

        let sut = WeatherReporter::new(geolocation_provider, weather_provider);
        let coordinates = Coordinates::new(1.23, 45.67);
        let parameters = Parameters { coordinates: Some(coordinates), request_kind: RequestKind::CurrentFull };
        let actual_report = sut.run(parameters);
        assert_eq!(report, actual_report);
    }
}
