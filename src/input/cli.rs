use crate::port::weather::*;
use crate::types::attributes::*;
use crate::types::units::*;
use crate::weather_reporter::Parameters;
use clap::builder::PossibleValue;
use clap::{Parser, Subcommand, ValueEnum};
use std::str::FromStr;
use strum::{IntoEnumIterator, VariantArray};

impl ValueEnum for WeatherAttribute {
    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::WeatherKind => Some(PossibleValue::new("weather_kind")),
            Self::Temperature => Some(PossibleValue::new("temperature")),
            Self::CloudCoverage => Some(PossibleValue::new("cloud_coverage")),
            Self::Humidity => Some(PossibleValue::new("humidity")),
            Self::Wind => Some(PossibleValue::new("wind")),
            Self::Pressure => Some(PossibleValue::new("pressure")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        Self::VARIANTS
    }
}

impl FromStr for Coordinates {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = string.split(',').collect();
        if values.len() != 2 {
            return Err("Coordinates must be in format 'latitude,longitude'".into());
        }
        let latitude: f32 = values[0].trim().parse().map_err(|_| "Invalid latitude")?;
        let longitude: f32 = values[1].trim().parse().map_err(|_| "Invalid longitude")?;
        Ok(Self::new(latitude, longitude))
    }
}

#[derive(Subcommand)]
enum Command {
    /// Report current weather
    Now {
        /// Format report as summary
        #[arg(long, group = "now_format")]
        summary: bool,

        /// Format report as list of all or selected attributes
        #[arg(long, group = "now_format", value_delimiter=',', num_args=0..)]
        list: Option<Vec<WeatherAttribute>>,
    },

    /// Report forecast
    Forecast {
        /// Format report as summary
        #[arg(long, group = "forecast_format")]
        summary: bool,

        /// Format report as list of all or selected attributes
        #[arg(long, group = "forecast_format", value_delimiter=',', num_args=0..)]
        list: Option<Vec<WeatherAttribute>>,

        /// Report for today
        #[arg(long, group = "forecast_time")]
        today: bool,

        /// Report for multiple days from today
        #[arg(long, group = "forecast_time", value_parser = clap::value_parser!(u8).range(1..16))]
        days: Option<DayCount>,
    },
}

#[derive(Parser)]
struct Args {
    /// Report type
    #[command(subcommand)]
    command: Option<Command>,

    /// Report from location specified by coordinates
    #[arg(long, group = "location")]
    coords: Option<Coordinates>,

    /// Report from current location based on IP
    #[arg(long, group = "location")]
    here: bool,
}

fn convert_to_attribute_set(attributes: &[WeatherAttribute]) -> WeatherAttributeSet {
    if attributes.is_empty() {
        WeatherAttribute::iter().collect()
    } else {
        attributes.iter().cloned().collect()
    }
}

fn convert_args_to_parameters(args: Args) -> Parameters {
    let request_kind = match args.command {
        None => RequestKind::CurrentFull,
        Some(Command::Now { summary: _, list }) => {
            if let Some(attributes) = list {
                let attribute_set = convert_to_attribute_set(&attributes);
                RequestKind::CurrentPartial(attribute_set)
            } else {
                RequestKind::CurrentFull
            }
        }
        Some(Command::Forecast {
            summary: _,
            list,
            today: _,
            days,
        }) => {
            let day_count = days.unwrap_or(1);
            if let Some(attributes) = list {
                let attribute_set = convert_to_attribute_set(&attributes);
                RequestKind::ForecastPartial(day_count, attribute_set)
            } else {
                RequestKind::ForecastFull(day_count)
            }
        }
    };
    Parameters {
        request_kind,
        coordinates: args.coords,
    }
}

