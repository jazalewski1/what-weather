use crate::adapters::gateways;
use crate::domain::model;
use crate::domain::model::Reporter;

pub fn run() {
    let reporter = model::WeatherReporter::new(
        Box::new(gateways::FakeGeolocationProvider),
        Box::new(gateways::FakeWeatherProvider),
        Box::new(crate::controller::adapters::ConsolePresenter),
    );
    reporter.report_current_weather();
}
