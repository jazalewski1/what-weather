use crate::format::common::summary::*;
use crate::types::report::{DailyFullData, DailyFullReport};
use crate::types::units::*;
use crate::types::weather::*;

pub fn describe(report: &DailyFullReport) -> String {
    let mut data_iter = report.data.iter();
    let mut result = String::new();
    if let Some(data) = data_iter.next() {
        let date_desc = String::from("Today");
        let day_summary = describe_day(date_desc, data);
        result.push_str(&day_summary);
    }
    if let Some(data) = data_iter.next() {
        let date_desc = String::from("Tomorrow");
        let day_summary = describe_day(date_desc, data);
        result.push('\n');
        result.push_str(&day_summary);
    }
    for data in data_iter {
        let date_desc = describe_date(&data.date);
        let day_summary = describe_day(date_desc, data);
        result.push('\n');
        result.push_str(&day_summary);
    }
    result
}

fn describe_date(date: &Date) -> String {
    format!("On {}", date.format("%d.%m.%Y"))
}

fn describe_day(date_desc: String, data: &DailyFullData) -> String {
    let temperature_desc = describe_temperature_range(&data.temperature_range);
    let kind_desc = describe_kind(&data.kind);
    let cloud_coverage_desc = describe_cloud_coverage_range(&data.cloud_coverage_range);
    let humidity_desc = describe_humidity_range(&data.humidity_range);
    let wind_desc = describe_wind_scope(&data.wind);
    let pressure_desc = describe_pressure_range(&data.pressure_range);
    #[allow(clippy::uninlined_format_args)]
    {
        format!(
            "{} {}.\n{} and {}.\n{} with {}.\n{}.\n",
            date_desc,
            temperature_desc,
            kind_desc,
            cloud_coverage_desc,
            humidity_desc,
            wind_desc,
            pressure_desc,
        )
    }
}

fn describe_kind(kind: &Kind) -> String {
    let desc = prepare_kind_description(kind);
    match desc {
        KindDescription::Clouds { sky_adjective } => format!("The sky will be {sky_adjective}"),
        KindDescription::Fog { description } => {
            format!("A {description} will be covering the area")
        }
        KindDescription::Precipitation { description } => {
            format!("There will be {description} falling")
        }
        KindDescription::Thunderstorm { description } => format!("A {description} will be raging"),
    }
}

fn describe_temperature_range(temperature_range: &TemperatureRange) -> String {
    match temperature_range {
        TemperatureRange::Celsius { min, max } => {
            let adjective = describe_temperature_adjective(&Temperature::Celsius(*max));
            format!("it will be {adjective} with temperatures starting at {min} and reaching {max}")
        }
    }
}

fn describe_cloud_coverage_range(range: &PercentageRange) -> String {
    format!(
        "clouds will cover from {} to {} of the sky",
        range.min, range.max
    )
}

fn describe_humidity_range(range: &PercentageRange) -> String {
    let make_without_humidity = |adjective| {
        format!(
            "The air will be {adjective} at {} to {}",
            range.min, range.max
        )
    };
    let make_with_humidity = |adjective| format!("{} humidity", make_without_humidity(adjective));
    let level = prepare_humidity_level(&range.max);
    match level {
        HumidityLevel::VeryDry => make_with_humidity("very dry"),
        HumidityLevel::Dry => make_with_humidity("dry"),
        HumidityLevel::Humid => make_without_humidity("humid"),
        HumidityLevel::VeryHumid => make_without_humidity("very humid"),
        HumidityLevel::Heavy => make_with_humidity("heavy"),
    }
}

fn describe_wind_scope(scope: &WindScope) -> String {
    let max_speed = match scope.speed_range {
        SpeedRange::MetersPerSecond { max, .. } => Speed::MetersPerSecond(max),
    };
    let desc = prepare_wind_description(&max_speed, &scope.dominant_direction);
    match desc {
        WindDescription::NoWind => "mostly no wind".into(),
        WindDescription::Wind { description } => {
            format!("mostly {description} blowing at maximum {max_speed}")
        }
    }
}

