use what_weather::adapters::{FakeGeolocationProvider, FakeWeatherProvider, ConsolePresenter};
use what_weather::domain::port::{GeolocationProvider, Presenter, WeatherProvider};
use what_weather::domain::types::WeatherQuery;

fn main() {
    let coordinates = FakeGeolocationProvider.get_current_coordinates();
    let query = WeatherQuery { coordinates };
    let weather_report = FakeWeatherProvider.fetch(&query);
    ConsolePresenter.display(&weather_report);
}
