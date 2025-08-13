use what_weather::adapters::gateways::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::adapters::controllers::cli;
use what_weather::domain::interactors::WeatherReporter;
use what_weather::view::ConsoleView;
use what_weather::adapters::presenters::SummaryPresenter;

fn main() {
    let presenter = SummaryPresenter::new(Box::new(ConsoleView));
    let reporter = WeatherReporter::new(
        Box::new(FakeGeolocationProvider),
        Box::new(FakeWeatherProvider),
        Box::new(presenter),
    );
    cli::run(Box::new(reporter))
}
