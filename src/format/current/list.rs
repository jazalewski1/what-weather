use crate::format::common::list_builder::write_param;
use crate::format::common::list_format::describe_kind;
use crate::types::report::CurrentPartialReport;
use crate::types::weather::*;

pub fn describe(report: &CurrentPartialReport) -> String {
    let mut result = String::default();

    write_param(
        &mut result,
        "Coordinates",
        format!("{:.5}", report.coordinates),
    );
    if let Some(kind) = report.kind {
        write_param(&mut result, "Weather", describe_kind(&kind));
    }
    if let Some(temperature) = report.temperature {
        write_param(&mut result, "Temperature", format!("{temperature:.1}"));
    }
    if let Some(coverage) = report.cloud_coverage {
        write_param(&mut result, "Cloud coverage", format!("{coverage}"));
    }
    if let Some(humidity) = report.humidity {
        write_param(&mut result, "Humidity", format!("{humidity}"));
    }
    if let Some(wind) = &report.wind {
        write_param(&mut result, "Wind", describe_wind(wind));
    }
    if let Some(pressure) = report.pressure {
        write_param(&mut result, "Pressure", format!("{pressure}"));
    }
    result
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
    use crate::types::units::*;

    #[test]
    fn describes_values_of_clouds_kind() {
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Clear)), "clear sky");
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Light)), "light clouds");
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Moderate)), "cloudy");
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Dense)), "overcast sky");
    }

    #[test]
    fn describes_values_of_fog_kind() {
        assert_eq!(describe_kind(&Kind::Fog(Fog::Normal)), "fog");
        assert_eq!(describe_kind(&Kind::Fog(Fog::Rime)), "rime fog");
    }

    #[test]
    fn describes_values_of_precipitation_kind() {
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
    fn describes_values_of_precipitation_intensity() {
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
    fn describes_values_of_precipitation_heat() {
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
    fn describes_value_of_thunderstorm() {
        assert_eq!(describe_kind(&Kind::Thunderstorm), "thunderstorm");
    }

    #[test]
    fn describes_values_of_wind() {
        let wind = Wind {
            speed: Speed::new_meters_per_second(42.5),
            direction: Azimuth::from(200.2),
        };
        assert_eq!(describe_wind(&wind), "42.5 m/s, 200.2° (S)");
    }

    #[test]
    fn describes_all_attributes() {
        let coordinates = Coordinates::new(1.2345, 67.89);
        let report = CurrentPartialReport {
            coordinates,
            kind: Some(Kind::Clouds(Clouds::Light)),
            temperature: Some(Temperature::new_celsius(22.4)),
            cloud_coverage: Some(Percentage::from(43)),
            humidity: Some(Percentage::from(81)),
            wind: Some(Wind {
                speed: Speed::new_meters_per_second(1.07),
                direction: Azimuth::from(155.5),
            }),
            pressure: Some(Pressure::new_hpa(1009.3)),
        };
        let result = describe(&report);
        let expected = "Coordinates: 1.23450°, 67.89000°\n\
            Weather: light clouds\n\
            Temperature: 22.4°C\n\
            Cloud coverage: 43%\n\
            Humidity: 81%\n\
            Wind: 1.1 m/s, 155.5° (SE)\n\
            Pressure: 1009.3 hPa\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn describes_only_selected_attributes() {
        let coordinates = Coordinates::new(1.2345, 67.89);
        let report = CurrentPartialReport {
            coordinates,
            kind: None,
            temperature: Some(Temperature::new_celsius(22.4)),
            cloud_coverage: None,
            humidity: Some(Percentage::from(81)),
            wind: Some(Wind {
                speed: Speed::new_meters_per_second(1.07),
                direction: Azimuth::from(155.5),
            }),
            pressure: None,
        };
        let result = describe(&report);
        let expected = "Coordinates: 1.23450°, 67.89000°\n\
            Temperature: 22.4°C\n\
            Humidity: 81%\n\
            Wind: 1.1 m/s, 155.5° (SE)\n";
        assert_eq!(result, expected);
    }
}
