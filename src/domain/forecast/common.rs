use crate::domain::common_format::*;
use crate::types::units::*;
use crate::types::weather::*;

pub fn describe_forecast(
    time_desc: &str,
    temperature_range: &TemperatureRange,
    kind: &Kind,
    cloud_coverage: &PercentageRange,
    humidity: &PercentageRange,
    wind: &WindScope,
    pressure_range: &PressureRange,
) -> String {
    let temperature_desc = describe_temperature_range(temperature_range);
    let kind_desc = describe_kind(kind);
    let cloud_coverage_desc = describe_cloud_coverage_range(cloud_coverage);
    let humidity_desc = describe_humidity_range(humidity);
    let wind_desc = describe_wind_scope(wind);
    let pressure_desc = describe_pressure_range(pressure_range);
    #[allow(clippy::uninlined_format_args)]
    {
        format!(
            "{} {}.\n{} and {}.\n{} with {}.\n{}.\n",
            time_desc,
            temperature_desc,
            kind_desc,
            cloud_coverage_desc,
            humidity_desc,
            wind_desc,
            pressure_desc,
        )
    }
}

fn describe_kind(kind: &Kind) -> String {
    let desc = prepare_kind_description(kind);
    match desc {
        KindDescription::Clouds { sky_adjective } => format!("The sky will be {sky_adjective}"),
        KindDescription::Fog { description } => {
            format!("A {description} will be covering the area")
        }
        KindDescription::Precipitation { description } => {
            format!("There will be {description} falling")
        }
        KindDescription::Thunderstorm { description } => format!("A {description} will be raging"),
    }
}

fn describe_temperature_range(temperature_range: &TemperatureRange) -> String {
    match temperature_range {
        TemperatureRange::Celsius { min, max } => {
            let adjective = describe_temperature_adjective(&Temperature::Celsius(*max));
            format!("it will be {adjective} with temperatures starting at {min} and reaching {max}")
        }
    }
}

fn describe_cloud_coverage_range(range: &PercentageRange) -> String {
    format!(
        "clouds will cover from {} to {} of the sky",
        range.min, range.max
    )
}

fn describe_humidity_range(range: &PercentageRange) -> String {
    let make_without_humidity = |adjective| {
        format!(
            "The air will be {adjective} at {} to {}",
            range.min, range.max
        )
    };
    let make_with_humidity = |adjective| format!("{} humidity", make_without_humidity(adjective));
    let level = prepare_humidity_level(&range.max);
    match level {
        HumidityLevel::VeryDry => make_with_humidity("very dry"),
        HumidityLevel::Dry => make_with_humidity("dry"),
        HumidityLevel::Humid => make_without_humidity("humid"),
        HumidityLevel::VeryHumid => make_without_humidity("very humid"),
        HumidityLevel::Heavy => make_with_humidity("heavy"),
    }
}

fn describe_wind_scope(scope: &WindScope) -> String {
    let max_speed = match scope.speed_range {
        SpeedRange::MetersPerSecond { max, .. } => Speed::MetersPerSecond(max),
    };
    let desc = prepare_wind_description(&max_speed, &scope.dominant_direction);
    match desc {
        WindDescription::NoWind => "mostly no wind".into(),
        WindDescription::Wind { description } => {
            format!("mostly {description} blowing at maximum {max_speed}")
        }
    }
}

fn describe_pressure_range(pressure_range: &PressureRange) -> String {
    let PressureRange { min, max } = pressure_range;
    let adjective = describe_pressure_adjective(max);
    format!("{adjective} pressure will reach {min:.1} at lowest up to {max:.1}",)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_temperature_range() {
        let range = TemperatureRange::new_celsius(15.1, 33.3);
        let result = describe_temperature_range(&range);
        let expected = "it will be hot with temperatures starting at 15.1°C and reaching 33.3°C";
        assert_eq!(result, expected);
    }

    #[test]
    fn describes_cloud_kind() {
        let kind = Kind::Clouds(Clouds::Dense);
        let result = describe_kind(&kind);
        assert_eq!(result, "The sky will be overcast");
    }

    #[test]
    fn describes_fog_kind() {
        let kind = Kind::Fog(Fog::Normal);
        let result = describe_kind(&kind);
        assert_eq!(result, "A fog will be covering the area");
    }

    #[test]
    fn describes_precipitation_kind() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        };
        let kind = Kind::Precipitation(precipitation);
        let result = describe_kind(&kind);
        assert_eq!(result, "There will be moderate rain falling");
    }

    #[test]
    fn describes_thunderstorm_kind() {
        let kind = Kind::Thunderstorm;
        let result = describe_kind(&kind);
        assert_eq!(result, "A thunderstorm will be raging");
    }

    #[test]
    fn describes_cloud_coverage_ranges() {
        let range = PercentageRange::new(26, 57);
        let result = describe_cloud_coverage_range(&range);
        assert_eq!(result, "clouds will cover from 26% to 57% of the sky");
    }

    #[test]
    fn describes_humidity_ranges() {
        let describe = |min, max| {
            let range = PercentageRange::new(min, max);
            describe_humidity_range(&range)
        };

        assert_eq!(
            describe(0, 15),
            "The air will be very dry at 0% to 15% humidity"
        );
        assert_eq!(
            describe(15, 30),
            "The air will be dry at 15% to 30% humidity"
        );
        assert_eq!(describe(30, 60), "The air will be humid at 30% to 60%");
        assert_eq!(describe(60, 85), "The air will be very humid at 60% to 85%");
        assert_eq!(
            describe(85, 100),
            "The air will be heavy at 85% to 100% humidity"
        );
    }

    #[test]
    fn describes_wind_scope() {
        let wind = WindScope {
            speed_range: SpeedRange::new_meters_per_second(0.05, 0.15),
            dominant_direction: Azimuth::from(273.3),
        };
        let result = describe_wind_scope(&wind);
        assert_eq!(result, "mostly no wind");

        let wind = WindScope {
            speed_range: SpeedRange::new_meters_per_second(5.3, 9.7),
            dominant_direction: Azimuth::from(273.3),
        };
        let result = describe_wind_scope(&wind);
        assert_eq!(result, "mostly strong west wind blowing at maximum 9.7 m/s");
    }

    #[test]
    fn describes_pressure_range() {
        let range = PressureRange::new(1011.9, 1020.5);
        let result = describe_pressure_range(&range);
        assert_eq!(
            result,
            "High pressure will reach 1011.9 hPa at lowest up to 1020.5 hPa"
        );
    }

    #[test]
    fn describes_entire_forecast() {
        let result = describe_forecast(
            "Some day",
            &TemperatureRange::new_celsius(12.3, 23.4),
            &Kind::Clouds(Clouds::Dense),
            &PercentageRange::new(66, 94),
            &PercentageRange::new(23, 45),
            &WindScope {
                speed_range: SpeedRange::new_meters_per_second(2.5, 8.17),
                dominant_direction: Azimuth::from(115.2),
            },
            &PressureRange::new(1001.2, 1010.5),
        );
        let expected = "Some day it will be warm \
                        with temperatures starting at 12.3°C and reaching 23.4°C.\n\
                        The sky will be overcast \
                        and clouds will cover from 66% to 94% of the sky.\n\
                        The air will be humid at 23% to 45% \
                        with mostly strong southeast wind blowing at maximum 8.2 m/s.\n\
                        Normal pressure will reach 1001.2 hPa at lowest up to 1010.5 hPa.\n";
        assert_eq!(result, expected);
    }
}
