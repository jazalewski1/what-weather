use crate::domain::ReportStrategy;
use crate::port::geolocation::GeolocationProvider;
use crate::types::units::Coordinates;

pub struct Parameters {
    pub coordinates: Option<Coordinates>,
}

pub struct WeatherReporter<GP: GeolocationProvider> {
    geolocation_provider: GP,
}

impl<GP: GeolocationProvider> WeatherReporter<GP> {
    pub fn new(geolocation_provider: GP) -> Self {
        Self {
            geolocation_provider,
        }
    }

    pub fn run(&self, report_strategy: impl ReportStrategy, parameters: Parameters) -> String {
        let coordinates = if let Some(coords) = parameters.coordinates {
            coords
        } else {
            self.geolocation_provider.get_current_coordinates()
        };
        let report = report_strategy.fetch(&coordinates);
        report_strategy.format(&report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::MockReportStrategy;
    use crate::port::mocks::MockGeolocationProvider;
    use mockall::predicate::eq;

    #[test]
    fn fetches_coordinates_if_not_provided() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        geolocation_provider
            .expect_get_current_coordinates()
            .once()
            .return_const(coordinates);
        let sut = WeatherReporter::new(geolocation_provider);

        let mut strategy = MockReportStrategy::new();
        let report = String::from("report");
        strategy
            .expect_fetch()
            .with(eq(coordinates))
            .return_const(report.clone());
        strategy.expect_format().return_const(report.clone());
        let parameters = Parameters { coordinates: None };
        sut.run(strategy, parameters);
    }

    #[test]
    fn uses_provided_coordinates() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider
            .expect_get_current_coordinates()
            .never();
        let sut = WeatherReporter::new(geolocation_provider);

        let mut strategy = MockReportStrategy::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        strategy
            .expect_fetch()
            .once()
            .with(eq(coordinates))
            .return_const("report".to_string());
        strategy
            .expect_format()
            .once()
            .return_const("REPORT".to_string());
        let parameters = Parameters {
            coordinates: Some(coordinates),
        };
        sut.run(strategy, parameters);
    }

    #[test]
    fn formats_fetched_report() {
        let geolocation_provider = MockGeolocationProvider::new();
        let sut = WeatherReporter::new(geolocation_provider);

        let mut strategy = MockReportStrategy::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        strategy.expect_fetch().return_const("report");
        strategy
            .expect_format()
            .once()
            .with(eq("report".to_string()))
            .return_const("REPORT".to_string());
        let parameters = Parameters {
            coordinates: Some(coordinates),
        };
        sut.run(strategy, parameters);
    }

    #[test]
    fn returns_formatted_value() {
        let geolocation_provider = MockGeolocationProvider::new();
        let sut = WeatherReporter::new(geolocation_provider);

        let mut strategy = MockReportStrategy::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        strategy.expect_fetch().return_const("report");
        strategy
            .expect_format()
            .once()
            .with(eq("report".to_string()))
            .return_const("REPORT".to_string());
        let parameters = Parameters {
            coordinates: Some(coordinates),
        };
        let result = sut.run(strategy, parameters);
        assert_eq!(result, "REPORT");
    }
}
