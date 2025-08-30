use what_weather::domain::strategies::*;
use what_weather::external::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::input::cli;
use what_weather::output::{ConsoleView, View};
use what_weather::types::units::{DayCount, Period};
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
        cli::ReportType::TodayForecastSummary => {
            let strategy = TodayForecastSummary::new(FakeWeatherProvider);
            weather_reporter.run(strategy, parameters)
        }
        cli::ReportType::DailyForecastSummary(length) => {
            let strategy = DailyForecastSummary::new(FakeWeatherProvider, get_period(length));
            weather_reporter.run(strategy, parameters)
        }
        cli::ReportType::TodayForecastList(attributes) => {
            let strategy = TodayForecastList::new(FakeWeatherProvider, attributes);
            weather_reporter.run(strategy, parameters)
        }
        cli::ReportType::DailyForecastList(attributes, length) => {
            let strategy =
                DailyForecastList::new(FakeWeatherProvider, attributes, get_period(length));
            weather_reporter.run(strategy, parameters)
        }
    };
    ConsoleView.display(&string);
}
