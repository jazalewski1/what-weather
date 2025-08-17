use crate::types::query::*;
use clap::builder::PossibleValue;
use clap::{Parser, Subcommand, ValueEnum};
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReportType {
    Summary,
    List(WeatherAttributeSet),
}

pub struct Parameters {
    pub report_type: ReportType,
}

fn convert_to_attribute_set(attributes: &[WeatherAttribute]) -> WeatherAttributeSet {
    if attributes.is_empty() {
        WeatherAttribute::iter().collect()
    } else {
        attributes.iter().cloned().collect()
    }
}

impl From<Args> for Parameters {
    fn from(value: Args) -> Self {
        let report_type = match value.command {
            None => ReportType::Summary,
            Some(Command::Now { summary, list }) => {
                if summary {
                    ReportType::Summary
                } else if let Some(attributes) = list {
                    let attribute_set = convert_to_attribute_set(&attributes);
                    ReportType::List(attribute_set)
                } else {
                    ReportType::Summary
                }
            }
        };
        Parameters { report_type }
    }
}

pub fn parse() -> Parameters {
    let args = Args::parse();
    Parameters::from(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_now_command_when_none_is_provided() {
        let args = Args { command: None };
        let params: Parameters = args.into();
        assert_eq!(params.report_type, ReportType::Summary);
    }

    #[test]
    fn parses_now_command_with_summary_when_type_is_not_specified() {
        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: None,
            }),
        };
        let params: Parameters = args.into();
        assert_eq!(params.report_type, ReportType::Summary);
    }

    #[test]
    fn parses_now_command_with_summary_specified() {
        let args = Args {
            command: Some(Command::Now {
                summary: true,
                list: None,
            }),
        };
        let params: Parameters = args.into();
        assert_eq!(params.report_type, ReportType::Summary);
    }

    #[test]
    fn parses_now_command_with_list_without_attributes_specified() {
        let expected_attribute_set: WeatherAttributeSet = WeatherAttribute::iter().collect();
        let expected = ReportType::List(expected_attribute_set);

        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: Some(Vec::new()),
            }),
        };
        let params: Parameters = args.into();
        assert_eq!(params.report_type, expected);
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
        let expected = ReportType::List(expected_attribute_set);

        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: Some(requested_attributes),
            }),
        };
        let params: Parameters = args.into();
        assert_eq!(params.report_type, expected);
    }

    #[test]
    fn provides_all_weather_attribute_values() {
        let result: WeatherAttributeSet =
            WeatherAttribute::value_variants().iter().cloned().collect();
        let expected: WeatherAttributeSet = WeatherAttribute::iter().collect();
        assert_eq!(result, expected);
    }
}
