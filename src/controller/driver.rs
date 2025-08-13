use crate::adapters::gateways;
use crate::domain::model;
use crate::domain::model::Reporter;
use crate::view;

pub fn run() {
    let reporter = model::WeatherReporter::new(
        Box::new(gateways::FakeGeolocationProvider),
        Box::new(gateways::FakeWeatherProvider),
        Box::new(view::ConsoleView),
    );
    reporter.report_current_weather();
}
