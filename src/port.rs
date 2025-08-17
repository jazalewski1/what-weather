pub mod geolocation;
pub mod weather;

#[cfg(test)]
pub mod mocks {
    pub use super::geolocation::MockGeolocationProvider;
    pub use super::weather::MockWeatherProvider;
}
