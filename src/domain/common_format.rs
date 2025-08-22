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
    match temperature {
        Temperature::Celsius(Celsius { value }) => {
            if *value <= 0.0 {
                "freezing"
            } else if *value <= 10.0 {
                "cold"
            } else if *value <= 17.0 {
                "cool"
            } else if *value <= 24.0 {
                "warm"
            } else if *value <= 35.0 {
                "hot"
            } else {
                "very hot"
            }
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_values_of_temperatures_as_adjectives() {
        let describe = |value| describe_temperature_adjective(&Temperature::new_celsius(value));

        assert_eq!(describe(-3.0), "freezing");
        assert_eq!(describe(-0.1), "freezing");
        assert_eq!(describe(0.0), "freezing");

        assert_eq!(describe(1.0), "cold");
        assert_eq!(describe(4.5), "cold");
        assert_eq!(describe(10.0), "cold");

        assert_eq!(describe(10.1), "cool");
        assert_eq!(describe(13.7), "cool");
        assert_eq!(describe(17.0), "cool");

        assert_eq!(describe(17.1), "warm");
        assert_eq!(describe(20.0), "warm");
        assert_eq!(describe(24.0), "warm");

        assert_eq!(describe(24.1), "hot");
        assert_eq!(describe(29.9), "hot");
        assert_eq!(describe(35.0), "hot");

        assert_eq!(describe(35.1), "very hot");
        assert_eq!(describe(40.2), "very hot");
    }

    #[test]
    fn prepares_kind_descriptions_for_clouds() {
        let kind = Kind::Clouds(Clouds::Clear);
        let desc = prepare_kind_description(&kind);
        assert_eq!(
            desc,
            KindDescription::Clouds {
                sky_adjective: "clear".into()
            }
        );

        let kind = Kind::Clouds(Clouds::Light);
        let desc = prepare_kind_description(&kind);
        assert_eq!(
            desc,
            KindDescription::Clouds {
                sky_adjective: "mostly clear".into()
            }
        );

        let kind = Kind::Clouds(Clouds::Moderate);
        let desc = prepare_kind_description(&kind);
        assert_eq!(
            desc,
            KindDescription::Clouds {
                sky_adjective: "cloudy".into()
            }
        );

        let kind = Kind::Clouds(Clouds::Dense);
        let desc = prepare_kind_description(&kind);
        assert_eq!(
            desc,
            KindDescription::Clouds {
                sky_adjective: "overcast".into()
            }
        );
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

        let desc = prepare(PrecipitationKind::Rain);
        assert_eq!(
            desc,
            KindDescription::Precipitation {
                description: "light rain".into()
            }
        );

        let desc = prepare(PrecipitationKind::Snow);
        assert_eq!(
            desc,
            KindDescription::Precipitation {
                description: "light snow".into()
            }
        );
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

        let desc = prepare(PrecipitationIntensity::Light);
        assert_eq!(
            desc,
            KindDescription::Precipitation {
                description: "light rain".into()
            }
        );

        let desc = prepare(PrecipitationIntensity::Moderate);
        assert_eq!(
            desc,
            KindDescription::Precipitation {
                description: "moderate rain".into()
            }
        );

        let desc = prepare(PrecipitationIntensity::Heavy);
        assert_eq!(
            desc,
            KindDescription::Precipitation {
                description: "heavy rain".into()
            }
        );

        let desc = prepare(PrecipitationIntensity::Shower);
        assert_eq!(
            desc,
            KindDescription::Precipitation {
                description: "shower rain".into()
            }
        );
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

        let desc = prepare(PrecipitationHeat::Normal);
        assert_eq!(
            desc,
            KindDescription::Precipitation {
                description: "moderate rain".into()
            }
        );

        let desc = prepare(PrecipitationHeat::Freezing);
        assert_eq!(
            desc,
            KindDescription::Precipitation {
                description: "freezing moderate rain".into()
            }
        );
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
}
