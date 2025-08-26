use crate::types::attributes::WeatherAttributeSet;
use crate::types::report::*;
use crate::types::units::*;

#[mockall::automock]
pub trait WeatherProvider {
    fn fetch_current_full_report(&self, coordinates: &Coordinates) -> CurrentFullReport;

    fn fetch_current_partial_report(
        &self,
        coordinates: &Coordinates,
        attributes: &WeatherAttributeSet,
    ) -> CurrentPartialReport;

    fn fetch_forecast_full_report(&self, coordinates: &Coordinates) -> TodayForecastFullReport;

    fn fetch_daily_forecast_full_report(
        &self,
        coordinates: &Coordinates,
        period: &Period,
    ) -> DailyForecastFullReport;

    fn fetch_today_forecast_partial_report(
        &self,
        coordinates: &Coordinates,
        attributes: &WeatherAttributeSet,
    ) -> TodayForecastPartialReport;

    fn fetch_daily_forecast_partial_report(
        &self,
        coordinates: &Coordinates,
        period: &Period,
        attributes: &WeatherAttributeSet,
    ) -> DailyForecastPartialReport;
}