pub fn parse() -> Parameters {
    let args = Args::parse();
    convert_args_to_parameters(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_now_command_when_none_is_provided() {
        let args = Args {
            command: None,
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        assert_eq!(params.request_kind, RequestKind::CurrentFull);
    }

    #[test]
    fn parses_now_command_with_summary_when_type_is_not_specified() {
        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: None,
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        assert_eq!(params.request_kind, RequestKind::CurrentFull);
    }

    #[test]
    fn parses_now_command_with_summary_specified() {
        let args = Args {
            command: Some(Command::Now {
                summary: true,
                list: None,
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        assert_eq!(params.request_kind, RequestKind::CurrentFull);
    }

    #[test]
    fn parses_now_command_with_list_without_attributes_specified() {
        let expected_attribute_set: WeatherAttributeSet = WeatherAttribute::iter().collect();
        let expected = RequestKind::CurrentPartial(expected_attribute_set);

        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: Some(Vec::new()),
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_now_command_with_list_with_attributes_specified() {
        let requested_attributes = vec![
            WeatherAttribute::WeatherKind,
            WeatherAttribute::Temperature,
            WeatherAttribute::Pressure,
            WeatherAttribute::Humidity,
        ];
        let expected_attribute_set = requested_attributes.iter().cloned().collect();
        let expected = RequestKind::CurrentPartial(expected_attribute_set);

        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: Some(requested_attributes),
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_forecast_command_without_params_defaults_to_today_summary() {
        let args = Args {
            command: Some(Command::Forecast {
                summary: false,
                list: None,
                today: false,
                days: None,
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        let expected = RequestKind::ForecastFull(DayCount::from(1));
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_forecast_command_with_summary_defaults_to_today_summary() {
        let args = Args {
            command: Some(Command::Forecast {
                summary: true,
                list: None,
                today: false,
                days: None,
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        let expected = RequestKind::ForecastFull(DayCount::from(1));
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_forecast_command_with_today_defaults_to_today_summary() {
        let args = Args {
            command: Some(Command::Forecast {
                summary: false,
                list: None,
                today: true,
                days: None,
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        let expected = RequestKind::ForecastFull(DayCount::from(1));
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_forecast_command_with_summary_today() {
        let args = Args {
            command: Some(Command::Forecast {
                summary: true,
                list: None,
                today: true,
                days: None,
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        let expected = RequestKind::ForecastFull(DayCount::from(1));
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_forecast_command_with_days_defaults_to_daily_summary() {
        const DAY_COUNT: DayCount = 4;
        let args = Args {
            command: Some(Command::Forecast {
                summary: false,
                list: None,
                today: false,
                days: Some(DAY_COUNT),
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        let expected = RequestKind::ForecastFull(DAY_COUNT);
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_forecast_command_with_summary_and_days() {
        const DAY_COUNT: DayCount = 4;
        let args = Args {
            command: Some(Command::Forecast {
                summary: true,
                list: None,
                today: false,
                days: Some(DAY_COUNT),
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        let expected = RequestKind::ForecastFull(DAY_COUNT);
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_forecast_command_with_list_without_attributes_specified() {
        let expected_attribute_set: WeatherAttributeSet = WeatherAttribute::iter().collect();
        let expected = RequestKind::ForecastPartial(DayCount::from(1), expected_attribute_set);

        let args = Args {
            command: Some(Command::Forecast {
                summary: false,
                list: Some(Vec::new()),
                today: false,
                days: None,
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_forecast_command_with_list_with_attributes_specified() {
        let requested_attributes = vec![
            WeatherAttribute::WeatherKind,
            WeatherAttribute::Temperature,
            WeatherAttribute::Pressure,
            WeatherAttribute::Humidity,
        ];
        let expected_attribute_set = requested_attributes.iter().cloned().collect();
        let expected = RequestKind::ForecastPartial(DayCount::from(1), expected_attribute_set);

        let args = Args {
            command: Some(Command::Forecast {
                summary: false,
                list: Some(requested_attributes),
                today: false,
                days: None,
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_forecast_command_with_list_and_today() {
        let requested_attributes = vec![
            WeatherAttribute::WeatherKind,
            WeatherAttribute::Temperature,
            WeatherAttribute::Pressure,
            WeatherAttribute::Humidity,
        ];
        let expected_attribute_set = requested_attributes.iter().cloned().collect();
        let expected = RequestKind::ForecastPartial(DayCount::from(1), expected_attribute_set);

        let args = Args {
            command: Some(Command::Forecast {
                summary: false,
                list: Some(requested_attributes),
                today: true,
                days: None,
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_forecast_command_with_list_and_days() {
        const DAY_COUNT: DayCount = 4;
        let requested_attributes = vec![
            WeatherAttribute::WeatherKind,
            WeatherAttribute::Temperature,
            WeatherAttribute::Pressure,
            WeatherAttribute::Humidity,
        ];
        let expected_attribute_set = requested_attributes.iter().cloned().collect();
        let expected = RequestKind::ForecastPartial(DAY_COUNT, expected_attribute_set);

        let args = Args {
            command: Some(Command::Forecast {
                summary: false,
                list: Some(requested_attributes),
                today: false,
                days: Some(DAY_COUNT),
            }),
            coords: None,
            here: false,
        };
        let params = convert_args_to_parameters(args);
        assert_eq!(params.request_kind, expected);
    }

    #[test]
    fn parses_coordinate_values() {
        let coordinates = Coordinates::new(1.23, 45.67);
        assert_eq!(Coordinates::from_str("1.23,45.67"), Ok(coordinates));
        let coordinates = Coordinates::new(1.0, -4.0);
        assert_eq!(Coordinates::from_str("1,-4"), Ok(coordinates));
    }

    #[test]
    fn raises_error_on_invalid_coordinates() {
        matches!(Coordinates::from_str("foo,45.67"), Err(_));
        matches!(Coordinates::from_str("1.23,bar"), Err(_));
        matches!(Coordinates::from_str("1.2,3.4,5.6"), Err(_));
    }

    #[test]
    fn parses_coordinates() {
        let coordinates = Coordinates::new(1.23, 45.67);
        let args = Args {
            command: None,
            coords: Some(coordinates.clone()),
            here: false,
        };
        let params = convert_args_to_parameters(args);
        assert_eq!(params.coordinates, Some(coordinates));
    }
}
