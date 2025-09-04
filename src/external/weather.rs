use crate::port::weather::WeatherProvider;
use crate::port::weather::{ReportRequest, RequestKind};
use crate::types::attributes::*;
use crate::types::error::FetchError;
use crate::types::report::*;
use crate::types::units::*;
use crate::types::weather::*;
use serde::Deserialize;
use strum::IntoEnumIterator;

pub struct ConcreteWeatherProvider;

impl WeatherProvider for ConcreteWeatherProvider {
    fn fetch(&self, request: &ReportRequest) -> Result<Report, FetchError> {
        match &request.kind {
            RequestKind::PastFull(day_count) => {
                fetch_past_full_report(&request.coordinates, *day_count)
            }
            RequestKind::PastPartial(day_count, attributes) => todo!(),
            RequestKind::CurrentFull => todo!(),
            RequestKind::CurrentPartial(attributes) => todo!(),
            RequestKind::ForecastFull(day_count) => todo!(),
            RequestKind::ForecastPartial(day_count, attributes) => todo!(),
        }
    }
}

struct ListBuilder {
    result: String,
}

impl ListBuilder {
    fn new() -> Self {
        Self {
            result: String::new(),
        }
    }

    fn add(&mut self, value: &str) {
        if self.result.is_empty() {
            self.result.push_str(value);
        } else {
            self.result.push(',');
            self.result.push_str(value);
        }
    }

    fn string(self) -> String {
        self.result
    }
}

fn convert_attributes_to_list(attributes: &[WeatherAttribute]) -> String {
    let mut builder = ListBuilder::new();
    for attribute in attributes {
        match attribute {
            WeatherAttribute::WeatherKind => builder.add("weather_code"),
            WeatherAttribute::Temperature => {
                builder.add("temperature_2m_min");
                builder.add("temperature_2m_max");
            }
            WeatherAttribute::CloudCoverage => {
                builder.add("cloud_cover_min");
                builder.add("cloud_cover_max");
            }
            WeatherAttribute::Humidity => {
                builder.add("relative_humidity_2m_min");
                builder.add("relative_humidity_2m_max");
            }
            WeatherAttribute::Wind => {
                builder.add("wind_speed_10m_min");
                builder.add("wind_speed_10m_max");
                builder.add("wind_direction_10m_dominant");
            }
            WeatherAttribute::Pressure => {
                builder.add("pressure_msl_min");
                builder.add("pressure_msl_max");
            }
        }
    }
    builder.string()
}

fn fetch_past_full_report(
    coordinates: &Coordinates,
    day_count: DayCount,
) -> Result<Report, FetchError> {
    let attributes: Vec<WeatherAttribute> = WeatherAttribute::iter().collect();
    let params = [
        ("latitude", format!("{}", coordinates.latitude.raw())),
        ("longitude", format!("{}", coordinates.longitude.raw())),
        ("daily", convert_attributes_to_list(&attributes)),
        ("timezone", "auto".to_string()),
        ("past_days", day_count.to_string()),
        ("forecast_days", 0.to_string()),
        ("wind_speed_unit", "ms".to_string()),
    ];
    let client = reqwest::blocking::Client::new();
    let response =
        client.get("https://api.open-meteo.com/v1/forecast")
        .query(&params)
        .send()
        .expect("Failed to fetch weather");
    let response: DailyResponse = response.json().expect("Failed to decode");
    Ok(Report::PastFull(response.to_report_data(day_count)))
    // let json_string =
    //     std::fs::read_to_string("daily_full_response.json").expect("Failed to read from file");
    // let response: DailyResponse =
    //     serde_json::from_str(&json_string).expect("Failed to deserialize from JSON");
    // println!("{response:#?}");
    // Ok(Report::PastFull(response.to_report_data(day_count)))
}

#[derive(Deserialize, Debug)]
struct DailyData {
    time: Vec<String>,
    weather_code: Vec<u8>,
    temperature_2m_min: Vec<f32>,
    temperature_2m_max: Vec<f32>,
    cloud_cover_min: Vec<u8>,
    cloud_cover_max: Vec<u8>,
    relative_humidity_2m_min: Vec<u8>,
    relative_humidity_2m_max: Vec<u8>,
    wind_speed_10m_min: Vec<f32>,
    wind_speed_10m_max: Vec<f32>,
    wind_direction_10m_dominant: Vec<f32>,
    pressure_msl_min: Vec<f32>,
    pressure_msl_max: Vec<f32>,
}

