mod common;
mod current;
mod forecast;

use crate::types::report::Report;

pub fn describe(report: &Report) -> String {
    match report {
        Report::CurrentFull(inner) => current::summary::describe(inner),
        Report::CurrentPartial(inner) => current::list::describe(inner),
        Report::ForecastFull(inner) => forecast::summary::describe(inner),
        Report::ForecastPartial(inner) => forecast::list::describe(inner),
    }
}
