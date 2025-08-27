use super::common;
use crate::domain::ReportStrategy;
use crate::port::weather::WeatherProvider;
use crate::types::report::TodayForecastFullReport;
use crate::types::units::*;

pub struct TodayForecastSummary<P: WeatherProvider> {
    weather_provider: P,
}

impl<P: WeatherProvider> TodayForecastSummary<P> {
    pub fn new(weather_provider: P) -> Self {
        Self { weather_provider }
    }
}

impl<P: WeatherProvider> ReportStrategy for TodayForecastSummary<P> {
    type Report = TodayForecastFullReport;

    fn fetch(&self, coordinates: &Coordinates) -> Self::Report {
        self.weather_provider
            .fetch_forecast_full_report(coordinates)
    }

    fn format(&self, report: &Self::Report) -> String {
        let time_desc = "Today";
        common::describe_forecast(
            time_desc,
            &report.temperature_range,
            &report.kind,
            &report.cloud_coverage_range,
            &report.humidity_range,
            &report.wind,
            &report.pressure_range,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::MockWeatherProvider;
    use crate::types::weather::*;
    use mockall::predicate::eq;

    #[test]
    fn fetches_forecast_full_report() {
        let mut weather_provider = MockWeatherProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        let report = TodayForecastFullReport {
            kind: Kind::Clouds(Clouds::Dense),
            temperature_range: TemperatureRange::new_celsius(12.3, 23.4),
            cloud_coverage_range: PercentageRange::new(25, 76),
            humidity_range: PercentageRange::new(33, 46),
            wind: WindScope {
                speed_range: SpeedRange::new_meters_per_second(2.5, 8.17),
                dominant_direction: Azimuth::from(115.2),
            },
            pressure_range: PressureRange::new(1001.2, 1010.5),
        };
        weather_provider
            .expect_fetch_forecast_full_report()
            .once()
            .with(eq(coordinates))
            .return_const(report);

        let sut = TodayForecastSummary::new(weather_provider);
        sut.fetch(&coordinates);
    }

    #[test]
    fn describes_entire_report() {
        let sut = TodayForecastSummary::new(MockWeatherProvider::new());
        let report = TodayForecastFullReport {
            kind: Kind::Clouds(Clouds::Dense),
            temperature_range: TemperatureRange::new_celsius(12.3, 23.4),
            cloud_coverage_range: PercentageRange::new(66, 94),
            humidity_range: PercentageRange::new(23, 45),
            wind: WindScope {
                speed_range: SpeedRange::new_meters_per_second(2.5, 8.17),
                dominant_direction: Azimuth::from(115.2),
            },
            pressure_range: PressureRange::new(1001.2, 1010.5),
        };
        let result = sut.format(&report);
        let expected = "Today it will be warm \
                        with temperatures starting at 12.3°C and reaching 23.4°C.\n\
                        The sky will be overcast \
                        and clouds will cover from 66% to 94% of the sky.\n\
                        The air will be humid at 23% to 45% \
                        with mostly strong southeast wind blowing at maximum 8.2 m/s.\n\
                        Normal pressure will reach 1001.2 hPa at lowest up to 1010.5 hPa.\n";
        assert_eq!(result, expected);
    }
}
