mod geolocation;
mod output;
mod weather;

pub use geolocation::GeolocationProvider;
pub use output::Presenter;
pub use weather::WeatherProvider;

#[cfg(test)]
pub mod mocks {
    pub use super::geolocation::MockGeolocationProvider;
    pub use super::output::MockPresenter;
    pub use super::weather::MockWeatherProvider;
}
