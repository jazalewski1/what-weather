use what_weather::external::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::input::cli;
use what_weather::output::{ConsoleView, View};
use what_weather::weather_reporter::WeatherReporter;
use what_weather::format;

fn main() {
    let parameters = cli::parse();
    let weather_reporter = WeatherReporter::new(FakeGeolocationProvider, FakeWeatherProvider);
    let report = weather_reporter.run(parameters);
    let formatted_report = format::describe(&report);
    ConsoleView.display(&formatted_report);
}
