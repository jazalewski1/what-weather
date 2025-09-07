use super::connection::Params;
use crate::types::attributes::*;
use crate::types::units::Coordinates;

mod keys {
    pub const LATITUDE: &str = "latitude";
    pub const LONGITUDE: &str = "longitude";
    pub const DAILY: &str = "daily";
    pub const PAST_DAYS: &str = "past_days";
    pub const FORECAST_DAYS: &str = "forecast_days";
    pub const TIMEZONE: &str = "timezone";
    pub const WIND_SPEED_UNIT: &str = "wind_speed_unit";
}

mod values {
    pub const TZ_AUTO: &str = "auto";
    pub const METERS_PER_SECOND: &str = "ms";
}

pub fn build_past_params(
    coordinates: &Coordinates,
    day_count: u8,
    attributes: &WeatherAttributeSet,
) -> Params {
    vec![
        make_param(keys::LATITUDE, coordinates.latitude.raw()),
        make_param(keys::LONGITUDE, coordinates.longitude.raw()),
        make_param(keys::DAILY, build_daily_attribute_list(attributes)),
        make_param(keys::PAST_DAYS, day_count),
        make_param(keys::FORECAST_DAYS, 0),
        make_param(keys::TIMEZONE, values::TZ_AUTO),
        make_param(keys::WIND_SPEED_UNIT, values::METERS_PER_SECOND),
    ]
}

pub fn build_forecast_params(
    coordinates: &Coordinates,
    day_count: u8,
    attributes: &WeatherAttributeSet,
) -> Params {
    vec![
        make_param(keys::LATITUDE, coordinates.latitude.raw()),
        make_param(keys::LONGITUDE, coordinates.longitude.raw()),
        make_param(keys::DAILY, build_daily_attribute_list(attributes)),
        make_param(keys::PAST_DAYS, 0),
        make_param(keys::FORECAST_DAYS, day_count),
        make_param(keys::TIMEZONE, values::TZ_AUTO),
        make_param(keys::WIND_SPEED_UNIT, values::METERS_PER_SECOND),
    ]
}

fn make_param<T: std::fmt::Display>(key: &str, value: T) -> (String, String) {
    (key.into(), value.to_string())
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

fn build_daily_attribute_list(attributes: &WeatherAttributeSet) -> String {
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
