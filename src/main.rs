use what_weather::adapters::{FakeGeolocationProvider, FakeWeatherProvider, ConsolePresenter};
use what_weather::domain::model::{Parameters, Reporter, WeatherReporter};

fn main() {
    let reporter = WeatherReporter::new(Box::new(FakeGeolocationProvider), Box::new(FakeWeatherProvider), Box::new(ConsolePresenter));
    reporter.fetch_and_report(&Parameters);
}
