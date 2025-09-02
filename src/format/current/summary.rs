use crate::format::common::summary::*;
use crate::types::report::CurrentFullReport;
use crate::types::units::*;
use crate::types::weather::*;

pub fn describe(report: &CurrentFullReport) -> String {
    let temperature_desc = describe_temperature(&report.temperature);
    let weather_kind_desc = describe_weather_kind(&report.kind);
    let clouds_desc = describe_cloud_coverage(&report.cloud_coverage);
    let humidity_desc = describe_humidity(&report.humidity);
    let wind_desc = describe_wind(&report.wind);
    let pressure_desc = describe_pressure(&report.pressure);

    #[allow(clippy::uninlined_format_args)]
    {
        format!(
            "{} and {} with {}.\n{} with {}.\n{}.\n",
            temperature_desc,
            weather_kind_desc,
            clouds_desc,
            humidity_desc,
            wind_desc,
            pressure_desc,
        )
    }
}

fn describe_weather_kind(kind: &Kind) -> String {
    let desc = prepare_kind_description(kind);
    match desc {
        KindDescription::Clouds { sky_adjective } => format!("the sky is {sky_adjective}"),
        KindDescription::Fog { description } => format!("{description} is covering the area"),
        KindDescription::Precipitation { description } => format!("{description} is falling"),
        KindDescription::Thunderstorm { description } => format!("{description} is raging"),
    }
}

fn describe_temperature(temperature: &Temperature) -> String {
    let adjective = describe_temperature_adjective(temperature);
    format!("It's {adjective} at {temperature:.1}")
}

fn describe_cloud_coverage(coverage: &Percentage) -> String {
    if coverage.value == 0 {
        "no clouds".into()
    } else {
        format!("clouds covering {coverage} of the sky")
    }
}

fn describe_humidity(percentage: &Percentage) -> String {
    let make_without_humidity = |adjective| format!("The air is {adjective} at {percentage}");
    let make_with_humidity = |adjective| format!("{} humidity", make_without_humidity(adjective));
    let level = prepare_humidity_level(percentage);
    match level {
        HumidityLevel::VeryDry => make_with_humidity("very dry"),
        HumidityLevel::Dry => make_with_humidity("dry"),
        HumidityLevel::Humid => make_without_humidity("humid"),
        HumidityLevel::VeryHumid => make_without_humidity("very humid"),
        HumidityLevel::Heavy => make_with_humidity("heavy"),
    }
}

fn describe_wind(wind: &Wind) -> String {
    let desc = prepare_wind_description(&wind.speed, &wind.direction);
    match desc {
        WindDescription::NoWind => "no wind".into(),
        WindDescription::Wind { description } => {
            format!("{description} blowing at {:.1}", wind.speed)
        }
    }
}

fn describe_pressure(pressure: &Pressure) -> String {
    let adjective = describe_pressure_adjective(pressure);
    format!("{adjective} pressure stands at {pressure:.1}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_values_of_clouds_kind() {
        let string = describe_weather_kind(&Kind::Clouds(Clouds::Moderate));
        assert_eq!(string, "the sky is cloudy");
    }

    #[test]
    fn describes_values_of_fog() {
        let string = describe_weather_kind(&Kind::Fog(Fog::Rime));
        assert_eq!(string, "rime fog is covering the area");
    }

    #[test]
    fn describes_values_of_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&Kind::Precipitation(precipitation));
        assert_eq!(string, "light rain is falling");
    }

    #[test]
    fn describes_thunderstorm() {
        let string = describe_weather_kind(&Kind::Thunderstorm);
        assert_eq!(&string, "thunderstorm is raging");
    }

    #[test]
    fn describes_values_of_temperature_in_celsius() {
        let string = describe_temperature(&Temperature::new_celsius(24.5));
        assert_eq!(string, "It's hot at 24.5°C");
    }

    #[test]
    fn describes_values_of_cloud_coverage() {
        assert_eq!(describe_cloud_coverage(&Percentage::from(0)), "no clouds");
        assert_eq!(
            describe_cloud_coverage(&Percentage::from(27)),
            "clouds covering 27% of the sky"
        );
    }

    #[test]
    fn describes_values_of_humidity() {
        let describe = |value| describe_humidity(&Percentage::from(value));

        assert_eq!(describe(0), "The air is very dry at 0% humidity");
        assert_eq!(describe(16), "The air is dry at 16% humidity");
        assert_eq!(describe(60), "The air is humid at 60%");
        assert_eq!(describe(85), "The air is very humid at 85%");
        assert_eq!(describe(100), "The air is heavy at 100% humidity");
    }

    #[test]
    fn describes_values_of_wind_speed_in_meters_per_second() {
        let wind = Wind {
            speed: Speed::new_meters_per_second(0.11),
            direction: Azimuth::from(12.1),
        };
        let result = describe_wind(&wind);
        assert_eq!(result, "no wind");

        let wind = Wind {
            speed: Speed::new_meters_per_second(9.07),
            direction: Azimuth::from(12.1),
        };
        let result = describe_wind(&wind);
        assert_eq!(result, "strong north wind blowing at 9.1 m/s");
    }

    #[test]
    fn describes_values_of_pressure() {
        let result = describe_pressure(&Pressure::new_hpa(1005.3));
        assert_eq!(result, "Low pressure stands at 1005.3 hPa");
    }

    #[test]
    fn describes_full_report() {
        let report = CurrentFullReport {
            kind: Kind::Clouds(Clouds::Light),
            temperature: Temperature::new_celsius(22.4),
            cloud_coverage: Percentage::from(43),
            humidity: Percentage::from(81),
            wind: Wind {
                speed: Speed::new_meters_per_second(1.12),
                direction: Azimuth::from(140.3),
            },
            pressure: Pressure::new_hpa(1009.3),
        };

        let result = describe(&report);
        let expected: String = "It's warm at 22.4°C \
             and the sky is mostly clear \
             with clouds covering 43% of the sky.\n\
             The air is very humid at 81% \
             with gentle southeast breeze blowing at 1.1 m/s.\n\
             Low pressure stands at 1009.3 hPa.\n"
            .into();
        assert_eq!(result, expected);
    }
}
