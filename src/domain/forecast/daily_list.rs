use std::fmt::Write;

use crate::domain::ReportStrategy;
use crate::domain::common::list_builder::write_param;
use crate::domain::forecast::common_list::write_spec;
use crate::port::weather::WeatherProvider;
use crate::types::attributes::*;
use crate::types::report::*;
use crate::types::units::*;

pub struct DailyForecastList<P: WeatherProvider> {
    weather_provider: P,
    attributes: WeatherAttributeSet,
    period: Period,
}

impl<P: WeatherProvider> DailyForecastList<P> {
    pub fn new(weather_provider: P, attributes: WeatherAttributeSet, period: Period) -> Self {
        Self {
            weather_provider,
            attributes,
            period,
        }
    }
}

impl<P: WeatherProvider> ReportStrategy for DailyForecastList<P> {
    type Report = DailyForecastPartialReport;

    fn fetch(&self, coordinates: &Coordinates) -> Self::Report {
        self.weather_provider.fetch_daily_forecast_partial_report(
            coordinates,
            &self.attributes,
            &self.period,
        )
    }

    fn format(&self, report: &Self::Report) -> String {
        describe_report(report).expect("Failed to write to string")
    }
}

fn describe_report(report: &DailyForecastPartialReport) -> Result<String, std::fmt::Error> {
    let DailyForecastPartialReport { coordinates, data } = &report;
    let mut result = String::new();

    write_param(&mut result, "Coordinates", format!("{coordinates:.5}"));
    writeln!(&mut result)?;

    let mut day_iter = data.iter();
    if let Some(data) = day_iter.next() {
        describe_day(&mut result, data);
    }
    for daily_data in day_iter {
        writeln!(&mut result)?;
        describe_day(&mut result, daily_data);
    }
    Ok(result)
}

fn describe_day(result: &mut String, data: &DailyPartialData) {
    let date_str = data.date.format("%d.%m.%Y").to_string();
    write_param(result, "Date", date_str);
    write_spec(result, &data.spec);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::MockWeatherProvider;
    use crate::types::weather::*;
    use mockall::predicate::eq;

    fn generate_coordinates() -> Coordinates {
        Coordinates::new(1.23, 45.67)
    }

    fn generate_start_date(days_after: u64) -> Date {
        Date::from_ymd_opt(2025, 8, 26)
            .unwrap()
            .checked_add_days(chrono::Days::new(days_after))
            .unwrap()
    }

    fn generate_report_for_3_days() -> DailyForecastPartialReport {
        let data = vec![
            DailyPartialData {
                date: generate_start_date(0),
                spec: ForecastPartialSpec {
                    kind: Some(Kind::Clouds(Clouds::Light)),
                    temperature_range: Some(TemperatureRange::new_celsius(24.5, 27.1)),
                    cloud_coverage_range: None,
                    humidity_range: Some(PercentageRange::new(33, 46)),
                    wind: None,
                    pressure_range: None,
                },
            },
            DailyPartialData {
                date: generate_start_date(1),
                spec: ForecastPartialSpec {
                    kind: Some(Kind::Clouds(Clouds::Moderate)),
                    temperature_range: Some(TemperatureRange::new_celsius(26.5, 29.1)),
                    cloud_coverage_range: None,
                    humidity_range: Some(PercentageRange::new(34, 47)),
                    wind: None,
                    pressure_range: None,
                },
            },
            DailyPartialData {
                date: generate_start_date(2),
                spec: ForecastPartialSpec {
                    kind: Some(Kind::Clouds(Clouds::Dense)),
                    temperature_range: Some(TemperatureRange::new_celsius(28.5, 31.1)),
                    cloud_coverage_range: None,
                    humidity_range: Some(PercentageRange::new(35, 48)),
                    wind: None,
                    pressure_range: None,
                },
            },
        ];
        DailyForecastPartialReport {
            coordinates: generate_coordinates(),
            data,
        }
    }

    #[test]
    fn fetches_report_for_specified_attributes_and_period() {
        let mut weather_provider = MockWeatherProvider::new();
        let coordinates = generate_coordinates();
        let period = Period {
            start: generate_start_date(0),
            length: 3,
        };
        let attributes = WeatherAttributeSet::from([
            WeatherAttribute::Temperature,
            WeatherAttribute::Wind,
            WeatherAttribute::WeatherKind,
        ]);
        let report = generate_report_for_3_days();
        weather_provider
            .expect_fetch_daily_forecast_partial_report()
            .once()
            .with(eq(coordinates), eq(attributes.clone()), eq(period.clone()))
            .return_const(report);

        let sut = DailyForecastList::new(weather_provider, attributes, period);
        sut.fetch(&coordinates);
    }

    #[test]
    fn formats_report() {
        let attributes = WeatherAttributeSet::from([
            WeatherAttribute::Temperature,
            WeatherAttribute::Wind,
            WeatherAttribute::WeatherKind,
        ]);
        let period = Period {
            start: generate_start_date(0),
            length: 3,
        };
        let sut = DailyForecastList::new(MockWeatherProvider::new(), attributes, period);
        let report = generate_report_for_3_days();
        let result = sut.format(&report);
        let expected = "Coordinates: 1.23000°, 45.67000°\n\
                        \n\
                        Date: 26.08.2025\n\
                        Weather: light clouds\n\
                        Temperature: 24.5°C - 27.1°C\n\
                        Humidity: 33% - 46%\n\
                        \n\
                        Date: 27.08.2025\n\
                        Weather: cloudy\n\
                        Temperature: 26.5°C - 29.1°C\n\
                        Humidity: 34% - 47%\n\
                        \n\
                        Date: 28.08.2025\n\
                        Weather: overcast sky\n\
                        Temperature: 28.5°C - 31.1°C\n\
                        Humidity: 35% - 48%\n";
        assert_eq!(result, expected);
    }
}
