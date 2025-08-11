use what_weather::controller::adapters;
use what_weather::domain::model;
use what_weather::domain::model::Reporter;

fn main() {
    let reporter = model::WeatherReporter::new(
        Box::new(adapters::FakeGeolocationProvider),
        Box::new(adapters::FakeWeatherProvider),
        Box::new(adapters::ConsolePresenter),
    );
    reporter.fetch_and_report(&model::Parameters);
}
