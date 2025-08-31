mod common;
mod common_format;
mod current;
mod forecast;

use crate::types::report::Report;

pub fn describe(report: &Report) -> String {
    match report {
        Report::CurrentFull(inner) => current::summary::describe(&inner),
        Report::CurrentPartial(inner) => current::list::describe(&inner),
        Report::TodayForecastFull(inner) => forecast::today_summary::describe(&inner),
        Report::TodayForecastPartial(inner) => forecast::today_list::describe(&inner),
        Report::DailyForecastFull(inner) => forecast::daily_summary::describe(&inner),
        Report::DailyForecastPartial(inner) => forecast::daily_list::describe(&inner),
    }
}