fn describe_pressure_range(pressure_range: &PressureRange) -> String {
    match pressure_range {
        PressureRange::Hpa { min, max } => {
            let adjective = describe_hectopascal_adjective(max);
            format!("{adjective} pressure will reach {min:.1} at lowest up to {max:.1}",)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::report::DailyFullData;

    fn generate_report_for_3_days() -> DailyFullReport {
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
            pressure_range: PressureRange::new_hpa(995.8, 1019.8),
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
            pressure_range: PressureRange::new_hpa(990.3, 1014.3),
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
            pressure_range: PressureRange::new_hpa(995.6, 1019.6),
        };

        DailyFullReport {
            data: vec![daily_data_1, daily_data_2, daily_data_3],
        }
    }

    #[test]
    fn describes_dates() {
        let date = Date::from_ymd_opt(2025, 07, 18).unwrap();
        let expected = "On 18.07.2025";
        assert_eq!(describe_date(&date), expected);
    }

    #[test]
    fn describes_temperature_range() {
        let range = TemperatureRange::new_celsius(15.1, 33.3);
        let result = describe_temperature_range(&range);
        let expected = "it will be hot with temperatures starting at 15.1°C and reaching 33.3°C";
        assert_eq!(result, expected);
    }

    #[test]
    fn describes_cloud_kind() {
        let kind = Kind::Clouds(Clouds::Dense);
        let result = describe_kind(&kind);
        assert_eq!(result, "The sky will be overcast");
    }

    #[test]
    fn describes_fog_kind() {
        let kind = Kind::Fog(Fog::Normal);
        let result = describe_kind(&kind);
        assert_eq!(result, "A fog will be covering the area");
    }

    #[test]
    fn describes_precipitation_kind() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        };
        let kind = Kind::Precipitation(precipitation);
        let result = describe_kind(&kind);
        assert_eq!(result, "There will be moderate rain falling");
    }

    #[test]
    fn describes_thunderstorm_kind() {
        let kind = Kind::Thunderstorm;
        let result = describe_kind(&kind);
        assert_eq!(result, "A thunderstorm will be raging");
    }

    #[test]
    fn describes_cloud_coverage_ranges() {
        let range = PercentageRange::new(26, 57);
        let result = describe_cloud_coverage_range(&range);
        assert_eq!(result, "clouds will cover from 26% to 57% of the sky");
    }

    #[test]
    fn describes_humidity_ranges() {
        let describe = |min, max| {
            let range = PercentageRange::new(min, max);
            describe_humidity_range(&range)
        };

        assert_eq!(
            describe(0, 15),
            "The air will be very dry at 0% to 15% humidity"
        );
        assert_eq!(
            describe(15, 30),
            "The air will be dry at 15% to 30% humidity"
        );
        assert_eq!(describe(30, 60), "The air will be humid at 30% to 60%");
        assert_eq!(describe(60, 85), "The air will be very humid at 60% to 85%");
        assert_eq!(
            describe(85, 100),
            "The air will be heavy at 85% to 100% humidity"
        );
    }

    #[test]
    fn describes_wind_scope() {
        let wind = WindScope {
            speed_range: SpeedRange::new_meters_per_second(0.05, 0.15),
            dominant_direction: Azimuth::from(273.3),
        };
        let result = describe_wind_scope(&wind);
        assert_eq!(result, "mostly no wind");

        let wind = WindScope {
            speed_range: SpeedRange::new_meters_per_second(5.3, 9.7),
            dominant_direction: Azimuth::from(273.3),
        };
        let result = describe_wind_scope(&wind);
        assert_eq!(result, "mostly strong west wind blowing at maximum 9.7 m/s");
    }

    #[test]
    fn describes_pressure_range() {
        let range = PressureRange::new_hpa(1011.9, 1020.5);
        let result = describe_pressure_range(&range);
        assert_eq!(
            result,
            "High pressure will reach 1011.9 hPa at lowest up to 1020.5 hPa"
        );
    }

    #[test]
    fn describes_entire_report() {
        let report = generate_report_for_3_days();
        let result = describe(&report);
        let expected_day1 = "Today it will be hot \
            with temperatures starting at 20.6°C and reaching 26.8°C.\n\
            The sky will be mostly clear \
            and clouds will cover from 27% to 29% of the sky.\n\
            The air will be dry at 14% to 19% humidity \
            with mostly gentle southeast breeze blowing at maximum 3.3 m/s.\n\
            Normal pressure will reach 995.8 hPa at lowest up to 1019.8 hPa.\n";
        let expected_day2 = "Tomorrow it will be cool \
            with temperatures starting at 3.4°C and reaching 9.0°C.\n\
            The sky will be clear \
            and clouds will cover from 19% to 96% of the sky.\n\
            The air will be heavy at 29% to 86% humidity \
            with mostly gentle north breeze blowing at maximum 2.3 m/s.\n\
            Normal pressure will reach 990.3 hPa at lowest up to 1014.3 hPa.\n";
        let expected_day3 = "On 26.08.2025 it will be warm \
            with temperatures starting at 9.5°C and reaching 15.5°C.\n\
            There will be light snow falling \
            and clouds will cover from 0% to 1% of the sky.\n\
            The air will be very humid at 48% to 81% \
            with mostly strong south wind blowing at maximum 10.9 m/s.\n\
            Normal pressure will reach 995.6 hPa at lowest up to 1019.6 hPa.\n";
        let expected = format!("{expected_day1}\n{expected_day2}\n{expected_day3}");
        assert_eq!(result, expected);
    }
}
