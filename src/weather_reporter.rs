use crate::output::View;
use crate::output::format::{list, summary};
use crate::port::geolocation::GeolocationProvider;
use crate::port::weather::*;
use crate::types::attributes::*;
use crate::types::report::*;
use crate::types::units::Coordinates;

#[derive(Debug, Clone, PartialEq)]
pub enum ReportType {
    Summary,
    List(WeatherAttributeSet),
}

pub struct Parameters {
    pub report_type: ReportType,
    pub coordinates: Option<Coordinates>,
}

pub struct WeatherReporter<GP: GeolocationProvider, WP: WeatherProvider, V: View> {
    geolocation_provider: GP,
    weather_provider: WP,
    view: V,
}

impl<GP: GeolocationProvider, WP: WeatherProvider, V: View> WeatherReporter<GP, WP, V> {
    pub fn new(geolocation_provider: GP, weather_provider: WP, view: V) -> Self {
        Self {
            geolocation_provider,
            weather_provider,
            view,
        }
    }

    pub fn run(&self, parameters: Parameters) {
        let coordinates = if let Some(coords) = parameters.coordinates {
            coords
        } else {
            self.geolocation_provider.get_current_coordinates()
        };
        let string: String = match parameters.report_type {
            ReportType::Summary => {
                let request = FullRequest { coordinates };
                let response = self.weather_provider.fetch_all(&request);
                let report = FullReport { response };
                summary::describe(&report)
            }
            ReportType::List(attributes) => {
                let request = PartialRequest {
                    coordinates,
                    attributes,
                };
                let response = self.weather_provider.fetch_selected(&request);
                let report = PartialReport {
                    coordinates,
                    response,
                };
                list::describe(&report)
            }
        };
        self.view.display(&string);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::MockView;
    use crate::port::mocks::*;
    use crate::types::units::*;
    use crate::types::weather::*;
    use mockall::predicate::eq;

    fn make_full_response() -> FullResponse {
        FullResponse {
            kind: Kind::Clouds(Clouds::Light),
            temperature: Temperature::new_celsius(24.7),
            cloud_coverage: Percentage::from(47),
            humidity: Percentage::from(60),
            wind: Wind {
                speed: Speed::new_meters_per_second(2.35),
                direction: Azimuth::from(225.3),
            },
            pressure: Hectopascal::from(1001.5),
        }
    }

    fn make_coordinates() -> Coordinates {
        Coordinates::new(1.2, 3.4)
    }

    #[test]
    fn fetches_coordinates_when_not_provided() {
        let parameters = Parameters {
            report_type: ReportType::Summary,
            coordinates: None,
        };

        let mut geolocation_provider = MockGeolocationProvider::new();
        let coordinates = make_coordinates();
        geolocation_provider
            .expect_get_current_coordinates()
            .times(1)
            .return_const(coordinates.clone());

        let mut weather_provider = MockWeatherProvider::new();
        let response = make_full_response();
        weather_provider.expect_fetch_all().return_const(response);

        let mut view = MockView::new();
        view.expect_display().times(1).return_const(());

        let reporter = WeatherReporter::new(geolocation_provider, weather_provider, view);
        reporter.run(parameters);
    }

    #[test]
    fn uses_provided_coordinates() {
        let coordinates = make_coordinates();
        let parameters = Parameters {
            report_type: ReportType::Summary,
            coordinates: Some(coordinates.clone()),
        };

        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider
            .expect_get_current_coordinates()
            .never();

        let mut weather_provider = MockWeatherProvider::new();
        let response = make_full_response();
        weather_provider.expect_fetch_all().return_const(response);

        let mut view = MockView::new();
        view.expect_display().times(1).return_const(());

        let reporter = WeatherReporter::new(geolocation_provider, weather_provider, view);
        reporter.run(parameters);
    }

    #[test]
    fn fetches_full_response_for_summary_report_type() {
        let coordinates = make_coordinates();
        let parameters = Parameters {
            report_type: ReportType::Summary,
            coordinates: Some(coordinates.clone()),
        };

        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider
            .expect_get_current_coordinates()
            .never();

        let mut weather_provider = MockWeatherProvider::new();
        let response = make_full_response();
        weather_provider
            .expect_fetch_all()
            .with(eq(FullRequest { coordinates }))
            .times(1)
            .return_const(response);

        let mut view = MockView::new();
        view.expect_display().times(1).return_const(());

        let reporter = WeatherReporter::new(geolocation_provider, weather_provider, view);
        reporter.run(parameters);
    }

    #[test]
    fn fetches_partial_response_for_list_report_type() {
        let weather_attributes = WeatherAttributeSet::from([
            WeatherAttribute::Temperature,
            WeatherAttribute::Humidity,
            WeatherAttribute::Pressure,
        ]);
        let coordinates = make_coordinates();
        let parameters = Parameters {
            report_type: ReportType::List(weather_attributes.clone()),
            coordinates: Some(coordinates.clone()),
        };

        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider
            .expect_get_current_coordinates()
            .never();

        let mut weather_provider = MockWeatherProvider::new();
        let request = PartialRequest {
            coordinates,
            attributes: weather_attributes.clone(),
        };
        let response = PartialResponse {
            temperature: Some(Temperature::new_celsius(36.6)),
            kind: None,
            cloud_coverage: None,
            humidity: Some(Percentage::from(27)),
            wind: None,
            pressure: Some(Hectopascal::from(1001.2)),
        };
        weather_provider
            .expect_fetch_selected()
            .with(eq(request))
            .times(1)
            .return_const(response);

        let mut view = MockView::new();
        view.expect_display().times(1).return_const(());

        let reporter = WeatherReporter::new(geolocation_provider, weather_provider, view);
        reporter.run(parameters);
    }
}
