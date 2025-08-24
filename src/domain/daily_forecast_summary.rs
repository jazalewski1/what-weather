use crate::domain::ReportStrategy;
use crate::domain::common_forecast::describe_forecast;
use crate::port::weather::WeatherProvider;
use crate::types::report::DailyForecastFullReport;
use crate::types::units::*;

pub struct DailyForecastSummary<P: WeatherProvider> {
    pub weather_provider: P,
    pub period: Period,
}

impl<P: WeatherProvider> DailyForecastSummary<P> {
    pub fn new(weather_provider: P, period: Period) -> Self {
        Self {
            weather_provider,
            period,
        }
    }
}

impl<P: WeatherProvider> ReportStrategy for DailyForecastSummary<P> {
    type Report = DailyForecastFullReport;

    fn fetch(&self, coordinates: &Coordinates) -> Self::Report {
        self.weather_provider
            .fetch_daily_forecast_full_report(coordinates, &self.period)
    }

    fn format(&self, report: &Self::Report) -> String {
        let mut data_iter = report.data.iter();
        let mut result = String::new();
        if let Some(data) = data_iter.next() {
            let date_desc = describe_date(&data.date);
            let day_summary = describe_forecast(
                &date_desc,
                &data.temperature_range,
                &data.kind,
                &data.cloud_coverage_range,
                &data.humidity_range,
                &data.wind,
                &data.pressure_range,
            );
            result.push_str(&day_summary);
        }
        for data in data_iter {
            let date_desc = describe_date(&data.date);
            let day_summary = describe_forecast(
                &date_desc,
                &data.temperature_range,
                &data.kind,
                &data.cloud_coverage_range,
                &data.humidity_range,
                &data.wind,
                &data.pressure_range,
            );
            result.push_str("\n\n");
            result.push_str(&day_summary);
        }
        result
    }
}

fn describe_date(date: &Date) -> String {
    format!("On {}", date.format("%d.%m.%Y"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::MockWeatherProvider;
    use crate::types::report::DailyFullData;
    use crate::types::weather::*;
    use mockall::predicate::eq;

    fn generate_report_for_3_days() -> DailyForecastFullReport {
        let daily_data_1 = DailyFullData {
            date: Date::from_ymd_opt(2025, 08, 24).unwrap(),
            kind: Kind::Clouds(Clouds::Light),
            temperature_range: TemperatureRange::new_celsius(20.6, 26.8),
            cloud_coverage_range: PercentageRange {
                min: Percentage::from(27),
                max: Percentage::from(29),
            },
            humidity_range: PercentageRange {
                min: Percentage::from(14),
                max: Percentage::from(19),
            },
            wind: WindScope {
                speed_range: SpeedRange::new_meters_per_second(0.0, 3.28),
                dominant_direction: Azimuth::from(128.8),
            },
            pressure_range: PressureRange {
                min: Hectopascal::from(995.8),
                max: Hectopascal::from(1019.8),
            },
        };
        let daily_data_2 = DailyFullData {
            date: Date::from_ymd_opt(2025, 08, 25).unwrap(),
            kind: Kind::Clouds(Clouds::Clear),
            temperature_range: TemperatureRange::new_celsius(3.4, 9.0),
            cloud_coverage_range: PercentageRange {
                min: Percentage::from(19),
                max: Percentage::from(96),
            },
            humidity_range: PercentageRange {
                min: Percentage::from(29),
                max: Percentage::from(86),
            },
            wind: WindScope {
                speed_range: SpeedRange::new_meters_per_second(0.0, 2.33),
                dominant_direction: Azimuth::from(2.3),
            },
            pressure_range: PressureRange {
                min: Hectopascal::from(990.3),
                max: Hectopascal::from(1014.3),
            },
        };
        let daily_data_3 = DailyFullData {
            date: Date::from_ymd_opt(2025, 08, 26).unwrap(),
            kind: Kind::Precipitation(Precipitation {
                kind: PrecipitationKind::Snow,
                intensity: PrecipitationIntensity::Light,
                heat: PrecipitationHeat::Normal,
            }),
            temperature_range: TemperatureRange::new_celsius(9.5, 15.5),
            cloud_coverage_range: PercentageRange {
                min: Percentage::from(0),
                max: Percentage::from(1),
            },
            humidity_range: PercentageRange {
                min: Percentage::from(48),
                max: Percentage::from(81),
            },
            wind: WindScope {
                speed_range: SpeedRange::new_meters_per_second(7.39, 10.95),
                dominant_direction: Azimuth::from(167.6),
            },
            pressure_range: PressureRange {
                min: Hectopascal::from(995.6),
                max: Hectopascal::from(1019.6),
            },
        };

        DailyForecastFullReport {
            data: vec![daily_data_1, daily_data_2, daily_data_3],
        }
    }

    #[test]
    fn fetches_daily_forecast_full_report() {
        let mut weather_provider = MockWeatherProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        let report = generate_report_for_3_days();
        let period = Period {
            start: Date::from_ymd_opt(2025, 08, 24).unwrap(),
            length: 3,
        };
        weather_provider
            .expect_fetch_daily_forecast_full_report()
            .once()
            .with(eq(coordinates), eq(period.clone()))
            .return_const(report);

        let sut = DailyForecastSummary::new(weather_provider, period);
        sut.fetch(&coordinates);
    }

    #[test]
    fn describes_dates() {
        let date = Date::from_ymd_opt(2025, 07, 18).unwrap();
        let expected = "On 18.07.2025";
        assert_eq!(describe_date(&date), expected);
    }

    #[test]
    fn describes_entire_report() {
        let period = Period {
            start: Date::from_ymd_opt(2025, 08, 24).unwrap(),
            length: 3,
        };
        let sut = DailyForecastSummary::new(MockWeatherProvider::new(), period);

        let report = generate_report_for_3_days();
        let result = sut.format(&report);
        let expected_day1 = "On 24.08.2025 it will be hot \
            with temperatures starting at 20.6°C and reaching 26.8°C.\n\
            The sky will be mostly clear \
            and clouds will cover from 27% to 29% of the sky.\n\
            The air will be dry at 14% to 19% humidity \
            with mostly gentle southeast breeze blowing at maximum 3.3 m/s.\n\
            Normal pressure will reach 995.8 hPa at lowest up to 1019.8 hPa.";
        let expected_day2 = "On 25.08.2025 it will be cold \
            with temperatures starting at 3.4°C and reaching 9.0°C.\n\
            The sky will be clear \
            and clouds will cover from 19% to 96% of the sky.\n\
            The air will be heavy at 29% to 86% humidity \
            with mostly gentle north breeze blowing at maximum 2.3 m/s.\n\
            Normal pressure will reach 990.3 hPa at lowest up to 1014.3 hPa.";
        let expected_day3 = "On 26.08.2025 it will be cool \
            with temperatures starting at 9.5°C and reaching 15.5°C.\n\
            There will be light snow falling \
            and clouds will cover from 0% to 1% of the sky.\n\
            The air will be very humid at 48% to 81% \
            with mostly strong south wind blowing at maximum 10.9 m/s.\n\
            Normal pressure will reach 995.6 hPa at lowest up to 1019.6 hPa.";
        let expected = format!("{expected_day1}\n\n{expected_day2}\n\n{expected_day3}");
        assert_eq!(result, expected);
    }
}
