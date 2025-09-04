use what_weather::external::{ConcreteGeolocationProvider, ConcreteWeatherProvider};
use what_weather::format;
use what_weather::input::cli;
use what_weather::output::{ConsoleView, View};
use what_weather::weather_reporter::WeatherReporter;

fn main() {
    let parameters = cli::parse();
    let weather_reporter = WeatherReporter::new(ConcreteGeolocationProvider, ConcreteWeatherProvider);
    match weather_reporter.run(parameters) {
        Ok(report) => {
            let formatted_report = format::describe(&report);
            ConsoleView.display(&formatted_report);
        }
        Err(error) => {
            eprintln!("Error: {error}");
        }
    }
}
