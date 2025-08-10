use what_weather::adapters::FakeGeolocationProvider;
use what_weather::domain::port::GeolocationProvider;

fn main() {
    let coordinates = FakeGeolocationProvider.get_current_coordinates();
    let unused_variable = 0;
    println!("There is some weather at {coordinates:?}");
}
