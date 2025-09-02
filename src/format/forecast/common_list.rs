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
            humidity_range: Some(PercentageRange::new(33, 46)),
        };
        let mut result = String::default();
        write_spec(&mut result, &spec);
        let expected = "Weather: cloudy\n\
                        Temperature: 24.5°C - 27.1°C\n\
                        Humidity: 33% - 46%\n\
        assert_eq!(result, expected);
    }
}
