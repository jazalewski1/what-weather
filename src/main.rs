use what_weather::adapters::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::domain::port::{GeolocationProvider, WeatherProvider};
use what_weather::domain::types::WeatherQuery;

fn main() {
    let coordinates = FakeGeolocationProvider.get_current_coordinates();
    let query = WeatherQuery { coordinates };
    let weather_report = FakeWeatherProvider.fetch(&query);
    println!("There is some weather at {coordinates:?}");
}