impl DailyData {
    fn temperature_range(&self, day_index: usize) -> TemperatureRange {
        TemperatureRange::new_celsius(self.temperature_2m_min[day_index], self.temperature_2m_max[day_index])
    }
    fn cloud_coverage_range(&self, day_index: usize) -> PercentageRange {
        PercentageRange::new(self.cloud_cover_min[day_index] as i8, self.cloud_cover_max[day_index] as i8)
    }
    fn humidity_range(&self, day_index: usize) -> PercentageRange {
        PercentageRange::new(self.relative_humidity_2m_min[day_index] as i8, self.relative_humidity_2m_max[day_index] as i8)
    }
    fn wind_speed_range(&self, day_index: usize) -> SpeedRange {
        SpeedRange::new_meters_per_second(self.wind_speed_10m_min[day_index], self.wind_speed_10m_max[day_index])
    }
    fn wind_direction(&self, day_index: usize) -> Azimuth {
        Azimuth::from(self.wind_direction_10m_dominant[day_index])
    }
    fn pressure_range(&self, day_index: usize) -> PressureRange {
        PressureRange::new_hpa(self.pressure_msl_min[day_index], self.pressure_msl_max[day_index])
    }
}

#[derive(Deserialize, Debug)]
struct DailyResponse {
    daily: DailyData,
}

impl DailyResponse {
    fn to_report_data(self, day_count: DayCount) -> DailyFullReport {
        let mut data = Vec::new();
        let day_count: usize = day_count.into();
        for day_index in 0..day_count {
            let daily_data = DailyFullData {
                date: convert_date(&self.daily.time[day_index]),
                kind: convert_code_to_weather_kind(self.daily.weather_code[day_index]),
                temperature_range: self.daily.temperature_range(day_index),
                cloud_coverage_range: self.daily.cloud_coverage_range(day_index),
                humidity_range: self.daily.humidity_range(day_index),
                wind: WindScope {
                    speed_range: self.daily.wind_speed_range(day_index),
                    dominant_direction: self.daily.wind_direction(day_index),
                },
                pressure_range: self.daily.pressure_range(day_index),
            };
            data.push(daily_data);
        }
        DailyFullReport { data }
    }
}

fn convert_date(input: &str) -> Date {
    Date::parse_from_str(input, "%Y-%m-%d").expect("Failed to parse date")
}

