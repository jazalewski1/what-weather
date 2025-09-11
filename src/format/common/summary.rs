use crate::types::units::*;
use crate::types::weather::*;

#[derive(Debug, PartialEq, Eq)]
pub enum KindDescription {
    Clouds { sky_adjective: String },
    Fog { description: String },
    Precipitation { description: String },
    Thunderstorm { description: String },
}

pub fn prepare_kind_description(kind: &Kind) -> KindDescription {
    match kind {
        Kind::Clouds(clouds) => {
            let adj = match clouds {
                Clouds::Clear => "clear",
                Clouds::Light => "mostly clear",
                Clouds::Moderate => "cloudy",
                Clouds::Dense => "overcast",
            };
            KindDescription::Clouds {
                sky_adjective: adj.into(),
            }
        }
        Kind::Fog(fog) => {
            let desc = match fog {
                Fog::Normal => "fog",
                Fog::Rime => "rime fog",
            };
            KindDescription::Fog {
                description: desc.into(),
            }
        }
        Kind::Precipitation(precipitation) => {
            let intensity_desc = match precipitation.intensity {
                PrecipitationIntensity::Light => "light",
                PrecipitationIntensity::Moderate => "moderate",
                PrecipitationIntensity::Heavy => "heavy",
                PrecipitationIntensity::Shower => "shower",
            };
            let kind_desc = match precipitation.kind {
                PrecipitationKind::Rain => "rain",
                PrecipitationKind::Snow => "snow",
            };
            let description = match precipitation.heat {
                PrecipitationHeat::Normal => format!("{intensity_desc} {kind_desc}"),
                PrecipitationHeat::Freezing => format!("freezing {intensity_desc} {kind_desc}"),
            };
            KindDescription::Precipitation { description }
        }
        Kind::Thunderstorm => KindDescription::Thunderstorm {
            description: "thunderstorm".into(),
        },
    }
}

