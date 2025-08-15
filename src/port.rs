mod geolocation;
mod weather;

pub use geolocation::GeolocationProvider;
pub use weather::WeatherProvider;

#[cfg(test)]
pub mod mocks {
    pub use super::geolocation::MockGeolocationProvider;
    pub use super::weather::MockWeatherProvider;
}
