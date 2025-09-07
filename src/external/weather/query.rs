use super::connection::Params;
use crate::types::attributes::*;
use crate::types::units::Coordinates;

pub fn build_past_params(
    coordinates: &Coordinates,
    day_count: u8,
    attributes: &WeatherAttributeSet,
) -> Params {
    vec![
        ("latitude".into(), coordinates.latitude.raw().to_string()),
        ("longitude".into(), coordinates.longitude.raw().to_string()),
        ("daily".into(), build_daily_attribute_list(attributes)),
        ("past_days".into(), day_count.to_string()),
        ("forecast_days".into(), 0.to_string()),
        ("timezone".into(), "auto".to_string()),
        ("wind_speed_unit".into(), "ms".to_string()),
    ]
}

pub fn build_forecast_params(
    coordinates: &Coordinates,
    day_count: u8,
    attributes: &WeatherAttributeSet,
) -> Params {
    vec![
        ("latitude".into(), coordinates.latitude.raw().to_string()),
        ("longitude".into(), coordinates.longitude.raw().to_string()),
        ("daily".into(), build_daily_attribute_list(attributes)),
        ("past_days".into(), 0.to_string()),
        ("forecast_days".into(), day_count.to_string()),
        ("timezone".into(), "auto".to_string()),
        ("wind_speed_unit".into(), "ms".to_string()),
    ]
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

pub fn build_daily_attribute_list(attributes: &WeatherAttributeSet) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    mod utils {
        use super::*;

        #[derive(Debug)]
        pub enum ParamMatcher {
            Any(String),
            Some(String, String),
        }

        impl ParamMatcher {
            pub fn any(key: &str) -> Self {
                Self::Any(key.into())
            }
            pub fn some(key: &str, value: &str) -> Self {
                Self::Some(key.into(), value.into())
            }
            pub fn matches(&self, item: &(String, String)) -> bool {
                let (actual_key, actual_value) = item;
                match self {
                    Self::Any(expected_key) => actual_key == expected_key,
                    Self::Some(expected_key, expected_value) => {
                        actual_key == expected_key && actual_value == expected_value
                    }
                }
            }
        }

        pub fn matches(actual: &Params, mut expected: Vec<ParamMatcher>) -> bool {
            for item in actual {
                let matcher_index = expected.iter().position(|matcher| matcher.matches(&item));
                if let Some(index) = matcher_index {
                    expected.remove(index);
                } else {
                    return false;
                }
            }
            if !expected.is_empty() {
                return false;
            }
            return true;
        }
    }

    #[test]
    fn builds_params_for_past_query() {
        let coordinates = Coordinates::new(1.23, 45.67);
        let day_count = 3;
        let attributes =
            WeatherAttributeSet::from([WeatherAttribute::Temperature, WeatherAttribute::Humidity]);
        let result = build_past_params(&coordinates, day_count, &attributes);

        use utils::*;
        let expected = vec![
            ParamMatcher::some("latitude", "1.23"),
            ParamMatcher::some("longitude", "45.67"),
            ParamMatcher::any("daily"),
            ParamMatcher::some("past_days", "3"),
            ParamMatcher::some("forecast_days", "0"),
            ParamMatcher::some("timezone", "auto"),
            ParamMatcher::some("wind_speed_unit", "ms"),
        ];
        assert!(matches(&result, expected));
    }

    #[test]
    fn builds_params_for_forecast_query() {
        let coordinates = Coordinates::new(1.23, 45.67);
        let day_count = 3;
        let attributes =
            WeatherAttributeSet::from([WeatherAttribute::Temperature, WeatherAttribute::Humidity]);
        let result = build_forecast_params(&coordinates, day_count, &attributes);

        use utils::*;
        let expected = vec![
            ParamMatcher::some("latitude", "1.23"),
            ParamMatcher::some("longitude", "45.67"),
            ParamMatcher::any("daily"),
            ParamMatcher::some("past_days", "0"),
            ParamMatcher::some("forecast_days", "3"),
            ParamMatcher::some("timezone", "auto"),
            ParamMatcher::some("wind_speed_unit", "ms"),
        ];
        assert!(matches(&result, expected));
    }
}
