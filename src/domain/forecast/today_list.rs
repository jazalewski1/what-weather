use crate::domain::ReportStrategy;
use crate::domain::common::list_builder::ListBuilder;
use crate::domain::common::list_format::describe_kind;
use crate::port::weather::WeatherProvider;
use crate::types::attributes::*;
use crate::types::report::TodayForecastPartialReport;
use crate::types::units::*;
use crate::types::weather::WindScope;

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
        let mut builder = ListBuilder::default();
        builder.add("Coordinates", &format!("{:.5}", report.coordinates));
        if let Some(kind) = report.kind {
            builder.add("Weather", &describe_kind(&kind));
        }
        if let Some(temperature) = &report.temperature_range {
            let value = match temperature {
                TemperatureRange::Celsius { min, max } => format!("{min} - {max}"),
            };
            builder.add("Temperature", &value);
        }
        if let Some(PercentageRange { min, max }) = report.cloud_coverage_range {
            builder.add("Cloud coverage", &format!("{min} - {max}"));
        }
        if let Some(PercentageRange { min, max }) = report.humidity_range {
            builder.add("Humidity", &format!("{min} - {max}"));
        }
        if let Some(WindScope {
            speed_range,
            dominant_direction,
        }) = &report.wind
        {
            let speed_desc = match speed_range {
                SpeedRange::MetersPerSecond { min, max } => format!("{min} - {max}"),
            };
            let cardinal_symbol = dominant_direction.to_cardinal_direction().to_symbol();
            builder.add(
                "Wind",
                &format!("{speed_desc}, {dominant_direction} ({cardinal_symbol})"),
            );
        }
        if let Some(PressureRange { min, max }) = report.pressure_range {
            builder.add("Pressure", &format!("{min} - {max}"));
        }
        builder.string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::MockWeatherProvider;
    use crate::types::weather::*;
    use mockall::predicate::eq;

    #[test]
    fn fetches_forecast_partial_report_for_specified_attributes() {
        let attributes = WeatherAttributeSet::from([
            WeatherAttribute::Temperature,
            WeatherAttribute::Wind,
            WeatherAttribute::WeatherKind,
        ]);
        let mut weather_provider = MockWeatherProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        let report = TodayForecastPartialReport {
            coordinates,
            kind: Some(Kind::Clouds(Clouds::Moderate)),
            temperature_range: Some(TemperatureRange::new_celsius(24.5, 27.1)),
            cloud_coverage_range: None,
            humidity_range: Some(PercentageRange::new(33, 46)),
            wind: None,
            pressure_range: None,
        };
        weather_provider
            .expect_fetch_today_forecast_partial_report()
            .once()
            .with(eq(coordinates), eq(attributes.clone()))
            .return_const(report);

        let sut = TodayForecastList::new(weather_provider, attributes);
        sut.fetch(&coordinates);
    }

    #[test]
    fn describes_only_selected_attributes_in_a_report() {
        let attributes = WeatherAttributeSet::from([
            WeatherAttribute::Temperature,
            WeatherAttribute::Wind,
            WeatherAttribute::WeatherKind,
        ]);
        let sut = TodayForecastList::new(MockWeatherProvider::new(), attributes);
        let coordinates = Coordinates::new(1.23, 45.67);
        let report = TodayForecastPartialReport {
            coordinates,
            kind: Some(Kind::Clouds(Clouds::Moderate)),
            temperature_range: Some(TemperatureRange::new_celsius(24.5, 27.1)),
            cloud_coverage_range: None,
            humidity_range: Some(PercentageRange::new(33, 46)),
            wind: None,
            pressure_range: None,
        };
        let result = sut.format(&report);
        let expected = "Coordinates: 1.23000°, 45.67000°\n\
                        Weather: cloudy\n\
                        Temperature: 24.5°C - 27.1°C\n\
                        Humidity: 33% - 46%";
        assert_eq!(result, expected);
    }

    #[test]
    fn describes_all_attributes() {
        let attributes = WeatherAttributeSet::from([
            WeatherAttribute::WeatherKind,
            WeatherAttribute::Temperature,
            WeatherAttribute::CloudCoverage,
            WeatherAttribute::Humidity,
            WeatherAttribute::Wind,
            WeatherAttribute::Pressure,
        ]);
        let sut = TodayForecastList::new(MockWeatherProvider::new(), attributes);
        let coordinates = Coordinates::new(1.23, 45.67);
        let report = TodayForecastPartialReport {
            coordinates,
            kind: Some(Kind::Clouds(Clouds::Moderate)),
            temperature_range: Some(TemperatureRange::new_celsius(24.5, 27.1)),
            cloud_coverage_range: Some(PercentageRange::new(56, 79)),
            humidity_range: Some(PercentageRange::new(33, 46)),
            wind: Some(WindScope {
                speed_range: SpeedRange::new_meters_per_second(1.2, 2.84),
                dominant_direction: Azimuth::from(178.5),
            }),
            pressure_range: Some(PressureRange::new(999.9, 1111.1)),
        };
        let result = sut.format(&report);
        let expected = "Coordinates: 1.23000°, 45.67000°\n\
                        Weather: cloudy\n\
                        Temperature: 24.5°C - 27.1°C\n\
                        Cloud coverage: 56% - 79%\n\
                        Humidity: 33% - 46%\n\
                        Wind: 1.2 m/s - 2.8 m/s, 178.5° (S)\n\
                        Pressure: 999.9 hPa - 1111.1 hPa";
        assert_eq!(result, expected);
    }
}
