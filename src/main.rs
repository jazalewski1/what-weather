use what_weather::external::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::input::cli;
use what_weather::output::ConsoleView;
use what_weather::weather_reporter;

fn main() {
    let parameters = cli::parse();
    let reporter = weather_reporter::WeatherReporter::new(
        FakeGeolocationProvider,
        FakeWeatherProvider,
        ConsoleView,
    );
    reporter.run(parameters);
}
