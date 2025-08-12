use crate::domain::port::Presenter;
use crate::domain::types::*;

pub struct ConsolePresenter;

fn describe_weather_kind(kind: &WeatherKind) -> String {
    match kind {
        WeatherKind::Clouds(clouds) => match clouds {
            Clouds::Clear => "the sky is clear".into(),
            Clouds::Light => "the sky is mostly clear".into(),
            Clouds::Moderate => "the sky is moderately cloudy".into(),
            Clouds::Dense => "the sky is overcast".into(),
        },
        WeatherKind::Fog(fog) => {
            let kind = match fog {
                Fog::Normal => "fog",
                Fog::Rime => "rime fog",
            };
            format!("{kind} is covering the area")
        }
        WeatherKind::Precipitation(precipitation) => {
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
        WeatherKind::Thunderstorm => "thunderstorm is raging".into(),
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

fn describe(report: &WeatherReport) -> String {
    let temperature_desc = describe_temperature(report.temperature);
    let weather_kind_desc = describe_weather_kind(&report.kind);
    format!("{temperature_desc} and {weather_kind_desc}.")
}

impl Presenter for ConsolePresenter {
    fn display(&self, report: &WeatherReport) {
        let desc = describe(report);
        println!("{desc}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let string = describe_weather_kind(&WeatherKind::Clouds(Clouds::Clear));
        assert_eq!(&string, "the sky is clear");
    }

    #[test]
    fn describe_lightly_cloudy_sky() {
        let string = describe_weather_kind(&WeatherKind::Clouds(Clouds::Light));
        assert_starts_with(&string, "the sky is mostly clear");
    }

    #[test]
    fn describe_moderately_cloudy_sky() {
        let string = describe_weather_kind(&WeatherKind::Clouds(Clouds::Moderate));
        assert_starts_with(&string, "the sky is moderately cloudy");
    }

    #[test]
    fn describe_densely_cloudy_sky() {
        let string = describe_weather_kind(&WeatherKind::Clouds(Clouds::Dense));
        assert_starts_with(&string, "the sky is overcast");
    }

    #[test]
    fn describe_normal_fog() {
        let string = describe_weather_kind(&WeatherKind::Fog(Fog::Normal));
        assert_starts_with(&string, "fog is covering the area");
    }

    #[test]
    fn describe_rime_fog() {
        let string = describe_weather_kind(&WeatherKind::Fog(Fog::Rime));
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
                let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
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
                let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
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
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_starts_with(&string, "light");
    }

    #[test]
    fn describe_moderate_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_starts_with(&string, "moderate");
    }

    #[test]
    fn describe_heavy_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_starts_with(&string, "heavy");
    }

    #[test]
    fn describe_shower_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_starts_with(&string, "shower");
    }

    #[test]
    fn describe_freezing_heat_light_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_contains(&string, "freezing light");
    }

    #[test]
    fn describe_freezing_heat_moderate_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_contains(&string, "freezing moderate");
    }

    #[test]
    fn describe_freezing_heat_heavy_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_contains(&string, "freezing heavy");
    }

    #[test]
    fn describe_freezing_heat_shower_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_contains(&string, "freezing shower");
    }

    #[test]
    fn describe_thunderstorm() {
        let string = describe_weather_kind(&WeatherKind::Thunderstorm);
        assert_starts_with(&string, "thunderstorm is raging");
    }

    #[test]
    fn describe_temperatures() {
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
    fn desribe_entire_summary() {
        let report = WeatherReport {
            coordinates: Coordinates {
                latitude: 1.2,
                longitude: 3.4,
            },
            kind: WeatherKind::Clouds(Clouds::Light),
            temperature: 22.4,
        };
        let string = describe(&report);
        let expected = "It's warm at 22.4°C and the sky is mostly clear.";
        assert_eq!(string, expected);
    }
}
