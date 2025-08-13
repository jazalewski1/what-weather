mod geolocation;
mod output;
mod weather;
mod reporter;

pub use geolocation::GeolocationProvider;
pub use output::Presenter;
pub use weather::WeatherProvider;
pub use reporter::Reporter;

#[cfg(test)]
pub mod mocks {
    pub use super::geolocation::MockGeolocationProvider;
    pub use super::output::MockPresenter;
    pub use super::weather::MockWeatherProvider;
}
