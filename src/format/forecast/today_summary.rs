use crate::format::forecast::common::describe_forecast;
use crate::types::report::TodayForecastFullReport;

pub fn describe(report: &TodayForecastFullReport) -> String {
    let time_desc = "Today";
    describe_forecast(
        time_desc,
        &report.temperature_range,
        &report.kind,
        &report.cloud_coverage_range,
        &report.humidity_range,
        &report.wind,
        &report.pressure_range,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::weather::*;
    use crate::types::units::*;

    #[test]
    fn describes_entire_report() {
        let report = TodayForecastFullReport {
            kind: Kind::Clouds(Clouds::Dense),
            temperature_range: TemperatureRange::new_celsius(12.3, 23.4),
            cloud_coverage_range: PercentageRange::new(66, 94),
            humidity_range: PercentageRange::new(23, 45),
            wind: WindScope {
                speed_range: SpeedRange::new_meters_per_second(2.5, 8.17),
                dominant_direction: Azimuth::from(115.2),
            },
            pressure_range: PressureRange::new_hpa(1001.2, 1010.5),
        };
        let result = describe(&report);
        let expected = "Today it will be warm \
                        with temperatures starting at 12.3°C and reaching 23.4°C.\n\
                        The sky will be overcast \
                        and clouds will cover from 66% to 94% of the sky.\n\
                        The air will be humid at 23% to 45% \
                        with mostly strong southeast wind blowing at maximum 8.2 m/s.\n\
                        Normal pressure will reach 1001.2 hPa at lowest up to 1010.5 hPa.\n";
        assert_eq!(result, expected);
    }
}
