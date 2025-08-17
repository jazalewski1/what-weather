use what_weather::domain;
use what_weather::external::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::input::cli;
use what_weather::output::format::{list, summary};
use what_weather::output::{ConsoleView, View};
use what_weather::port::{GeolocationProvider, WeatherProvider};

fn run(
    geolocation_provider: impl GeolocationProvider,
    weather_provider: impl WeatherProvider,
    view: impl View,
) {
    let parameters = cli::parse();
    let reporter = domain::WeatherReporter::new(geolocation_provider, weather_provider);
    let string: String = match parameters.report_type {
        cli::ReportType::Summary => {
            let report = reporter.fetch_all();
            summary::describe(&report)
        }
        cli::ReportType::List(parameters) => {
            let report = reporter.fetch_selected(&parameters);
            list::describe(&report)
        }
    };
    view.display(&string);
}

fn main() {
    run(FakeGeolocationProvider, FakeWeatherProvider, ConsoleView);
}
