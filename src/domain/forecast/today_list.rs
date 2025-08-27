use crate::domain::ReportStrategy;
use crate::domain::common::list_builder::write_param;
use crate::domain::forecast::common_list::write_spec;
use crate::port::weather::WeatherProvider;
use crate::types::attributes::*;
use crate::types::report::*;
use crate::types::units::*;

pub struct TodayForecastList<P: WeatherProvider> {
    weather_provider: P,
    attributes: WeatherAttributeSet,
}

impl<P: WeatherProvider> TodayForecastList<P> {
    pub fn new(weather_provider: P, attributes: WeatherAttributeSet) -> Self {
        Self {
            weather_provider,
            attributes,
        }
    }
}

impl<P: WeatherProvider> ReportStrategy for TodayForecastList<P> {
    type Report = TodayForecastPartialReport;

    fn fetch(&self, coordinates: &Coordinates) -> Self::Report {
        self.weather_provider
            .fetch_today_forecast_partial_report(coordinates, &self.attributes)
    }

    fn format(&self, report: &Self::Report) -> String {
        let Self::Report { coordinates, spec } = &report;
        let mut result = String::default();
        write_param(&mut result, "Coordinates", format!("{coordinates:.5}"));
        write_spec(&mut result, spec);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::MockWeatherProvider;
    use crate::types::weather::*;
    use mockall::predicate::eq;

    #[test]
    fn fetches_report_for_specified_attributes() {
        let attributes = WeatherAttributeSet::from([
            WeatherAttribute::Temperature,
            WeatherAttribute::Wind,
            WeatherAttribute::WeatherKind,
        ]);
        let mut weather_provider = MockWeatherProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        let spec = ForecastPartialSpec {
            kind: Some(Kind::Clouds(Clouds::Moderate)),
            temperature_range: Some(TemperatureRange::new_celsius(24.5, 27.1)),
            cloud_coverage_range: None,
            humidity_range: Some(PercentageRange::new(33, 46)),
            wind: None,
            pressure_range: None,
        };
        let report = TodayForecastPartialReport { coordinates, spec };
        weather_provider
            .expect_fetch_today_forecast_partial_report()
            .once()
            .with(eq(coordinates), eq(attributes.clone()))
            .return_const(report);

        let sut = TodayForecastList::new(weather_provider, attributes);
        sut.fetch(&coordinates);
    }

    #[test]
    fn formats_report() {
        let attributes = WeatherAttributeSet::from([
            WeatherAttribute::Temperature,
            WeatherAttribute::Wind,
            WeatherAttribute::WeatherKind,
        ]);
        let sut = TodayForecastList::new(MockWeatherProvider::new(), attributes);
        let coordinates = Coordinates::new(1.23, 45.67);
        let spec = ForecastPartialSpec {
            kind: Some(Kind::Clouds(Clouds::Moderate)),
            temperature_range: Some(TemperatureRange::new_celsius(24.5, 27.1)),
            cloud_coverage_range: None,
            humidity_range: Some(PercentageRange::new(33, 46)),
            wind: None,
            pressure_range: None,
        };
        let report = TodayForecastPartialReport { coordinates, spec };
        let result = sut.format(&report);
        let expected = "Coordinates: 1.23000°, 45.67000°\n\
                        Weather: cloudy\n\
                        Temperature: 24.5°C - 27.1°C\n\
                        Humidity: 33% - 46%\n";
        assert_eq!(result, expected);
    }
}
