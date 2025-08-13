use crate::adapters::gateways::{FakeGeolocationProvider, FakeWeatherProvider};
use crate::domain::port::Reporter;
use crate::domain::interactors::WeatherReporter;
use crate::view::ConsoleView;
use crate::adapters::presenters::SummaryPresenter;

pub fn run() {
    let presenter = SummaryPresenter::new(Box::new(ConsoleView));
    let reporter = WeatherReporter::new(
        Box::new(FakeGeolocationProvider),
        Box::new(FakeWeatherProvider),
        Box::new(presenter),
    );
    reporter.report_current_weather();
}
