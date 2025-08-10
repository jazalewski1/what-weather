use crate::domain::model::{Reporter, Parameters};
use crate::domain::port::{GeolocationProvider, WeatherProvider, Presenter};
use crate::domain::types::WeatherQuery;

pub struct WeatherReporter<P: Presenter, GP: GeolocationProvider, WP: WeatherProvider> {
    presenter: P,
    geolocation_provider: GP,
    weather_provider: WP,
}

impl<P: Presenter, GP: GeolocationProvider, WP: WeatherProvider> WeatherReporter<P, GP, WP> {
    pub fn new(presenter: P, geolocation_provider: GP, weather_provider: WP) -> Self {
        WeatherReporter { geolocation_provider, weather_provider, presenter }
    }
}

impl<P: Presenter, GP: GeolocationProvider, WP: WeatherProvider> Reporter for WeatherReporter<P, GP, WP> {
    fn fetch_and_report(&self, _parameters: &Parameters) {
        let coordinates = self.geolocation_provider.get_current_coordinates();
        let query = WeatherQuery { coordinates };
        let report = self.weather_provider.fetch(&query);
        self.presenter.display(&report);
    }
}