pub fn describe_temperature_adjective(temperature: &Temperature) -> String {
    const ADJECTIVES: [&str; 6] = ["freezing", "cold", "cool", "warm", "hot", "very hot"];

    fn find_adjective(value: f32, thresholds: &[f32; 5]) -> String {
        let index = thresholds
            .iter()
            .position(|t| value <= *t)
            .unwrap_or(ADJECTIVES.len() - 1);
        ADJECTIVES[index].to_string()
    }

    match temperature {
        Temperature::Celsius(Celsius { degrees }) => {
            let thresholds = [0.0, 7.0, 15.0, 26.0, 35.0];
            find_adjective(degrees.raw(), &thresholds)
        }
        Temperature::Fahrenheit(Fahrenheit { degrees }) => {
            let thresholds = [32.0, 44.6, 59.0, 78.8, 95.0];
            find_adjective(degrees.raw(), &thresholds)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HumidityLevel {
    VeryDry,
    Dry,
    Humid,
    VeryHumid,
    Heavy,
}

pub fn prepare_humidity_level(percentage: &Percentage) -> HumidityLevel {
    if percentage.value <= 15 {
        HumidityLevel::VeryDry
    } else if percentage.value <= 30 {
        HumidityLevel::Dry
    } else if percentage.value <= 60 {
        HumidityLevel::Humid
    } else if percentage.value <= 85 {
        HumidityLevel::VeryHumid
    } else {
        HumidityLevel::Heavy
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum WindDescription {
    NoWind,
    Wind { description: String },
}

pub fn prepare_wind_description(speed: &Speed, direction: &Azimuth) -> WindDescription {
    let direction_definition = direction.to_cardinal_direction().to_name();
    match *speed {
        Speed::MetersPerSecond(MetersPerSecond { value }) => {
            if value <= 0.2 {
                return WindDescription::NoWind;
            }
            let description = if value <= 3.3 {
                format!("gentle {direction_definition} breeze")
            } else if value <= 8.0 {
                format!("{direction_definition} wind")
            } else if value <= 13.8 {
                format!("strong {direction_definition} wind")
            } else {
                format!("very strong {direction_definition} wind")
            };
            WindDescription::Wind { description }
        }
    }
}

pub fn describe_hectopascal_adjective(hpa: &Hectopascal) -> String {
    if hpa.value <= 1000.0 {
        "Very low"
    } else if hpa.value <= 1010.0 {
        "Low"
    } else if hpa.value <= 1020.0 {
        "Normal"
    } else if hpa.value <= 1030.0 {
        "High"
    } else {
        "Very high"
    }
    .into()
}

pub fn describe_pressure_adjective(pressure: &Pressure) -> String {
    match pressure {
        Pressure::Hpa(hpa) => describe_hectopascal_adjective(hpa),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_temperature_in_celsius_as_adjectives() {
        let describe = |value| describe_temperature_adjective(&Temperature::new_celsius(value));

        assert_eq!(describe(-3.0), "freezing");
        assert_eq!(describe(-0.1), "freezing");
        assert_eq!(describe(0.0), "freezing");

        assert_eq!(describe(1.0), "cold");
        assert_eq!(describe(4.5), "cold");
        assert_eq!(describe(7.0), "cold");

        assert_eq!(describe(7.1), "cool");
        assert_eq!(describe(13.7), "cool");
        assert_eq!(describe(15.0), "cool");

        assert_eq!(describe(15.1), "warm");
        assert_eq!(describe(20.0), "warm");
        assert_eq!(describe(26.0), "warm");

        assert_eq!(describe(26.1), "hot");
        assert_eq!(describe(29.9), "hot");
        assert_eq!(describe(35.0), "hot");

        assert_eq!(describe(35.1), "very hot");
        assert_eq!(describe(40.2), "very hot");
    }

    #[test]
    fn describes_temperature_in_fahrenheit_as_adjectives() {
        let describe = |value| describe_temperature_adjective(&Temperature::new_fahrenheit(value));

        assert_eq!(describe(0.0), "freezing");
        assert_eq!(describe(32.0), "freezing");

        assert_eq!(describe(32.1), "cold");
        assert_eq!(describe(40.0), "cold");
        assert_eq!(describe(44.6), "cold");

        assert_eq!(describe(44.7), "cool");
        assert_eq!(describe(50.0), "cool");
        assert_eq!(describe(59.0), "cool");

        assert_eq!(describe(59.1), "warm");
        assert_eq!(describe(65.0), "warm");
        assert_eq!(describe(78.8), "warm");

        assert_eq!(describe(78.9), "hot");
        assert_eq!(describe(85.0), "hot");
        assert_eq!(describe(95.0), "hot");

        assert_eq!(describe(95.1), "very hot");
        assert_eq!(describe(100.0), "very hot");
    }

    #[test]
    fn prepares_kind_descriptions_for_clouds() {
        let make_expected = |adj: &str| KindDescription::Clouds {
            sky_adjective: adj.into(),
        };

        let desc = prepare_kind_description(&Kind::Clouds(Clouds::Clear));
        assert_eq!(desc, make_expected("clear"));
        let desc = prepare_kind_description(&Kind::Clouds(Clouds::Light));
        assert_eq!(desc, make_expected("mostly clear"));
        let desc = prepare_kind_description(&Kind::Clouds(Clouds::Moderate));
        assert_eq!(desc, make_expected("cloudy"));
        let desc = prepare_kind_description(&Kind::Clouds(Clouds::Dense));
        assert_eq!(desc, make_expected("overcast"));
    }

    #[test]
    fn prepares_kind_descriptions_for_fog() {
        let kind = Kind::Fog(Fog::Normal);
        let desc = prepare_kind_description(&kind);
        assert_eq!(
            desc,
            KindDescription::Fog {
                description: "fog".into()
            }
        );

        let kind = Kind::Fog(Fog::Rime);
        let desc = prepare_kind_description(&kind);
        assert_eq!(
            desc,
            KindDescription::Fog {
                description: "rime fog".into()
            }
        );
    }

    #[test]
    fn prepares_kind_descriptions_for_precipitation_kind() {
        let prepare = |kind| {
            let kind = Kind::Precipitation(Precipitation {
                kind,
                intensity: PrecipitationIntensity::Light,
                heat: PrecipitationHeat::Normal,
            });
            prepare_kind_description(&kind)
        };
        let make_expected = |desc: &str| KindDescription::Precipitation {
            description: desc.into(),
        };

        let desc = prepare(PrecipitationKind::Rain);
        assert_eq!(desc, make_expected("light rain"));
        let desc = prepare(PrecipitationKind::Snow);
        assert_eq!(desc, make_expected("light snow"));
    }

    #[test]
    fn prepares_kind_descriptions_for_precipitation_intensity() {
        let prepare = |intensity| {
            let kind = Kind::Precipitation(Precipitation {
                kind: PrecipitationKind::Rain,
                intensity,
                heat: PrecipitationHeat::Normal,
            });
            prepare_kind_description(&kind)
        };
        let make_expected = |desc: &str| KindDescription::Precipitation {
            description: desc.into(),
        };

        let desc = prepare(PrecipitationIntensity::Light);
        assert_eq!(desc, make_expected("light rain"));
        let desc = prepare(PrecipitationIntensity::Moderate);
        assert_eq!(desc, make_expected("moderate rain"));
        let desc = prepare(PrecipitationIntensity::Heavy);
        assert_eq!(desc, make_expected("heavy rain"));
        let desc = prepare(PrecipitationIntensity::Shower);
        assert_eq!(desc, make_expected("shower rain"));
    }

    #[test]
    fn prepares_kind_descriptions_for_precipitation_heat() {
        let prepare = |heat| {
            let kind = Kind::Precipitation(Precipitation {
                kind: PrecipitationKind::Rain,
                intensity: PrecipitationIntensity::Moderate,
                heat,
            });
            prepare_kind_description(&kind)
        };
        let make_expected = |desc: &str| KindDescription::Precipitation {
            description: desc.into(),
        };

        let desc = prepare(PrecipitationHeat::Normal);
        assert_eq!(desc, make_expected("moderate rain"));
        let desc = prepare(PrecipitationHeat::Freezing);
        assert_eq!(desc, make_expected("freezing moderate rain"));
    }

    #[test]
    fn prepares_kind_description_for_thunderstorm() {
        let kind = Kind::Thunderstorm;
        let desc = prepare_kind_description(&kind);
        assert_eq!(
            desc,
            KindDescription::Thunderstorm {
                description: "thunderstorm".into()
            }
        );
    }

    #[test]
    fn prepares_humidity_level_from_percentage() {
        let prepare = |value| prepare_humidity_level(&Percentage::from(value));

        assert_eq!(prepare(0), HumidityLevel::VeryDry);
        assert_eq!(prepare(15), HumidityLevel::VeryDry);

        assert_eq!(prepare(16), HumidityLevel::Dry);
        assert_eq!(prepare(30), HumidityLevel::Dry);

        assert_eq!(prepare(31), HumidityLevel::Humid);
        assert_eq!(prepare(60), HumidityLevel::Humid);

        assert_eq!(prepare(61), HumidityLevel::VeryHumid);
        assert_eq!(prepare(85), HumidityLevel::VeryHumid);

        assert_eq!(prepare(86), HumidityLevel::Heavy);
        assert_eq!(prepare(100), HumidityLevel::Heavy);
    }

    #[test]
    fn prepares_wind_description_from_params() {
        let prepare = |speed_value| {
            prepare_wind_description(
                &Speed::new_meters_per_second(speed_value),
                &Azimuth::from(89.5),
            )
        };
        let make_wind_desc = |desc: &str| WindDescription::Wind {
            description: desc.into(),
        };

        assert_eq!(prepare(0.0), WindDescription::NoWind);
        assert_eq!(prepare(0.2), WindDescription::NoWind);

        assert_eq!(prepare(0.21), make_wind_desc("gentle east breeze"));
        assert_eq!(prepare(2.9), make_wind_desc("gentle east breeze"));
        assert_eq!(prepare(3.3), make_wind_desc("gentle east breeze"));

        assert_eq!(prepare(3.31), make_wind_desc("east wind"));
        assert_eq!(prepare(5.57), make_wind_desc("east wind"));
        assert_eq!(prepare(8.0), make_wind_desc("east wind"));

        assert_eq!(prepare(8.01), make_wind_desc("strong east wind"));
        assert_eq!(prepare(10.3), make_wind_desc("strong east wind"));
        assert_eq!(prepare(13.8), make_wind_desc("strong east wind"));

        assert_eq!(prepare(13.81), make_wind_desc("very strong east wind"));
        assert_eq!(prepare(15.0), make_wind_desc("very strong east wind"));
    }

    #[test]
    fn describes_hectopascal_adjective() {
        let describe = |value| describe_pressure_adjective(&Pressure::new_hpa(value));

        assert_eq!(describe(995.0), "Very low");
        assert_eq!(describe(1000.0), "Very low");

        assert_eq!(describe(1000.1), "Low");
        assert_eq!(describe(1005.3), "Low");
        assert_eq!(describe(1010.0), "Low");

        assert_eq!(describe(1010.1), "Normal");
        assert_eq!(describe(1018.7), "Normal");
        assert_eq!(describe(1020.0), "Normal");

        assert_eq!(describe(1020.1), "High");
        assert_eq!(describe(1026.1), "High");
        assert_eq!(describe(1030.0), "High");

        assert_eq!(describe(1030.1), "Very high");
        assert_eq!(describe(1035.0), "Very high");
    }
}
