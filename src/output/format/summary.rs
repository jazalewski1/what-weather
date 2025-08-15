use crate::types::WeatherReport;
use crate::types::weather::*;

pub fn format(report: &WeatherReport) -> String {
    let temperature_desc = describe_temperature(report.temperature);
    let weather_kind_desc = describe_weather_kind(&report.kind);
    let clouds_desc = describe_cloud_coverage(report.cloud_coverage);
    let humidity_desc = describe_humidity(report.humidity);
    let wind_desc = describe_wind(&report.wind);
    let pressure_desc = describe_pressure(report.pressure);

    #[allow(clippy::uninlined_format_args)]
    {
        format!(
            "{} and {} with {}.\n{} with {}.\n{}.",
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
    match kind {
        Kind::Clouds(clouds) => match clouds {
            Clouds::Clear => "the sky is clear".into(),
            Clouds::Light => "the sky is mostly clear".into(),
            Clouds::Moderate => "the sky is moderately cloudy".into(),
            Clouds::Dense => "the sky is overcast".into(),
        },
        Kind::Fog(fog) => {
            let kind = match fog {
                Fog::Normal => "fog",
                Fog::Rime => "rime fog",
            };
            format!("{kind} is covering the area")
        }
        Kind::Precipitation(precipitation) => {
            let intensity = match precipitation.intensity {
                PrecipitationIntensity::Light => "light",
                PrecipitationIntensity::Moderate => "moderate",
                PrecipitationIntensity::Heavy => "heavy",
                PrecipitationIntensity::Shower => "shower",
            };
            let kind = match precipitation.kind {
                PrecipitationKind::Rain => "rain",
                PrecipitationKind::Snow => "snow",
            };
            match precipitation.heat {
                PrecipitationHeat::Normal => format!("{intensity} {kind} is falling"),
                PrecipitationHeat::Freezing => format!("freezing {intensity} {kind} is falling"),
            }
        }
        Kind::Thunderstorm => "thunderstorm is raging".into(),
    }
}

fn describe_temperature(temperature: Temperature) -> String {
    let adjective = if temperature <= 0.0 {
        "freezing"
    } else if temperature <= 10.0 {
        "cold"
    } else if temperature <= 17.0 {
        "cool"
    } else if temperature <= 24.0 {
        "warm"
    } else if temperature <= 35.0 {
        "hot"
    } else {
        "very hot"
    };
    format!("It's {adjective} at {temperature:.1}°C")
}

fn describe_cloud_coverage(coverage: CloudCoverage) -> String {
    if coverage == 0 {
        "no clouds".into()
    } else {
        format!("clouds covering {coverage}% of the sky")
    }
}

fn describe_humidity(humidity: i8) -> String {
    let (definition, with_postfix) = if humidity <= 15 {
        ("very dry", true)
    } else if humidity <= 30 {
        ("dry", true)
    } else if humidity <= 60 {
        ("humid", false)
    } else if humidity <= 85 {
        ("very humid", false)
    } else {
        ("heavy", true)
    };
    let label = if with_postfix {
        format!("{humidity}% humidity")
    } else {
        format!("{humidity}%")
    };
    format!("The air is {definition} at {label}")
}

fn describe_wind(wind: &Wind) -> String {
    if wind.speed <= 0.2 {
        return "no wind".into();
    }

    let direction_definition = if wind.direction <= 22.5 {
        "north"
    } else if wind.direction <= 67.5 {
        "northeast"
    } else if wind.direction <= 112.5 {
        "east"
    } else if wind.direction <= 157.5 {
        "southeast"
    } else if wind.direction <= 202.5 {
        "south"
    } else if wind.direction <= 247.5 {
        "southwest"
    } else if wind.direction <= 292.5 {
        "west"
    } else if wind.direction <= 337.5 {
        "northwest"
    } else {
        "north"
    };

    let wind_definition = if wind.speed <= 3.3 {
        format!("gentle {direction_definition} breeze")
    } else if wind.speed <= 8.0 {
        format!("{direction_definition} wind")
    } else if wind.speed <= 13.8 {
        format!("strong {direction_definition} wind")
    } else {
        format!("very strong {direction_definition} wind")
    };
    format!("{wind_definition} blowing at {:.1} m/s", wind.speed)
}

fn describe_pressure(pressure: f32) -> String {
    let adjective = if pressure <= 1000.0 {
        "Very low"
    } else if pressure <= 1010.0 {
        "Low"
    } else if pressure <= 1020.0 {
        "Normal"
    } else if pressure <= 1030.0 {
        "High"
    } else {
        "Very high"
    };
    format!("{adjective} pressure stands at {pressure:.1} hPa")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Coordinates;

    fn assert_starts_with(string: &str, expected_start: &str) {
        assert!(
            string.starts_with(expected_start),
            "\"{string}\"\nexpected to start with\n\"{expected_start}\""
        );
    }

    fn assert_contains(string: &str, expected_substring: &str) {
        assert!(
            string.contains(expected_substring),
            "\"{string}\"\nexpected to contain\n\"{expected_substring}\""
        );
    }

    #[test]
    fn describe_clear_sky() {
        let string = describe_weather_kind(&Kind::Clouds(Clouds::Clear));
        assert_eq!(&string, "the sky is clear");
    }

    #[test]
    fn describe_lightly_cloudy_sky() {
        let string = describe_weather_kind(&Kind::Clouds(Clouds::Light));
        assert_starts_with(&string, "the sky is mostly clear");
    }

    #[test]
    fn describe_moderately_cloudy_sky() {
        let string = describe_weather_kind(&Kind::Clouds(Clouds::Moderate));
        assert_starts_with(&string, "the sky is moderately cloudy");
    }

    #[test]
    fn describe_densely_cloudy_sky() {
        let string = describe_weather_kind(&Kind::Clouds(Clouds::Dense));
        assert_starts_with(&string, "the sky is overcast");
    }

    #[test]
    fn describe_normal_fog() {
        let string = describe_weather_kind(&Kind::Fog(Fog::Normal));
        assert_starts_with(&string, "fog is covering the area");
    }

    #[test]
    fn describe_rime_fog() {
        let string = describe_weather_kind(&Kind::Fog(Fog::Rime));
        assert_starts_with(&string, "rime fog is covering the area");
    }

    const INTENSITY_VALUES: [PrecipitationIntensity; 4] = [
        PrecipitationIntensity::Light,
        PrecipitationIntensity::Moderate,
        PrecipitationIntensity::Heavy,
        PrecipitationIntensity::Shower,
    ];
    const HEAT_VALUES: [PrecipitationHeat; 2] =
        [PrecipitationHeat::Normal, PrecipitationHeat::Freezing];

    #[test]
    fn describe_any_rain() {
        for intensity in INTENSITY_VALUES {
            for heat in HEAT_VALUES {
                let precipitation = Precipitation {
                    kind: PrecipitationKind::Rain,
                    intensity,
                    heat,
                };
                let string = describe_weather_kind(&Kind::Precipitation(precipitation));
                assert_contains(&string, "rain is falling");
            }
        }
    }

    #[test]
    fn describe_any_snow() {
        for intensity in INTENSITY_VALUES {
            for heat in HEAT_VALUES {
                let precipitation = Precipitation {
                    kind: PrecipitationKind::Snow,
                    intensity,
                    heat,
                };
                let string = describe_weather_kind(&Kind::Precipitation(precipitation));
                assert_contains(&string, "snow is falling");
            }
        }
    }

    #[test]
    fn describe_light_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&Kind::Precipitation(precipitation));
        assert_starts_with(&string, "light");
    }

    #[test]
    fn describe_moderate_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&Kind::Precipitation(precipitation));
        assert_starts_with(&string, "moderate");
    }

    #[test]
    fn describe_heavy_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&Kind::Precipitation(precipitation));
        assert_starts_with(&string, "heavy");
    }

    #[test]
    fn describe_shower_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&Kind::Precipitation(precipitation));
        assert_starts_with(&string, "shower");
    }

    #[test]
    fn describe_freezing_heat_light_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&Kind::Precipitation(precipitation));
        assert_contains(&string, "freezing light");
    }

    #[test]
    fn describe_freezing_heat_moderate_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&Kind::Precipitation(precipitation));
        assert_contains(&string, "freezing moderate");
    }

    #[test]
    fn describe_freezing_heat_heavy_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&Kind::Precipitation(precipitation));
        assert_contains(&string, "freezing heavy");
    }

    #[test]
    fn describe_freezing_heat_shower_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&Kind::Precipitation(precipitation));
        assert_contains(&string, "freezing shower");
    }

    #[test]
    fn describe_thunderstorm() {
        let string = describe_weather_kind(&Kind::Thunderstorm);
        assert_starts_with(&string, "thunderstorm is raging");
    }

    #[test]
    fn describe_temperature_values() {
        assert_eq!(describe_temperature(-3.0), "It's freezing at -3.0°C");
        assert_eq!(describe_temperature(-0.1), "It's freezing at -0.1°C");
        assert_eq!(describe_temperature(0.0), "It's freezing at 0.0°C");

        assert_eq!(describe_temperature(1.0), "It's cold at 1.0°C");
        assert_eq!(describe_temperature(4.5), "It's cold at 4.5°C");
        assert_eq!(describe_temperature(10.0), "It's cold at 10.0°C");

        assert_eq!(describe_temperature(10.1), "It's cool at 10.1°C");
        assert_eq!(describe_temperature(13.7), "It's cool at 13.7°C");
        assert_eq!(describe_temperature(17.0), "It's cool at 17.0°C");

        assert_eq!(describe_temperature(17.1), "It's warm at 17.1°C");
        assert_eq!(describe_temperature(20.0), "It's warm at 20.0°C");
        assert_eq!(describe_temperature(24.0), "It's warm at 24.0°C");

        assert_eq!(describe_temperature(24.1), "It's hot at 24.1°C");
        assert_eq!(describe_temperature(29.9), "It's hot at 29.9°C");
        assert_eq!(describe_temperature(35.0), "It's hot at 35.0°C");

        assert_eq!(describe_temperature(35.1), "It's very hot at 35.1°C");
        assert_eq!(describe_temperature(40.2), "It's very hot at 40.2°C");
    }

    #[test]
    fn describe_cloud_coverage_values() {
        assert_eq!(describe_cloud_coverage(0), "no clouds");
        assert_eq!(
            describe_cloud_coverage(27),
            "clouds covering 27% of the sky"
        );
        assert_eq!(
            describe_cloud_coverage(100),
            "clouds covering 100% of the sky"
        );
    }

    #[test]
    fn describe_humidity_values() {
        assert_eq!(describe_humidity(0), "The air is very dry at 0% humidity");
        assert_eq!(describe_humidity(15), "The air is very dry at 15% humidity");

        assert_eq!(describe_humidity(16), "The air is dry at 16% humidity");
        assert_eq!(describe_humidity(30), "The air is dry at 30% humidity");

        assert_eq!(describe_humidity(31), "The air is humid at 31%");
        assert_eq!(describe_humidity(60), "The air is humid at 60%");

        assert_eq!(describe_humidity(61), "The air is very humid at 61%");
        assert_eq!(describe_humidity(85), "The air is very humid at 85%");

        assert_eq!(describe_humidity(86), "The air is heavy at 86% humidity");
        assert_eq!(describe_humidity(100), "The air is heavy at 100% humidity");
    }

    #[test]
    fn describe_wind_speed() {
        let assert_string_with_speed = |speed, expected_str| {
            let wind = Wind {
                speed,
                direction: 42.0,
            };
            let result = describe_wind(&wind);
            assert_eq!(result, expected_str);
        };

        assert_string_with_speed(0.0, "no wind");
        assert_string_with_speed(0.2, "no wind");

        assert_string_with_speed(0.21, "gentle northeast breeze blowing at 0.2 m/s");
        assert_string_with_speed(2.9, "gentle northeast breeze blowing at 2.9 m/s");
        assert_string_with_speed(3.3, "gentle northeast breeze blowing at 3.3 m/s");

        assert_string_with_speed(3.31, "northeast wind blowing at 3.3 m/s");
        assert_string_with_speed(5.57, "northeast wind blowing at 5.6 m/s");
        assert_string_with_speed(8.0, "northeast wind blowing at 8.0 m/s");

        assert_string_with_speed(8.01, "strong northeast wind blowing at 8.0 m/s");
        assert_string_with_speed(10.3, "strong northeast wind blowing at 10.3 m/s");
        assert_string_with_speed(13.8, "strong northeast wind blowing at 13.8 m/s");

        assert_string_with_speed(13.81, "very strong northeast wind blowing at 13.8 m/s");
        assert_string_with_speed(15.0, "very strong northeast wind blowing at 15.0 m/s");
    }

    #[test]
    fn describe_wind_direction() {
        let assert_string_with_direction = |direction, expected_str| {
            let wind = Wind {
                speed: 5.0,
                direction,
            };
            let result = describe_wind(&wind);
            assert_eq!(result, expected_str);
        };

        assert_string_with_direction(337.6, "north wind blowing at 5.0 m/s");
        assert_string_with_direction(345.0, "north wind blowing at 5.0 m/s");
        assert_string_with_direction(359.9, "north wind blowing at 5.0 m/s");
        assert_string_with_direction(0.0, "north wind blowing at 5.0 m/s");
        assert_string_with_direction(13.1, "north wind blowing at 5.0 m/s");
        assert_string_with_direction(22.5, "north wind blowing at 5.0 m/s");

        assert_string_with_direction(22.6, "northeast wind blowing at 5.0 m/s");
        assert_string_with_direction(65.2, "northeast wind blowing at 5.0 m/s");
        assert_string_with_direction(67.5, "northeast wind blowing at 5.0 m/s");

        assert_string_with_direction(67.6, "east wind blowing at 5.0 m/s");
        assert_string_with_direction(100.1, "east wind blowing at 5.0 m/s");
        assert_string_with_direction(112.5, "east wind blowing at 5.0 m/s");

        assert_string_with_direction(112.6, "southeast wind blowing at 5.0 m/s");
        assert_string_with_direction(121.9, "southeast wind blowing at 5.0 m/s");
        assert_string_with_direction(157.5, "southeast wind blowing at 5.0 m/s");

        assert_string_with_direction(157.6, "south wind blowing at 5.0 m/s");
        assert_string_with_direction(200.0, "south wind blowing at 5.0 m/s");
        assert_string_with_direction(202.5, "south wind blowing at 5.0 m/s");

        assert_string_with_direction(202.6, "southwest wind blowing at 5.0 m/s");
        assert_string_with_direction(213.3, "southwest wind blowing at 5.0 m/s");
        assert_string_with_direction(247.5, "southwest wind blowing at 5.0 m/s");

        assert_string_with_direction(247.6, "west wind blowing at 5.0 m/s");
        assert_string_with_direction(281.4, "west wind blowing at 5.0 m/s");
        assert_string_with_direction(292.5, "west wind blowing at 5.0 m/s");

        assert_string_with_direction(292.6, "northwest wind blowing at 5.0 m/s");
        assert_string_with_direction(293.5, "northwest wind blowing at 5.0 m/s");
        assert_string_with_direction(337.5, "northwest wind blowing at 5.0 m/s");
    }

    #[test]
    fn describe_pressure_values() {
        assert_eq!(
            describe_pressure(995.0),
            "Very low pressure stands at 995.0 hPa"
        );
        assert_eq!(
            describe_pressure(1000.0),
            "Very low pressure stands at 1000.0 hPa"
        );

        assert_eq!(
            describe_pressure(1000.1),
            "Low pressure stands at 1000.1 hPa"
        );
        assert_eq!(
            describe_pressure(1005.3),
            "Low pressure stands at 1005.3 hPa"
        );
        assert_eq!(
            describe_pressure(1010.0),
            "Low pressure stands at 1010.0 hPa"
        );

        assert_eq!(
            describe_pressure(1010.1),
            "Normal pressure stands at 1010.1 hPa"
        );
        assert_eq!(
            describe_pressure(1018.7),
            "Normal pressure stands at 1018.7 hPa"
        );
        assert_eq!(
            describe_pressure(1020.0),
            "Normal pressure stands at 1020.0 hPa"
        );

        assert_eq!(
            describe_pressure(1020.1),
            "High pressure stands at 1020.1 hPa"
        );
        assert_eq!(
            describe_pressure(1026.1),
            "High pressure stands at 1026.1 hPa"
        );
        assert_eq!(
            describe_pressure(1030.0),
            "High pressure stands at 1030.0 hPa"
        );

        assert_eq!(
            describe_pressure(1030.1),
            "Very high pressure stands at 1030.1 hPa"
        );
        assert_eq!(
            describe_pressure(1035.0),
            "Very high pressure stands at 1035.0 hPa"
        );
    }

    #[test]
    fn describe_entire_summary() {
        let report = WeatherReport {
            coordinates: Coordinates {
                latitude: 1.2,
                longitude: 3.4,
            },
            kind: Kind::Clouds(Clouds::Light),
            temperature: 22.4,
            cloud_coverage: 43,
            humidity: 81,
            wind: Wind {
                speed: 1.07,
                direction: 155.5,
            },
            pressure: 1009.3,
        };
        let expected: String = "It's warm at 22.4°C \
             and the sky is mostly clear \
             with clouds covering 43% of the sky.\n\
             The air is very humid at 81% \
             with gentle southeast breeze blowing at 1.1 m/s.\n\
             Low pressure stands at 1009.3 hPa."
            .into();
        let result = format(&report);
        assert_eq!(result, expected);
    }
}
