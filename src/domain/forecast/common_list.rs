use crate::domain::common::list_builder::write_param;
use crate::domain::common::list_format::describe_kind;
use crate::types::report::ForecastPartialSpec;
use crate::types::units::*;
use crate::types::weather::*;

pub fn write_spec(result: &mut String, spec: &ForecastPartialSpec) {
    if let Some(kind) = spec.kind {
        write_param(result, "Weather", describe_kind(&kind));
    }
    if let Some(temperature) = &spec.temperature_range {
        let value = match temperature {
            TemperatureRange::Celsius { min, max } => format!("{min} - {max}"),
        };
        write_param(result, "Temperature", value);
    }
    if let Some(PercentageRange { min, max }) = spec.cloud_coverage_range {
        write_param(result, "Cloud coverage", format!("{min} - {max}"));
    }
    if let Some(PercentageRange { min, max }) = spec.humidity_range {
        write_param(result, "Humidity", format!("{min} - {max}"));
    }
    if let Some(WindScope {
        speed_range,
        dominant_direction,
    }) = &spec.wind
    {
        let speed_desc = match speed_range {
            SpeedRange::MetersPerSecond { min, max } => format!("{min} - {max}"),
        };
        let cardinal_symbol = dominant_direction.to_cardinal_direction().to_symbol();
        write_param(
            result,
            "Wind",
            format!("{speed_desc}, {dominant_direction} ({cardinal_symbol})"),
        );
    }
    if let Some(PressureRange { min, max }) = spec.pressure_range {
        write_param(result, "Pressure", format!("{min} - {max}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_only_selected_attributes_in_spec() {
        let spec = ForecastPartialSpec {
            kind: Some(Kind::Clouds(Clouds::Moderate)),
            temperature_range: Some(TemperatureRange::new_celsius(24.5, 27.1)),
            cloud_coverage_range: None,
            humidity_range: Some(PercentageRange::new(33, 46)),
            wind: None,
            pressure_range: None,
        };
        let mut result = String::default();
        write_spec(&mut result, &spec);
        let expected = "Weather: cloudy\n\
                        Temperature: 24.5°C - 27.1°C\n\
                        Humidity: 33% - 46%\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn adds_all_attributes_in_spec() {
        let spec = ForecastPartialSpec {
            kind: Some(Kind::Clouds(Clouds::Moderate)),
            temperature_range: Some(TemperatureRange::new_celsius(24.5, 27.1)),
            cloud_coverage_range: Some(PercentageRange::new(56, 79)),
            humidity_range: Some(PercentageRange::new(33, 46)),
            wind: Some(WindScope {
                speed_range: SpeedRange::new_meters_per_second(1.2, 2.84),
                dominant_direction: Azimuth::from(178.5),
            }),
            pressure_range: Some(PressureRange::new(999.9, 1111.1)),
        };
        let mut result = String::default();
        write_spec(&mut result, &spec);
        let expected = "Weather: cloudy\n\
                        Temperature: 24.5°C - 27.1°C\n\
                        Cloud coverage: 56% - 79%\n\
                        Humidity: 33% - 46%\n\
                        Wind: 1.2 m/s - 2.8 m/s, 178.5° (S)\n\
                        Pressure: 999.9 hPa - 1111.1 hPa\n";
        assert_eq!(result, expected);
    }
}
