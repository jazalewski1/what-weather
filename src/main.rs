use what_weather::domain;
use what_weather::external::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::input::cli;
use what_weather::output::{ConsoleView, View};
use what_weather::port::{GeolocationProvider, WeatherProvider};

fn run(
    geolocation_provider: impl GeolocationProvider,
    weather_provider: impl WeatherProvider,
    view: impl View,
) {
    let parameters = cli::parse();
    let reporter = domain::WeatherReporter::new(geolocation_provider, weather_provider);
    let report = reporter.fetch();
    let string = parameters.report_format.describe(&report);
    view.display(&string);
}

fn main() {
    run(FakeGeolocationProvider, FakeWeatherProvider, ConsoleView);
}
