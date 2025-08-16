use crate::types::WeatherReport;
use crate::types::weather::*;

#[allow(
    clippy::format_in_format_args,
    reason = "Fits in one line. Executed only once, so performance is not a concern."
)]
pub fn format(report: &WeatherReport) -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        format!("Weather: {}", describe_kind(&report.kind)),
        format!("Temperature: {:.1}", report.temperature),
        format!("Cloud coverage: {}", report.cloud_coverage),
        format!("Humidity: {}", report.humidity),
        format!("Wind: {}", describe_wind(&report.wind)),
        format!("Pressure: {}", report.pressure)
    )
}

fn describe_kind(kind: &Kind) -> String {
    match kind {
        Kind::Clouds(clouds) => match clouds {
            Clouds::Clear => "clear sky".into(),
            Clouds::Light => "light clouds".into(),
            Clouds::Moderate => "cloudy".into(),
            Clouds::Dense => "overcast sky".into(),
        },
        Kind::Fog(fog) => match fog {
            Fog::Normal => "fog".into(),
            Fog::Rime => "rime fog".into(),
        },
        Kind::Precipitation(precipitation) => {
            let kind_desc = match precipitation.kind {
                PrecipitationKind::Rain => "rain",
                PrecipitationKind::Snow => "snow",
            };
            let intensity_desc = match precipitation.intensity {
                PrecipitationIntensity::Light => "light",
                PrecipitationIntensity::Moderate => "moderate",
                PrecipitationIntensity::Heavy => "heavy",
                PrecipitationIntensity::Shower => "shower",
            };
            if precipitation.heat == PrecipitationHeat::Freezing {
                format!("freezing {intensity_desc} {kind_desc}")
            } else {
                format!("{intensity_desc} {kind_desc}")
            }
        }
        Kind::Thunderstorm => "thunderstorm".into(),
    }
}

fn describe_wind(wind: &Wind) -> String {
    format!(
        "{:.1}, {} ({})",
        wind.speed,
        wind.direction,
        wind.direction.to_cardinal_direction().to_symbol()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Coordinates;
    use crate::types::units::*;

    #[test]
    fn describe_clouds_kind_values() {
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Clear)), "clear sky");
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Light)), "light clouds");
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Moderate)), "cloudy");
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Dense)), "overcast sky");
    }

    #[test]
    fn describe_fog_kind_values() {
        assert_eq!(describe_kind(&Kind::Fog(Fog::Normal)), "fog");
        assert_eq!(describe_kind(&Kind::Fog(Fog::Rime)), "rime fog");
    }

    #[test]
    fn describe_precipitation_kind_values() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            describe_kind(&Kind::Precipitation(precipitation)),
            "light rain"
        );
        let precipitation = Precipitation {
            kind: PrecipitationKind::Snow,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            describe_kind(&Kind::Precipitation(precipitation)),
            "light snow"
        );
    }

    #[test]
    fn describe_precipitation_intensity_values() {
        let make_rain = |intensity| {
            let precipitation = Precipitation {
                kind: PrecipitationKind::Rain,
                intensity,
                heat: PrecipitationHeat::Normal,
            };
            Kind::Precipitation(precipitation)
        };
        assert_eq!(
            describe_kind(&make_rain(PrecipitationIntensity::Light)),
            "light rain"
        );
        assert_eq!(
            describe_kind(&make_rain(PrecipitationIntensity::Moderate)),
            "moderate rain"
        );
        assert_eq!(
            describe_kind(&make_rain(PrecipitationIntensity::Heavy)),
            "heavy rain"
        );
        assert_eq!(
            describe_kind(&make_rain(PrecipitationIntensity::Shower)),
            "shower rain"
        );
    }

    #[test]
    fn describe_precipitation_heat_values() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            describe_kind(&Kind::Precipitation(precipitation)),
            "light rain"
        );
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Freezing,
        };
        assert_eq!(
            describe_kind(&Kind::Precipitation(precipitation)),
            "freezing light rain"
        );
    }

    #[test]
    fn describe_thunderstorm_values() {
        assert_eq!(describe_kind(&Kind::Thunderstorm), "thunderstorm");
    }

    #[test]
    fn describe_wind_values() {
        let wind = Wind {
            speed: Speed::new_meters_per_second(42.5),
            direction: Azimuth::from(200.2),
        };
        assert_eq!(describe_wind(&wind), "42.5 m/s, 200.2° (S)");
    }

    #[test]
    fn describe_all_parameters() {
        let report = WeatherReport {
            coordinates: Coordinates {
                latitude: 1.2,
                longitude: 3.4,
            },
            kind: Kind::Clouds(Clouds::Light),
            temperature: Temperature::new_celsius(22.4),
            cloud_coverage: Percentage::from(43),
            humidity: Percentage::from(81),
            wind: Wind {
                speed: Speed::new_meters_per_second(1.07),
                direction: Azimuth::from(155.5),
            },
            pressure: Hectopascal::from(1009.3),
        };
        let result = format(&report);
        let expected = "Weather: light clouds\n\
             Temperature: 22.4°C\n\
             Cloud coverage: 43%\n\
             Humidity: 81%\n\
             Wind: 1.1 m/s, 155.5° (SE)\n\
             Pressure: 1009.3 hPa";
        assert_eq!(result, expected);
    }
}
