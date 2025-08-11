use crate::controller::adapters;
use crate::domain::model;
use crate::domain::model::Reporter;

pub fn run() {
    let reporter = model::WeatherReporter::new(
        Box::new(adapters::FakeGeolocationProvider),
        Box::new(adapters::FakeWeatherProvider),
        Box::new(adapters::ConsolePresenter),
    );
    reporter.fetch_and_report(&model::Parameters);
}
