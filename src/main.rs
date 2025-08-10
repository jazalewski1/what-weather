use what_weather::adapters::{FakeGeolocationProvider, FakeWeatherProvider, ConsolePresenter};
use what_weather::domain::model::{Parameters, Reporter, WeatherReporter};
fn main() {
    let reporter = WeatherReporter::new(ConsolePresenter, FakeGeolocationProvider, FakeWeatherProvider);
    reporter.fetch_and_report(&Parameters);
}
