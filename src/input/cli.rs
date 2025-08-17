use crate::types::query::*;
use clap::builder::PossibleValue;
use clap::{Parser, Subcommand, ValueEnum};
use strum::IntoEnumIterator;

impl ValueEnum for WeatherParameter {
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
        /// Report all parameters as summary
        #[arg(long, group = "now_format")]
        summary: bool,

        /// Report all or selected parameters as a list
        #[arg(long, group = "now_format", value_delimiter=',', num_args=0..)]
        list: Option<Vec<WeatherParameter>>,
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
    List(WeatherParameterSet),
}

pub struct Parameters {
    pub report_type: ReportType,
}

fn convert_to_parameter_selection(params: &[WeatherParameter]) -> WeatherParameterSet {
    if params.is_empty() {
        WeatherParameter::iter().collect()
    } else {
        params.iter().cloned().collect()
    }
}

impl From<Args> for Parameters {
    fn from(value: Args) -> Self {
        let report_type = match value.command {
            None => ReportType::Summary,
            Some(Command::Now { summary, list }) => {
                if summary {
                    ReportType::Summary
                } else if let Some(weather_parameters) = list {
                    let selection = convert_to_parameter_selection(&weather_parameters);
                    ReportType::List(selection)
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
    fn parse_now_command_by_default() {
        let args = Args { command: None };
        let params: Parameters = args.into();
        assert_eq!(params.report_type, ReportType::Summary);
    }

    #[test]
    fn parse_now_command_with_summary_report_by_default() {
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
    fn parse_now_command_with_summary_report() {
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
    fn parse_now_command_with_list_report_without_parameters() {
        let expected_parameter_set: WeatherParameterSet = WeatherParameter::iter().collect();
        let expected = ReportType::List(expected_parameter_set);

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
    fn parse_now_command_with_list_report_with_parameters() {
        let requested_paramaters = vec![
            WeatherParameter::WeatherKind,
            WeatherParameter::Temperature,
            WeatherParameter::Pressure,
            WeatherParameter::Humidity,
        ];
        let expected_parameter_set = requested_paramaters.iter().cloned().collect();
        let expected = ReportType::List(expected_parameter_set);

        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: Some(requested_paramaters),
            }),
        };
        let params: Parameters = args.into();
        assert_eq!(params.report_type, expected);
    }

    #[test]
    fn verify_all_weather_parameters_variants_are_covered() {
        let result: WeatherParameterSet =
            WeatherParameter::value_variants().iter().cloned().collect();
        let expected: WeatherParameterSet = WeatherParameter::iter().collect();
        assert_eq!(result, expected);
    }
}
