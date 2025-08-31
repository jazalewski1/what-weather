use crate::format::common::list_builder::write_param;
use crate::format::forecast::common_list::write_spec;
use crate::types::report::TodayForecastPartialReport;

pub fn describe(report: &TodayForecastPartialReport) -> String {
    let TodayForecastPartialReport { coordinates, spec } = &report;
    let mut result = String::default();
    write_param(&mut result, "Coordinates", format!("{coordinates:.5}"));
    write_spec(&mut result, spec);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::weather::*;
    use crate::types::units::*;
    use crate::types::report::ForecastPartialSpec;

    #[test]
    fn describes_report() {
        let coordinates = Coordinates::new(1.23, 45.67);
        let spec = ForecastPartialSpec {
            kind: Some(Kind::Clouds(Clouds::Moderate)),
            temperature_range: Some(TemperatureRange::new_celsius(24.5, 27.1)),
            cloud_coverage_range: None,
            humidity_range: Some(PercentageRange::new(33, 46)),
            wind: None,
            pressure_range: None,
        };
        let report = TodayForecastPartialReport { coordinates, spec };
        let result = describe(&report);
        let expected = "Coordinates: 1.23000°, 45.67000°\n\
                        Weather: cloudy\n\
                        Temperature: 24.5°C - 27.1°C\n\
                        Humidity: 33% - 46%\n";
        assert_eq!(result, expected);
    }
}
