use crate::domain::port::Presenter;
use crate::domain::types::*;

pub struct ConsolePresenter;

fn describe_weather_kind(kind: &WeatherKind) -> String {
    match kind {
        WeatherKind::Clouds(clouds) => match clouds {
            Clouds::Clear => "The sky is clear".into(),
            Clouds::Light => "The sky is mostly clear".into(),
            Clouds::Moderate => "The sky is moderately cloudy".into(),
            Clouds::Dense => "The sky is overcast".into(),
        },
        WeatherKind::Fog(fog) => {
            let kind = match fog {
                Fog::Normal => "Fog",
                Fog::Rime => "Rime fog",
            };
            format!("{kind} is covering the area")
        }
        WeatherKind::Precipitation(precipitation) => {
            let heat_and_intensity = match precipitation.heat {
                PrecipitationHeat::Normal => match precipitation.intensity {
                    PrecipitationIntensity::Light => "Light",
                    PrecipitationIntensity::Moderate => "Moderate",
                    PrecipitationIntensity::Heavy => "Heavy",
                    PrecipitationIntensity::Shower => "Shower",
                },
                PrecipitationHeat::Freezing => match precipitation.intensity {
                    PrecipitationIntensity::Light => "Freezing light",
                    PrecipitationIntensity::Moderate => "Freezing moderate",
                    PrecipitationIntensity::Heavy => "Freezing heavy",
                    PrecipitationIntensity::Shower => "Freezing shower",
                },
            };
            let kind = match precipitation.kind {
                PrecipitationKind::Rain => "rain",
                PrecipitationKind::Snow => "snow",
            };
            format!("{heat_and_intensity} {kind} is falling")
        }
        WeatherKind::Thunderstorm => "Thunderstorm is raging".into(),
    }
}

fn describe(report: &WeatherReport) -> String {
    let weather_kind_desc = describe_weather_kind(&report.kind);
    weather_kind_desc
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
        assert_eq!(&string, "The sky is clear");
    }

    #[test]
    fn describe_lightly_cloudy_sky() {
        let string = describe_weather_kind(&WeatherKind::Clouds(Clouds::Light));
        assert_starts_with(&string, "The sky is mostly clear");
    }

    #[test]
    fn describe_moderately_cloudy_sky() {
        let string = describe_weather_kind(&WeatherKind::Clouds(Clouds::Moderate));
        assert_starts_with(&string, "The sky is moderately cloudy");
    }

    #[test]
    fn describe_densely_cloudy_sky() {
        let string = describe_weather_kind(&WeatherKind::Clouds(Clouds::Dense));
        assert_starts_with(&string, "The sky is overcast");
    }

    #[test]
    fn describe_normal_fog() {
        let string = describe_weather_kind(&WeatherKind::Fog(Fog::Normal));
        assert_starts_with(&string, "Fog is covering the area");
    }

    #[test]
    fn describe_rime_fog() {
        let string = describe_weather_kind(&WeatherKind::Fog(Fog::Rime));
        assert_starts_with(&string, "Rime fog is covering the area");
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
        assert_starts_with(&string, "Light");
    }

    #[test]
    fn describe_moderate_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_starts_with(&string, "Moderate");
    }

    #[test]
    fn describe_heavy_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_starts_with(&string, "Heavy");
    }

    #[test]
    fn describe_shower_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_starts_with(&string, "Shower");
    }

    #[test]
    fn describe_freezing_heat_light_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_contains(&string, "Freezing light");
    }

    #[test]
    fn describe_freezing_heat_moderate_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_contains(&string, "Freezing moderate");
    }

    #[test]
    fn describe_freezing_heat_heavy_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_contains(&string, "Freezing heavy");
    }

    #[test]
    fn describe_freezing_heat_shower_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Freezing,
        };
        let string = describe_weather_kind(&WeatherKind::Precipitation(precipitation));
        assert_contains(&string, "Freezing shower");
    }

    #[test]
    fn describe_thunderstorm() {
        let string = describe_weather_kind(&WeatherKind::Thunderstorm);
        assert_starts_with(&string, "Thunderstorm is raging");
    }
}
