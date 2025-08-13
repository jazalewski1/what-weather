use crate::adapters::gateways::{FakeGeolocationProvider, FakeWeatherProvider};
use crate::adapters::controllers::cli;
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
    cli::run(Box::new(reporter))
}
