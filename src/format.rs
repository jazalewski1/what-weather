mod common;
mod current;
mod daily;
mod forecast;
mod past;

use crate::types::report::Report;

pub fn describe(report: &Report) -> String {
    match report {
        Report::PastFull(inner) => past::summary::describe(inner),
        Report::PastPartial(inner) => daily::list::describe(inner),
        Report::CurrentFull(inner) => current::summary::describe(inner),
        Report::CurrentPartial(inner) => current::list::describe(inner),
        Report::ForecastFull(inner) => forecast::summary::describe(inner),
        Report::ForecastPartial(inner) => daily::list::describe(inner),
    }
}
