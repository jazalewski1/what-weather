use what_weather::domain::current_list::CurrentList;
use what_weather::domain::current_summary::CurrentSummary;
use what_weather::domain::forecast_summary::ForecastSummary;
use what_weather::external::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::input::cli;
use what_weather::output::{ConsoleView, View};
use what_weather::weather_reporter::{self, Parameters};

fn main() {
    let input = cli::parse();
    let weather_reporter = weather_reporter::WeatherReporter::new(FakeGeolocationProvider);
    let parameters = Parameters {
        coordinates: input.coordinates,
    };
    let string = match input.report_type {
        cli::ReportType::CurrentSummary => {
            let strategy = CurrentSummary::new(FakeWeatherProvider);
            weather_reporter.run(strategy, parameters)
        }
        cli::ReportType::CurrentList(attributes) => {
            let strategy = CurrentList::new(FakeWeatherProvider, attributes);
            weather_reporter.run(strategy, parameters)
        }
        cli::ReportType::ForecastSummary => {
            let strategy = ForecastSummary::new(FakeWeatherProvider);
            weather_reporter.run(strategy, parameters)
        }
    };
    ConsoleView.display(&string);
}
