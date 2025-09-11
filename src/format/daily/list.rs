use crate::format::common::list_builder::write_param;
use crate::format::common::list_format::describe_kind;
use crate::types::report::*;
use crate::types::units::*;
use crate::types::weather::*;
use std::fmt::Write;

pub fn describe(report: &DailyPartialReport) -> String {
    describe_report(report).expect("Failed to write to string")
}

fn describe_report(report: &DailyPartialReport) -> Result<String, std::fmt::Error> {
    let DailyPartialReport { coordinates, data } = &report;
    let mut result = String::new();

    write_param(&mut result, "Coordinates", format!("{coordinates:.5}"));
    writeln!(&mut result)?;

    let mut day_iter = data.iter();
    if let Some(data) = day_iter.next() {
        describe_day(&mut result, data);
    }
    for daily_data in day_iter {
        writeln!(&mut result)?;
        describe_day(&mut result, daily_data);
    }
    Ok(result)
}

fn describe_day(result: &mut String, data: &DailyPartialData) {
    let date_str = data.date.format("%d.%m.%Y").to_string();
    write_param(result, "Date", date_str);
    if let Some(kind) = data.kind {
        write_param(result, "Weather", describe_kind(&kind));
    }
    if let Some(range) = &data.temperature_range {
        write_temperature(result, range);
    }
    if let Some(PercentageRange { min, max }) = data.cloud_coverage_range {
        write_param(result, "Cloud coverage", format_range(min, max));
    }
    if let Some(PercentageRange { min, max }) = data.humidity_range {
        write_param(result, "Humidity", format_range(min, max));
    }
    if let Some(scope) = &data.wind {
        let WindScope {
            speed_range,
            dominant_direction,
        } = scope;
        let speed_desc = match speed_range {
            SpeedRange::MetersPerSecond { min, max } => format_range(min, max),
        };
        let cardinal_symbol = dominant_direction.to_cardinal_direction().to_symbol();
        let value = format!("{speed_desc}, {dominant_direction} ({cardinal_symbol})");
        write_param(result, "Wind", value);
    }
    if let Some(pressure) = &data.pressure_range {
        let value = match pressure {
            PressureRange::Hpa { min, max } => format_range(min, max),
        };
        write_param(result, "Pressure", value);
    }
}

fn format_range<T: std::fmt::Display>(min: T, max: T) -> String {
    format!("{min} - {max}")
}

fn write_temperature(result: &mut String, range: &TemperatureRange) {
    let value = match range {
        TemperatureRange::Celsius { min, max } => format_range(min, max),
        TemperatureRange::Fahrenheit { min, max } => format_range(min, max),
    };
    write_param(result, "Temperature", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_temperature_range_in_celsius() {
        let mut result = String::new();
        write_temperature(&mut result, &TemperatureRange::new_celsius(12.3, 23.4));
        assert_eq!(result, "Temperature: 12.3°C - 23.4°C\n");
    }

    #[test]
    fn writes_temperature_range_in_fahrenheit() {
        let mut result = String::new();
        write_temperature(&mut result, &TemperatureRange::new_fahrenheit(12.3, 23.4));
        assert_eq!(result, "Temperature: 12.3°F - 23.4°F\n");
    }

    fn generate_coordinates() -> Coordinates {
        Coordinates::new(1.23, 45.67)
    }

    fn generate_start_date(days_after: u64) -> Date {
        Date::from_ymd_opt(2025, 8, 26)
            .unwrap()
            .checked_add_days(chrono::Days::new(days_after))
            .unwrap()
    }

    fn generate_report_for_3_days() -> DailyPartialReport {
        let data = vec![
            DailyPartialData {
                date: generate_start_date(0),
                kind: Some(Kind::Clouds(Clouds::Light)),
                temperature_range: Some(TemperatureRange::new_celsius(24.5, 27.1)),
                cloud_coverage_range: None,
                humidity_range: Some(PercentageRange::new(33, 46)),
                wind: None,
                pressure_range: None,
            },
            DailyPartialData {
                date: generate_start_date(1),
                kind: Some(Kind::Clouds(Clouds::Moderate)),
                temperature_range: Some(TemperatureRange::new_celsius(26.5, 29.1)),
                cloud_coverage_range: Some(PercentageRange::new(56, 79)),
                humidity_range: Some(PercentageRange::new(34, 47)),
                wind: Some(WindScope {
                    speed_range: SpeedRange::new_meters_per_second(1.2, 2.84),
                    dominant_direction: Azimuth::from(178.5),
                }),
                pressure_range: Some(PressureRange::new_hpa(999.9, 1111.1)),
            },
            DailyPartialData {
                date: generate_start_date(2),
                kind: Some(Kind::Clouds(Clouds::Dense)),
                temperature_range: Some(TemperatureRange::new_celsius(28.5, 31.1)),
                cloud_coverage_range: None,
                humidity_range: Some(PercentageRange::new(35, 48)),
                wind: None,
                pressure_range: None,
            },
        ];
        DailyPartialReport {
            coordinates: generate_coordinates(),
            data,
        }
    }

    #[test]
    fn formats_report() {
        let report = generate_report_for_3_days();
        let result = describe(&report);
        let expected = "Coordinates: 1.23000°, 45.67000°\n\
                        \n\
                        Date: 26.08.2025\n\
                        Weather: light clouds\n\
                        Temperature: 24.5°C - 27.1°C\n\
                        Humidity: 33% - 46%\n\
                        \n\
                        Date: 27.08.2025\n\
                        Weather: cloudy\n\
                        Temperature: 26.5°C - 29.1°C\n\
                        Cloud coverage: 56% - 79%\n\
                        Humidity: 34% - 47%\n\
                        Wind: 1.2 m/s - 2.8 m/s, 178.5° (S)\n\
                        Pressure: 999.9 hPa - 1111.1 hPa\n\
                        \n\
                        Date: 28.08.2025\n\
                        Weather: overcast sky\n\
                        Temperature: 28.5°C - 31.1°C\n\
                        Humidity: 35% - 48%\n";
        assert_eq!(result, expected);
    }
}
