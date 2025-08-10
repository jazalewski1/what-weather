mod geolocation;
mod weather;
mod output;

pub use geolocation::GeolocationProvider;
pub use weather::WeatherProvider;
pub use output::Presenter;

#[cfg(test)]
pub mod mocks {
    pub use super::geolocation::MockGeolocationProvider;
    pub use super::weather::MockWeatherProvider;
    pub use super::output::MockPresenter;
}
