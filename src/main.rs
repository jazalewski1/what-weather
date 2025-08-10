use what_weather::adapters::{FakeGeolocationProvider, FakeWeatherProvider};
use what_weather::domain::port::{GeolocationProvider, WeatherProvider};

fn main() {
    let coordinates = FakeGeolocationProvider.get_current_coordinates();
    let weather_report = FakeWeatherProvider.fetch();
    println!("There is some weather at {coordinates:?}");
}
