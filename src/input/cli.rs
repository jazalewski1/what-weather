use crate::types::query::*;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum WeatherParameter {
    WeatherKind,
    Temperature,
    CloudCoverage,
    Humidity,
    Wind,
    Pressure,
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
    List(ParameterSelection),
}

pub struct Parameters {
    pub report_type: ReportType,
}

fn convert_to_parameter_selection(params: &[WeatherParameter]) -> ParameterSelection {
    if params.is_empty() {
        ParameterSelection {
            with_kind: true,
            with_temperature: true,
            with_cloud_coverage: true,
            with_humidity: true,
            with_wind: true,
            with_pressure: true,
        }
    } else {
        let mut selection = ParameterSelection::default();
        for param in params {
            match param {
                WeatherParameter::WeatherKind => selection.with_kind = true,
                WeatherParameter::Temperature => selection.with_temperature = true,
                WeatherParameter::CloudCoverage => selection.with_cloud_coverage = true,
                WeatherParameter::Humidity => selection.with_humidity = true,
                WeatherParameter::Wind => selection.with_wind = true,
                WeatherParameter::Pressure => selection.with_pressure = true,
            }
        }
        selection
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
        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: Some(Vec::new()),
            }),
        };
        let params: Parameters = args.into();
        let expected = ReportType::List(ParameterSelection {
            with_kind: true,
            with_temperature: true,
            with_cloud_coverage: true,
            with_humidity: true,
            with_wind: true,
            with_pressure: true,
        });
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
        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: Some(requested_paramaters),
            }),
        };
        let params: Parameters = args.into();
        let expected = ReportType::List(ParameterSelection {
            with_kind: true,
            with_temperature: true,
            with_cloud_coverage: false,
            with_humidity: true,
            with_wind: false,
            with_pressure: true,
        });
        assert_eq!(params.report_type, expected);
    }
}
