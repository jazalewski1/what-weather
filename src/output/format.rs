mod list;
mod summary;

use crate::types::WeatherReport;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Summary,
    List,
}

impl Format {
    pub fn describe(&self, report: &WeatherReport) -> String {
        match self {
            Self::Summary => summary::format(report),
            Self::List => list::format(report),
        }
    }
}
