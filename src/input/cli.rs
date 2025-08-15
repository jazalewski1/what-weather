use clap::{Parser, Subcommand};

use crate::output::Format;

#[derive(Subcommand)]
enum Command {
    /// Report current weather
    Now {
        /// Format report as summary
        #[arg(long, group = "now_format")]
        summary: bool,

        /// Format report as list of parameters
        #[arg(long, group = "now_format")]
        list: bool,
    },
}

#[derive(Parser)]
struct Args {
    /// Report type
    #[command(subcommand)]
    command: Option<Command>,
}

pub struct Parameters {
    pub report_format: Format,
}

impl From<Args> for Parameters {
    fn from(value: Args) -> Self {
        let report_format = match value.command {
            None => Format::Summary,
            Some(Command::Now { summary, list }) => {
                if summary {
                    Format::Summary
                } else if list {
                    Format::List
                } else {
                    Format::Summary
                }
            }
        };
        Parameters { report_format }
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
        assert_eq!(params.report_format, Format::Summary);
    }

    #[test]
    fn parse_now_command_with_summary_format_by_default() {
        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: false,
            }),
        };
        let params: Parameters = args.into();
        assert_eq!(params.report_format, Format::Summary);
    }

    #[test]
    fn parse_now_command_with_summary_format() {
        let args = Args {
            command: Some(Command::Now {
                summary: true,
                list: false,
            }),
        };
        let params: Parameters = args.into();
        assert_eq!(params.report_format, Format::Summary);
    }

    #[test]
    fn parse_now_command_with_list_format() {
        let args = Args {
            command: Some(Command::Now {
                summary: false,
                list: true,
            }),
        };
        let params: Parameters = args.into();
        assert_eq!(params.report_format, Format::List);
    }
}
