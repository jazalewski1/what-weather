mod geolocation;
mod output;
mod reporter;
mod weather;

pub use geolocation::GeolocationProvider;
pub use output::Presenter;
pub use reporter::Reporter;
pub use weather::WeatherProvider;

#[cfg(test)]
pub mod mocks {
    pub use super::geolocation::MockGeolocationProvider;
    pub use super::output::MockPresenter;
    pub use super::reporter::MockReporter;
    pub use super::weather::MockWeatherProvider;
}
