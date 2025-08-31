use what_weather::domain::strategies::*;
use what_weather::external::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::input::cli;
use what_weather::output::{ConsoleView, View};
use what_weather::weather_reporter::{self, Parameters};

fn main() {
    let parameters = cli::parse();
    let weather_reporter = weather_reporter::WeatherReporter::new(FakeGeolocationProvider);
    // ConsoleView.display(&string);
}
