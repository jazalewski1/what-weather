use crate::types::WeatherReport;
use crate::types::weather::*;

fn format(report: &WeatherReport) -> String {
    let kind_desc = describe_kind(&report.kind);
    let temperature_desc = describe_temperature(report.temperature);
    let clouds_desc = describe_cloud_coverage(report.cloud_coverage);
    let humidity_desc = describe_humidity(report.humidity);
    let wind_desc = describe_wind(&report.wind);
    let pressure_desc = describe_pressure(report.pressure);

    #[allow(
        clippy::format_in_format_args,
        reason = "Fits in one line. Executed only once, so performance is not a concern."
    )]
    {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            format!("Weather: {kind_desc}"),
            format!("Temperature: {temperature_desc}"),
            format!("Cloud coverage: {clouds_desc}"),
            format!("Humidity: {humidity_desc}"),
            format!("Wind: {wind_desc}"),
            format!("Pressure: {pressure_desc}")
        )
    }
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

fn describe_temperature(value: Temperature) -> String {
    format!("{value:.1}°C")
}

fn describe_cloud_coverage(value: CloudCoverage) -> String {
    format!("{value}%")
}

fn describe_humidity(value: Humidity) -> String {
    format!("{value}%")
}

fn describe_wind(wind: &Wind) -> String {
    let direction_symbol = if wind.direction <= 22.5 {
        "N"
    } else if wind.direction <= 67.5 {
        "NE"
    } else if wind.direction <= 112.5 {
        "E"
    } else if wind.direction <= 157.5 {
        "SE"
    } else if wind.direction <= 202.5 {
        "S"
    } else if wind.direction <= 247.5 {
        "SW"
    } else if wind.direction <= 292.5 {
        "W"
    } else if wind.direction <= 337.5 {
        "NW"
    } else {
        "N"
    };
    format!(
        "{} m/s, {}° ({direction_symbol})",
        wind.speed, wind.direction
    )
}

fn describe_pressure(value: Pressure) -> String {
    format!("{value:.1} hPa")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Coordinates;

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
    fn describe_temperature_values() {
        assert_eq!(describe_temperature(-13.5), "-13.5°C");
        assert_eq!(describe_temperature(27.0), "27.0°C");
        assert_eq!(describe_temperature(1.97), "2.0°C");
    }

    #[test]
    fn describe_clouds_coverage_values() {
        assert_eq!(describe_cloud_coverage(0), "0%");
        assert_eq!(describe_cloud_coverage(43), "43%");
        assert_eq!(describe_cloud_coverage(100), "100%");
    }

    #[test]
    fn describe_humidity_values() {
        assert_eq!(describe_humidity(0), "0%");
        assert_eq!(describe_humidity(66), "66%");
        assert_eq!(describe_humidity(100), "100%");
    }

    #[test]
    fn describe_wind_values() {
        let wind = Wind {
            speed: 42.5,
            direction: 200.2,
        };
        assert_eq!(describe_wind(&wind), "42.5 m/s, 200.2° (S)");
    }

    #[test]
    fn describe_pressure_values() {
        assert_eq!(describe_pressure(990.2), "990.2 hPa");
        assert_eq!(describe_pressure(1015.0), "1015.0 hPa");
    }

    #[test]
    fn describe_all_parameters() {
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
        let result = format(&report);
        let expected = "Weather: light clouds\n\
             Temperature: 22.4°C\n\
             Cloud coverage: 43%\n\
             Humidity: 81%\n\
             Wind: 1.07 m/s, 155.5° (SE)\n\
             Pressure: 1009.3 hPa";
        assert_eq!(result, expected);
    }
}
