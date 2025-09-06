use crate::port::weather::WeatherProvider;
use crate::port::weather::{ReportRequest, RequestKind};
use crate::types::attributes::*;
use crate::types::error::FetchError;
use crate::types::report::*;
use crate::types::units::*;
use crate::types::weather::*;
use serde::Deserialize;
use std::collections::HashSet;
use strum::IntoEnumIterator;

pub struct ConcreteWeatherProvider;

impl WeatherProvider for ConcreteWeatherProvider {
    fn fetch(&self, request: &ReportRequest) -> Result<Report, FetchError> {
        match &request.kind {
            RequestKind::PastFull(day_count) => {
                fetch_past_full_report(&request.coordinates, *day_count)
            }
            RequestKind::PastPartial(day_count, attributes) => {
                fetch_past_partial_report(&request.coordinates, *day_count, attributes)
            }
            RequestKind::CurrentFull => todo!(),
            RequestKind::CurrentPartial(_attributes) => todo!(),
            RequestKind::ForecastFull(_day_count) => todo!(),
            RequestKind::ForecastPartial(_day_count, _attributes) => todo!(),
        }
    }
}

fn fetch_past_full_report(
    coordinates: &Coordinates,
    day_count: DayCount,
) -> Result<Report, FetchError> {
    let attributes: WeatherAttributeSet = WeatherAttribute::iter().collect();
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
    let response = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&params)
        .send()
        .expect("Failed to fetch weather");
    let response: DailyResponse = response.json().expect("Failed to decode");
    Ok(Report::PastFull(response.to_daily_full_report(day_count)))
}

fn fetch_past_partial_report(
    coordinates: &Coordinates,
    day_count: u8,
    attributes: &HashSet<WeatherAttribute>,
) -> Result<Report, FetchError> {
    let params = [
        ("latitude", format!("{}", coordinates.latitude.raw())),
        ("longitude", format!("{}", coordinates.longitude.raw())),
        ("daily", convert_attributes_to_list(attributes)),
        ("timezone", "auto".to_string()),
        ("past_days", day_count.to_string()),
        ("forecast_days", 0.to_string()),
        ("wind_speed_unit", "ms".to_string()),
    ];
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&params)
        .send()
        .expect("Failed to fetch weather");
    let response: DailyResponse = response.json().expect("Failed to decode");
    Ok(Report::PastPartial(
        response.to_daily_partial_report(coordinates, day_count),
    ))
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
        if !self.result.is_empty() {
            self.result.push(',');
        }
        self.result.push_str(value);
    }

    fn string(self) -> String {
        self.result
    }
}