fn convert_code_to_weather_kind(code: u8) -> Kind {
    match code {
        0 => Kind::Clouds(Clouds::Clear),
        1 => Kind::Clouds(Clouds::Light),
        2 => Kind::Clouds(Clouds::Moderate),
        3 => Kind::Clouds(Clouds::Dense),
        45 => Kind::Fog(Fog::Normal),
        48 => Kind::Fog(Fog::Rime),
        code @ (51 | 53 | 55 | 61 | 63 | 65 | 80 | 81 | 82) => {
            let intensity = match code {
                51 | 53 | 61 => PrecipitationIntensity::Light,
                55 | 63 => PrecipitationIntensity::Moderate,
                65 | 80 => PrecipitationIntensity::Heavy,
                81 | 82 => PrecipitationIntensity::Shower,
                _ => panic!("Unknown weather code '{code}'"),
            };
            let precipitation = Precipitation {
                kind: PrecipitationKind::Rain,
                intensity,
                heat: PrecipitationHeat::Normal,
            };
            Kind::Precipitation(precipitation)
        }
        code @ (56 | 57 | 66 | 67) => {
            let intensity = match code {
                56 => PrecipitationIntensity::Light,
                57 | 66 => PrecipitationIntensity::Moderate,
                67 => PrecipitationIntensity::Heavy,
                _ => panic!("Unknown weather code '{code}'"),
            };
            let precipitation = Precipitation {
                kind: PrecipitationKind::Rain,
                intensity,
                heat: PrecipitationHeat::Freezing,
            };
            Kind::Precipitation(precipitation)
        }
        code @ (71 | 73 | 75 | 85 | 86) => {
            let intensity = match code {
                71 => PrecipitationIntensity::Light,
                73 => PrecipitationIntensity::Moderate,
                75 | 85  => PrecipitationIntensity::Heavy,
                86 => PrecipitationIntensity::Shower,
                _ => panic!("Unknown weather code '{code}'"),
            };
            let precipitation = Precipitation {
                kind: PrecipitationKind::Snow,
                intensity,
                heat: PrecipitationHeat::Normal,
            };
            Kind::Precipitation(precipitation)
        }
        95 | 96 | 99 => Kind::Thunderstorm,
        _ => panic!("Unknown weather code '{code}'"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_weather_codes_for_clouds() {
        assert_eq!(convert_code_to_weather_kind(0), Kind::Clouds(Clouds::Clear));
        assert_eq!(convert_code_to_weather_kind(1), Kind::Clouds(Clouds::Light));
        assert_eq!(
            convert_code_to_weather_kind(2),
            Kind::Clouds(Clouds::Moderate)
        );
        assert_eq!(convert_code_to_weather_kind(3), Kind::Clouds(Clouds::Dense));
    }

    #[test]
    fn converts_weather_codes_for_fog() {
        assert_eq!(convert_code_to_weather_kind(45), Kind::Fog(Fog::Normal));
        assert_eq!(convert_code_to_weather_kind(48), Kind::Fog(Fog::Rime));
    }

    #[test]
    fn converts_weather_codes_for_normal_rain() {
        let expected = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            convert_code_to_weather_kind(51),
            Kind::Precipitation(expected)
        );
        assert_eq!(
            convert_code_to_weather_kind(53),
            Kind::Precipitation(expected)
        );
        assert_eq!(
            convert_code_to_weather_kind(61),
            Kind::Precipitation(expected)
        );
        let expected = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            convert_code_to_weather_kind(55),
            Kind::Precipitation(expected)
        );
        assert_eq!(
            convert_code_to_weather_kind(63),
            Kind::Precipitation(expected)
        );
        let expected = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            convert_code_to_weather_kind(65),
            Kind::Precipitation(expected)
        );
        assert_eq!(
            convert_code_to_weather_kind(80),
            Kind::Precipitation(expected)
        );
        let expected = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            convert_code_to_weather_kind(81),
            Kind::Precipitation(expected)
        );
        assert_eq!(
            convert_code_to_weather_kind(82),
            Kind::Precipitation(expected)
        );
    }

    #[test]
    fn converts_weather_codes_for_freezing_rain() {
        let expected = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Freezing,
        };
        assert_eq!(
            convert_code_to_weather_kind(56),
            Kind::Precipitation(expected)
        );
        let expected = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Freezing,
        };
        assert_eq!(
            convert_code_to_weather_kind(57),
            Kind::Precipitation(expected)
        );
        assert_eq!(
            convert_code_to_weather_kind(66),
            Kind::Precipitation(expected)
        );
        let expected = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Freezing,
        };
        assert_eq!(
            convert_code_to_weather_kind(67),
            Kind::Precipitation(expected)
        );
    }

    #[test]
    fn converts_weather_codes_for_snow() {
        let expected = Precipitation {
            kind: PrecipitationKind::Snow,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            convert_code_to_weather_kind(71),
            Kind::Precipitation(expected)
        );
        let expected = Precipitation {
            kind: PrecipitationKind::Snow,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            convert_code_to_weather_kind(73),
            Kind::Precipitation(expected)
        );
        let expected = Precipitation {
            kind: PrecipitationKind::Snow,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            convert_code_to_weather_kind(75),
            Kind::Precipitation(expected)
        );
        assert_eq!(
            convert_code_to_weather_kind(85),
            Kind::Precipitation(expected)
        );
        let expected = Precipitation {
            kind: PrecipitationKind::Snow,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            convert_code_to_weather_kind(86),
            Kind::Precipitation(expected)
        );
    }

    #[test]
    fn converts_weather_codes_for_thunderstorm() {
        assert_eq!(convert_code_to_weather_kind(95), Kind::Thunderstorm);
        assert_eq!(convert_code_to_weather_kind(96), Kind::Thunderstorm);
        assert_eq!(convert_code_to_weather_kind(99), Kind::Thunderstorm);
    }

    #[test]
    fn panics_with_unknown_weather_codes() {
        let result = std::panic::catch_unwind(|| convert_code_to_weather_kind(77));
        assert!(result.is_err());
        let result = std::panic::catch_unwind(|| convert_code_to_weather_kind(4));
        assert!(result.is_err());
        let result = std::panic::catch_unwind(|| convert_code_to_weather_kind(100));
        assert!(result.is_err());
    }

    #[test]
    fn converts_daily_response_to_report() {
        let response = DailyResponse {
            daily: DailyData {
                time: vec![
                    "2025-09-01".into(),
                    "2025-09-02".into(),
                    "2025-09-01".into(),
                ],
                weather_code: vec![3, 2, 1],
                temperature_2m_min: vec![11.1, 12.2, 13.3],
                temperature_2m_max: vec![21.1, 22.2, 23.3],
                cloud_cover_min: vec![11, 12, 13],
                cloud_cover_max: vec![21, 22, 23],
                relative_humidity_2m_min: vec![31, 32, 33],
                relative_humidity_2m_max: vec![41, 42, 43],
                wind_speed_10m_min: vec![31.1, 32.2, 33.3],
                wind_speed_10m_max: vec![41.1, 42.2, 43.3],
                wind_direction_10m_dominant: vec![90.1, 180.2, 270.3],
                pressure_msl_min: vec![1001.1, 1002.2, 1003.3],
                pressure_msl_max: vec![1011.1, 1012.2, 1013.3],
            },
        };
        let report = response.to_report_data(3);
        let day1 = &report.data[0];
        assert_eq!(day1.kind, Kind::Clouds(Clouds::Dense));
        assert_eq!(
            day1.temperature_range,
            TemperatureRange::new_celsius(11.1, 21.1)
        );
        assert_eq!(day1.cloud_coverage_range, PercentageRange::new(11, 21));
        assert_eq!(day1.humidity_range, PercentageRange::new(31, 41));
        assert_eq!(day1.wind.speed_range, SpeedRange::new_meters_per_second(31.1, 41.1));
        assert_eq!(day1.wind.dominant_direction, Azimuth::from(90.1));
        assert_eq!(day1.pressure_range, PressureRange::new_hpa(1001.1, 1011.1));
    }
}
