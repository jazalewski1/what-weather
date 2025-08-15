use what_weather::domain;
use what_weather::external::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::input::cli;
use what_weather::output::formatters::summary;
use what_weather::output::{ConsoleView, View};
use what_weather::port::{GeolocationProvider, WeatherProvider};

fn run(
    geolocation_provider: impl GeolocationProvider,
    weather_provider: impl WeatherProvider,
    view: impl View,
) {
    let reporter = domain::WeatherReporter::new(geolocation_provider, weather_provider);
    let report = reporter.fetch();
    let string = summary::format(&report);
    view.display(&string);
}

fn main() {
    let _parameters = cli::parse();
    run(FakeGeolocationProvider, FakeWeatherProvider, ConsoleView);
}
