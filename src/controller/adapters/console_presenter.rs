use crate::domain::port::Presenter;
use crate::domain::types::*;

pub struct ConsolePresenter;

fn describe_to_string(report: &WeatherReport) -> String {
    let text: String = match &report.kind {
        WeatherKind::Clouds(clouds) => match clouds {
            Clouds::Clear => "The sky is clear".into(),
            Clouds::Light => "The sky is mostly clear".into(),
            Clouds::Moderate => "The sky is moderately cloudy".into(),
            Clouds::Dense => "The sky is overcast".into(),
        },
        WeatherKind::Fog(fog) => match fog {
            Fog::Normal => "Fog is present".into(),
            Fog::Rime => "Rime fog covers the area".into(),
        },
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
        WeatherKind::Thunderstorm => "Thunderstorm is raging ".into(),
    };
    format!(
        "{text} at latitude {:.5} and longitude {:.5}.",
        report.coordinates.latitude, report.coordinates.longitude
    )
}

impl Presenter for ConsolePresenter {
    fn display(&self, report: &WeatherReport) {
        let text = describe_to_string(report);
        println!("{text}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COORDINATES: Coordinates = Coordinates {
        latitude: 1.2,
        longitude: 3.4,
    };

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

    fn assert_ends_with(string: &str, expected_end: &str) {
        assert!(
            string.ends_with(expected_end),
            "\"{string}\"\nexpected to end with\n\"{expected_end}\""
        );
    }

    #[test]
    fn describe_coordinates() {
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Clouds(Clouds::Clear),
        };
        let string = describe_to_string(&report);
        assert_ends_with(&string, "at latitude 1.20000 and longitude 3.40000.");
    }

    #[test]
    fn describe_clear_sky() {
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Clouds(Clouds::Clear),
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "The sky is clear ");
    }

    #[test]
    fn describe_lightly_cloudy_sky() {
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Clouds(Clouds::Light),
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "The sky is mostly clear ");
    }

    #[test]
    fn describe_moderately_cloudy_sky() {
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Clouds(Clouds::Moderate),
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "The sky is moderately cloudy ");
    }

    #[test]
    fn describe_densely_cloudy_sky() {
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Clouds(Clouds::Dense),
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "The sky is overcast ");
    }

    #[test]
    fn describe_normal_fog() {
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Fog(Fog::Normal),
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "Fog is present ");
    }

    #[test]
    fn describe_rime_fog() {
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Fog(Fog::Rime),
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "Rime fog covers the area ");
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
                let report = WeatherReport {
                    coordinates: COORDINATES,
                    kind: WeatherKind::Precipitation(precipitation),
                };
                let string = describe_to_string(&report);
                assert_contains(&string, " rain is falling ");
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
                let report = WeatherReport {
                    coordinates: COORDINATES,
                    kind: WeatherKind::Precipitation(precipitation),
                };
                let string = describe_to_string(&report);
                assert_contains(&string, " snow is falling ");
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
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Precipitation(precipitation),
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "Light ");
    }

    #[test]
    fn describe_moderate_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        };
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Precipitation(precipitation),
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "Moderate ");
    }

    #[test]
    fn describe_heavy_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Normal,
        };
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Precipitation(precipitation),
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "Heavy ");
    }

    #[test]
    fn describe_shower_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Normal,
        };
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Precipitation(precipitation),
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "Shower ");
    }

    #[test]
    fn describe_freezing_heat_light_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Freezing,
        };
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Precipitation(precipitation),
        };
        let string = describe_to_string(&report);
        assert_contains(&string, "Freezing light rain is falling ");
    }

    #[test]
    fn describe_freezing_heat_moderate_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Freezing,
        };
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Precipitation(precipitation),
        };
        let string = describe_to_string(&report);
        assert_contains(&string, "Freezing moderate rain is falling ");
    }

    #[test]
    fn describe_freezing_heat_heavy_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Freezing,
        };
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Precipitation(precipitation),
        };
        let string = describe_to_string(&report);
        assert_contains(&string, "Freezing heavy rain is falling ");
    }

    #[test]
    fn describe_freezing_heat_shower_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Freezing,
        };
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Precipitation(precipitation),
        };
        let string = describe_to_string(&report);
        assert_contains(&string, "Freezing shower rain is falling ");
    }

    #[test]
    fn describe_thunderstorm() {
        let report = WeatherReport {
            coordinates: COORDINATES,
            kind: WeatherKind::Thunderstorm,
        };
        let string = describe_to_string(&report);
        assert_starts_with(&string, "Thunderstorm is raging ");
    }
}
