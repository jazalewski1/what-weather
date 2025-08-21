use crate::types::attributes::*;
use crate::types::units::*;
use clap::builder::PossibleValue;
use clap::{Parser, Subcommand, ValueEnum};
use std::str::FromStr;
use strum::IntoEnumIterator;

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
        &[
            Self::WeatherKind,
            Self::Temperature,
            Self::CloudCoverage,
            Self::Humidity,
            Self::Wind,
            Self::Pressure,
        ]
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
        /// Report all attributes as summary
        #[arg(long, group = "now_format")]
        summary: bool,

        /// Report all or selected attributes as a list
        #[arg(long, group = "now_format", value_delimiter=',', num_args=0..)]
        list: Option<Vec<WeatherAttribute>>,
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

#[derive(Debug, PartialEq, Eq)]
pub enum ReportType {
    CurrentSummary,
    CurrentList(WeatherAttributeSet),
}

pub struct Input {
    pub report_type: ReportType,
    pub coordinates: Option<Coordinates>,
}

fn convert_to_attribute_set(attributes: &[WeatherAttribute]) -> WeatherAttributeSet {
    if attributes.is_empty() {
        WeatherAttribute::iter().collect()
    } else {
        attributes.iter().cloned().collect()
    }
}

impl From<Args> for Input {
    fn from(args: Args) -> Self {
        let report_type = match args.command {
            None => ReportType::CurrentSummary,
            Some(Command::Now { summary, list }) => {
                if summary {
                    ReportType::CurrentSummary
                } else if let Some(attributes) = list {
                    let attribute_set = convert_to_attribute_set(&attributes);
                    ReportType::CurrentList(attribute_set)
                } else {
                    ReportType::CurrentSummary
                }
            }
        };
        Input {
            report_type,
            coordinates: args.coords,
        }
    }
}

pub fn parse() -> Input {
    Args::parse().into()
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
        let input: Input = args.into();
        assert_eq!(input.report_type, ReportType::CurrentSummary);
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
        let input: Input = args.into();
        assert_eq!(input.report_type, ReportType::CurrentSummary);
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
        let input: Input = args.into();
        assert_eq!(input.report_type, ReportType::CurrentSummary);
    }

    #[test]
    fn parses_now_command_with_list_without_attributes_specified() {
        let expected_attribute_set: WeatherAttributeSet = WeatherAttribute::iter().collect();
        let expected = ReportType::CurrentList(expected_attribute_set);

        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: Some(Vec::new()),
            }),
            coords: None,
            here: false,
        };
        let input: Input = args.into();
        assert_eq!(input.report_type, expected);
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
        let expected = ReportType::CurrentList(expected_attribute_set);

        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: Some(requested_attributes),
            }),
            coords: None,
            here: false,
        };
        let input: Input = args.into();
        assert_eq!(input.report_type, expected);
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
        let input: Input = args.into();
        assert_eq!(input.coordinates, Some(coordinates));
    }

    #[test]
    fn provides_all_weather_attribute_values() {
        let result: WeatherAttributeSet =
            WeatherAttribute::value_variants().iter().cloned().collect();
        let expected: WeatherAttributeSet = WeatherAttribute::iter().collect();
        assert_eq!(result, expected);
    }
}