fn convert_attributes_to_list(attributes: &WeatherAttributeSet) -> String {
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

#[derive(Clone, Deserialize, Debug)]
struct DailyData {
    time: Option<Vec<String>>,
    weather_code: Option<Vec<u8>>,
    temperature_2m_min: Option<Vec<f32>>,
    temperature_2m_max: Option<Vec<f32>>,
    cloud_cover_min: Option<Vec<u8>>,
    cloud_cover_max: Option<Vec<u8>>,
    relative_humidity_2m_min: Option<Vec<u8>>,
    relative_humidity_2m_max: Option<Vec<u8>>,
    wind_speed_10m_min: Option<Vec<f32>>,
    wind_speed_10m_max: Option<Vec<f32>>,
    wind_direction_10m_dominant: Option<Vec<f32>>,
    pressure_msl_min: Option<Vec<f32>>,
    pressure_msl_max: Option<Vec<f32>>,
}

impl DailyData {
    fn date(&self, day_index: usize) -> Option<Date> {
        self.time
            .as_ref()?
            .get(day_index)
            .map(|string| convert_date(string))
            .or_else(|| panic!("Missing date for day {day_index}"))
    }
    fn weather_kind(&self, day_index: usize) -> Option<Kind> {
        self.weather_code
            .as_ref()?
            .get(day_index)
            .map(|code| convert_code_to_weather_kind(*code))
            .or_else(|| panic!("Missing weather code for day {day_index}"))
    }
    fn temperature_range(&self, day_index: usize) -> Option<TemperatureRange> {
        let min = self
            .temperature_2m_min
            .as_ref()?
            .get(day_index)
            .or_else(|| panic!("Missing min temperature for day {day_index}"));
        let max = self
            .temperature_2m_max
            .as_ref()?
            .get(day_index)
            .or_else(|| panic!("Missing max temperature for day {day_index}"));
        min.zip(max)
            .map(|(min, max)| TemperatureRange::new_celsius(*min, *max))
    }
    fn cloud_coverage_range(&self, day_index: usize) -> Option<PercentageRange> {
        let min = self
            .cloud_cover_min
            .as_ref()?
            .get(day_index)
            .or_else(|| panic!("Missing min cloud coverage for day {day_index}"));
        let max = self
            .cloud_cover_max
            .as_ref()?
            .get(day_index)
            .or_else(|| panic!("Missing max cloud coverage for day {day_index}"));
        min.zip(max)
            .map(|(min, max)| PercentageRange::new(*min as i8, *max as i8))
    }
    fn humidity_range(&self, day_index: usize) -> Option<PercentageRange> {
        let min = self
            .relative_humidity_2m_min
            .as_ref()?
            .get(day_index)
            .or_else(|| panic!("Missing min humidity for day {day_index}"));
        let max = self
            .relative_humidity_2m_max
            .as_ref()?
            .get(day_index)
            .or_else(|| panic!("Missing max humidity for day {day_index}"));
        min.zip(max)
            .map(|(min, max)| PercentageRange::new(*min as i8, *max as i8))
    }
    fn wind_scope(&self, day_index: usize) -> Option<WindScope> {
        let min_speed = self
            .wind_speed_10m_min
            .as_ref()?
            .get(day_index)
            .or_else(|| panic!("Missing min wind speed for day {day_index}"));
        let max_speed = self
            .wind_speed_10m_max
            .as_ref()?
            .get(day_index)
            .or_else(|| panic!("Missing max wind speed for day {day_index}"));
        let azimuth = self
            .wind_direction_10m_dominant
            .as_ref()
            .map(|values| Azimuth::from(values[day_index]));
        match (min_speed, max_speed, azimuth) {
            (Some(min), Some(max), Some(dominant_direction)) => Some(WindScope {
                speed_range: SpeedRange::new_meters_per_second(*min, *max),
                dominant_direction,
            }),
            (None, Some(_), Some(_)) => panic!("Missing values for min wind speed"),
            (Some(_), None, Some(_)) => panic!("Missing values for max wind speed"),
            (Some(_), Some(_), None) => panic!("Missing values for wind direction"),
            _ => None,
        }
    }
    fn pressure_range(&self, day_index: usize) -> Option<PressureRange> {
        let min = self
            .pressure_msl_min
            .as_ref()?
            .get(day_index)
            .or_else(|| panic!("Missing min pressure for day {day_index}"));
        let max = self
            .pressure_msl_max
            .as_ref()?
            .get(day_index)
            .or_else(|| panic!("Missing max pressure for day {day_index}"));
        min.zip(max)
            .map(|(min, max)| PressureRange::new_hpa(*min, *max))
    }
}

#[derive(Clone, Deserialize, Debug)]
struct DailyResponse {
    daily: DailyData,
}

impl DailyResponse {
    fn to_daily_full_report(&self, day_count: DayCount) -> DailyFullReport {
        let mut data = Vec::new();
        let day_count: usize = day_count.into();
        let daily = &self.daily;
        for day_index in 0..day_count {
            let date = daily
                .date(day_index)
                .unwrap_or_else(|| panic!("Missing date for day {day_index}"));
            let kind = daily
                .weather_kind(day_index)
                .unwrap_or_else(|| panic!("Missing weather kind for day {day_index}"));
            let temperature_range = daily
                .temperature_range(day_index)
                .unwrap_or_else(|| panic!("Missing temperature for day {day_index}"));
            let cloud_coverage_range = daily
                .cloud_coverage_range(day_index)
                .unwrap_or_else(|| panic!("Missing cloud coverage for day {day_index}"));
            let humidity_range = daily
                .humidity_range(day_index)
                .unwrap_or_else(|| panic!("Missing humidity for day {day_index}"));
            let wind = daily
                .wind_scope(day_index)
                .unwrap_or_else(|| panic!("Missing wind for day {day_index}"));
            let pressure_range = daily
                .pressure_range(day_index)
                .unwrap_or_else(|| panic!("Missing pressure for day {day_index}"));
            let daily_data = DailyFullData {
                date,
                kind,
                temperature_range,
                cloud_coverage_range,
                humidity_range,
                wind,
                pressure_range,
            };
            data.push(daily_data);
        }
        DailyFullReport { data }
    }

    fn to_daily_partial_report(
        &self,
        coordinates: &Coordinates,
        day_count: DayCount,
    ) -> DailyPartialReport {
        let mut data = Vec::new();
        let day_count: usize = day_count.into();
        for day_index in 0..day_count {
            let date = self
                .daily
                .date(day_index)
                .unwrap_or_else(|| panic!("Missing date at day {day_index}"));
            let daily_data = DailyPartialData {
                date,
                kind: self.daily.weather_kind(day_index),
                temperature_range: self.daily.temperature_range(day_index),
                cloud_coverage_range: self.daily.cloud_coverage_range(day_index),
                humidity_range: self.daily.humidity_range(day_index),
                wind: self.daily.wind_scope(day_index),
                pressure_range: self.daily.pressure_range(day_index),
            };
            data.push(daily_data);
        }
        DailyPartialReport {
            coordinates: *coordinates,
            data,
        }
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
                75 | 85 => PrecipitationIntensity::Heavy,
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

    fn generate_daily_response() -> DailyResponse {
        DailyResponse {
            daily: DailyData {
                time: Some(vec![
                    "2025-09-01".into(),
                    "2025-09-02".into(),
                    "2025-09-03".into(),
                ]),
                weather_code: Some(vec![3, 2, 1]),
                temperature_2m_min: Some(vec![11.1, 12.2, 13.3]),
                temperature_2m_max: Some(vec![21.1, 22.2, 23.3]),
                cloud_cover_min: Some(vec![11, 12, 13]),
                cloud_cover_max: Some(vec![21, 22, 23]),
                relative_humidity_2m_min: Some(vec![31, 32, 33]),
                relative_humidity_2m_max: Some(vec![41, 42, 43]),
                wind_speed_10m_min: Some(vec![31.1, 32.2, 33.3]),
                wind_speed_10m_max: Some(vec![41.1, 42.2, 43.3]),
                wind_direction_10m_dominant: Some(vec![90.1, 180.2, 270.3]),
                pressure_msl_min: Some(vec![1001.1, 1002.2, 1003.3]),
                pressure_msl_max: Some(vec![1011.1, 1012.2, 1013.3]),
            },
        }
    }

    #[test]
    fn converts_daily_response_to_daily_full_report() {
        let response = generate_daily_response();
        let report = response.to_daily_full_report(3);
        let expected_day0 = DailyFullData {
            date: Date::from_ymd_opt(2025, 9, 1).unwrap(),
            kind: Kind::Clouds(Clouds::Dense),
            temperature_range: TemperatureRange::new_celsius(11.1, 21.1),
            cloud_coverage_range: PercentageRange::new(11, 21),
            humidity_range: PercentageRange::new(31, 41),
            wind: WindScope {
                speed_range: SpeedRange::new_meters_per_second(31.1, 41.1),
                dominant_direction: Azimuth::from(90.1),
            },
            pressure_range: PressureRange::new_hpa(1001.1, 1011.1),
        };
        assert_eq!(report.data[0], expected_day0);

        let expected_day1 = DailyFullData {
            date: Date::from_ymd_opt(2025, 9, 2).unwrap(),
            kind: Kind::Clouds(Clouds::Moderate),
            temperature_range: TemperatureRange::new_celsius(12.2, 22.2),
            cloud_coverage_range: PercentageRange::new(12, 22),
            humidity_range: PercentageRange::new(32, 42),
            wind: WindScope {
                speed_range: SpeedRange::new_meters_per_second(32.2, 42.2),
                dominant_direction: Azimuth::from(180.2),
            },
            pressure_range: PressureRange::new_hpa(1002.2, 1012.2),
        };
        assert_eq!(report.data[1], expected_day1);

        let expected_day2 = DailyFullData {
            date: Date::from_ymd_opt(2025, 9, 3).unwrap(),
            kind: Kind::Clouds(Clouds::Light),
            temperature_range: TemperatureRange::new_celsius(13.3, 23.3),
            cloud_coverage_range: PercentageRange::new(13, 23),
            humidity_range: PercentageRange::new(33, 43),
            wind: WindScope {
                speed_range: SpeedRange::new_meters_per_second(33.3, 43.3),
                dominant_direction: Azimuth::from(270.3),
            },
            pressure_range: PressureRange::new_hpa(1003.3, 1013.3),
        };
        assert_eq!(report.data[2], expected_day2);
    }

    macro_rules! generate_daily_response_without {
        ($field_to_skip:ident) => {{
            let mut response = generate_daily_response();
            response.daily.$field_to_skip = None;
            response
        }};
    }

    #[test]
    fn fails_to_convert_daily_response_to_daily_full_report_when_any_param_is_missing() {
        let expect_panic = |response: DailyResponse| {
            let result = std::panic::catch_unwind(|| response.to_daily_full_report(3));
            assert!(result.is_err());
        };
        expect_panic(generate_daily_response_without!(time));
        expect_panic(generate_daily_response_without!(weather_code));
        expect_panic(generate_daily_response_without!(temperature_2m_min));
        expect_panic(generate_daily_response_without!(temperature_2m_max));
        expect_panic(generate_daily_response_without!(cloud_cover_min));
        expect_panic(generate_daily_response_without!(cloud_cover_max));
        expect_panic(generate_daily_response_without!(relative_humidity_2m_min));
        expect_panic(generate_daily_response_without!(relative_humidity_2m_max));
        expect_panic(generate_daily_response_without!(wind_speed_10m_min));
        expect_panic(generate_daily_response_without!(wind_speed_10m_max));
        expect_panic(generate_daily_response_without!(
            wind_direction_10m_dominant
        ));
        expect_panic(generate_daily_response_without!(pressure_msl_min));
        expect_panic(generate_daily_response_without!(pressure_msl_min));
    }

    #[test]
    fn converts_daily_response_to_daily_partial_report_with_all_parameters() {
        let response = generate_daily_response();
        let coordinates = Coordinates::new(1.23, 45.67);
        let report = response.to_daily_partial_report(&coordinates, 3);

        assert_eq!(report.coordinates, coordinates);

        let expected_day0 = DailyPartialData {
            date: Date::from_ymd_opt(2025, 9, 1).unwrap(),
            kind: Some(Kind::Clouds(Clouds::Dense)),
            temperature_range: Some(TemperatureRange::new_celsius(11.1, 21.1)),
            cloud_coverage_range: Some(PercentageRange::new(11, 21)),
            humidity_range: Some(PercentageRange::new(31, 41)),
            wind: Some(WindScope {
                speed_range: SpeedRange::new_meters_per_second(31.1, 41.1),
                dominant_direction: Azimuth::from(90.1),
            }),
            pressure_range: Some(PressureRange::new_hpa(1001.1, 1011.1)),
        };
        assert_eq!(report.data[0], expected_day0);

        let expected_day1 = DailyPartialData {
            date: Date::from_ymd_opt(2025, 9, 2).unwrap(),
            kind: Some(Kind::Clouds(Clouds::Moderate)),
            temperature_range: Some(TemperatureRange::new_celsius(12.2, 22.2)),
            cloud_coverage_range: Some(PercentageRange::new(12, 22)),
            humidity_range: Some(PercentageRange::new(32, 42)),
            wind: Some(WindScope {
                speed_range: SpeedRange::new_meters_per_second(32.2, 42.2),
                dominant_direction: Azimuth::from(180.2),
            }),
            pressure_range: Some(PressureRange::new_hpa(1002.2, 1012.2)),
        };
        assert_eq!(report.data[1], expected_day1);

        let expected_day2 = DailyPartialData {
            date: Date::from_ymd_opt(2025, 9, 3).unwrap(),
            kind: Some(Kind::Clouds(Clouds::Light)),
            temperature_range: Some(TemperatureRange::new_celsius(13.3, 23.3)),
            cloud_coverage_range: Some(PercentageRange::new(13, 23)),
            humidity_range: Some(PercentageRange::new(33, 43)),
            wind: Some(WindScope {
                speed_range: SpeedRange::new_meters_per_second(33.3, 43.3),
                dominant_direction: Azimuth::from(270.3),
            }),
            pressure_range: Some(PressureRange::new_hpa(1003.3, 1013.3)),
        };
        assert_eq!(report.data[2], expected_day2);
    }

    #[test]
    fn converts_daily_response_to_daily_partial_report_with_some_parameters() {
        let mut response = generate_daily_response();
        response.daily.wind_direction_10m_dominant = None;
        response.daily.wind_speed_10m_min = None;
        response.daily.wind_speed_10m_max = None;
        response.daily.temperature_2m_min = None;
        response.daily.temperature_2m_max = None;
        response.daily.cloud_cover_min = None;
        response.daily.cloud_cover_max = None;
        let coordinates = Coordinates::new(1.23, 45.67);
        let report = response.to_daily_partial_report(&coordinates, 3);

        assert_eq!(report.coordinates, coordinates);

        let expected_day0 = DailyPartialData {
            date: Date::from_ymd_opt(2025, 9, 1).unwrap(),
            kind: Some(Kind::Clouds(Clouds::Dense)),
            temperature_range: None,
            cloud_coverage_range: None,
            humidity_range: Some(PercentageRange::new(31, 41)),
            wind: None,
            pressure_range: Some(PressureRange::new_hpa(1001.1, 1011.1)),
        };
        assert_eq!(report.data[0], expected_day0);

        let expected_day1 = DailyPartialData {
            date: Date::from_ymd_opt(2025, 9, 2).unwrap(),
            kind: Some(Kind::Clouds(Clouds::Moderate)),
            temperature_range: None,
            cloud_coverage_range: None,
            humidity_range: Some(PercentageRange::new(32, 42)),
            wind: None,
            pressure_range: Some(PressureRange::new_hpa(1002.2, 1012.2)),
        };
        assert_eq!(report.data[1], expected_day1);

        let expected_day2 = DailyPartialData {
            date: Date::from_ymd_opt(2025, 9, 3).unwrap(),
            kind: Some(Kind::Clouds(Clouds::Light)),
            temperature_range: None,
            cloud_coverage_range: None,
            humidity_range: Some(PercentageRange::new(33, 43)),
            wind: None,
            pressure_range: Some(PressureRange::new_hpa(1003.3, 1013.3)),
        };
        assert_eq!(report.data[2], expected_day2);
    }
}
