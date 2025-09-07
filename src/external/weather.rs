mod connection;
mod query;
mod response;

use crate::port::weather::WeatherProvider;
use crate::port::weather::{ReportRequest, RequestKind};
use crate::types::attributes::*;
use crate::types::error::FetchError;
use crate::types::report::*;
use strum::IntoEnumIterator;

pub struct ConcreteWeatherProvider;

impl WeatherProvider for ConcreteWeatherProvider {
    fn fetch(&self, request: &ReportRequest) -> Result<Report, FetchError> {
        match &request.kind {
            RequestKind::PastFull(day_count) => {
                let attributes: WeatherAttributeSet = WeatherAttribute::iter().collect();
                let params =
                    query::build_past_params(&request.coordinates, *day_count, &attributes);
                let resp: response::DailyResponse = connection::fetch_response(&params)?;
                let inner = resp.to_daily_full_report(*day_count);
                Ok(Report::PastFull(inner))
            }
            RequestKind::PastPartial(day_count, attributes) => {
                let params = query::build_past_params(&request.coordinates, *day_count, attributes);
                let resp: response::DailyResponse = connection::fetch_response(&params)?;
                let inner = resp.to_daily_partial_report(&request.coordinates, *day_count);
                Ok(Report::PastPartial(inner))
            }
            RequestKind::CurrentFull => {
                let attributes: WeatherAttributeSet = WeatherAttribute::iter().collect();
                let _params = query::build_current_params(&request.coordinates, &attributes);
                todo!();
            }
            RequestKind::CurrentPartial(_attributes) => todo!(),
            RequestKind::ForecastFull(day_count) => {
                let attributes: WeatherAttributeSet = WeatherAttribute::iter().collect();
                let params =
                    query::build_forecast_params(&request.coordinates, *day_count, &attributes);
                let resp: response::DailyResponse = connection::fetch_response(&params)?;
                let inner = resp.to_daily_full_report(*day_count);
                Ok(Report::ForecastFull(inner))
            }
            RequestKind::ForecastPartial(day_count, attributes) => {
                let params =
                    query::build_forecast_params(&request.coordinates, *day_count, attributes);
                let resp: response::DailyResponse = connection::fetch_response(&params)?;
                let inner = resp.to_daily_partial_report(&request.coordinates, *day_count);
                Ok(Report::ForecastPartial(inner))
            }
        }
    }
}
