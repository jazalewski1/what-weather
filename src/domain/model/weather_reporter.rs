use crate::domain::model::{Parameters, Reporter};
use crate::domain::port::{GeolocationProvider, Presenter, WeatherProvider};
use crate::domain::types::WeatherQuery;

pub struct WeatherReporter {
    geolocation_provider: Box<dyn GeolocationProvider>,
    weather_provider: Box<dyn WeatherProvider>,
    presenter: Box<dyn Presenter>,
}

impl WeatherReporter {
    pub fn new(
        geolocation_provider: Box<dyn GeolocationProvider>,
        weather_provider: Box<dyn WeatherProvider>,
        presenter: Box<dyn Presenter>,
    ) -> Self {
        WeatherReporter {
            geolocation_provider,
            weather_provider,
            presenter,
        }
    }
}

impl Reporter for WeatherReporter {
    fn fetch_and_report(&self, _parameters: &Parameters) {
        let coordinates = self.geolocation_provider.get_current_coordinates();
        let query = WeatherQuery { coordinates };
        let report = self.weather_provider.fetch(&query);
        self.presenter.display(&report);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::port::mocks::*;
    use crate::domain::types::{Clouds, Coordinates, WeatherKind, WeatherReport};

    #[test]
    fn fetch_and_display_report() {
        let mut geolocation_provider = MockGeolocationProvider::new();
        geolocation_provider
            .expect_get_current_coordinates()
            .times(1)
            .returning(|| Coordinates {
                latitude: 1.2,
                longitude: 3.4,
            });

        let mut weather_provider = MockWeatherProvider::new();
        weather_provider
            .expect_fetch()
            .times(1)
            .returning(|_| WeatherReport {
                coordinates: Coordinates {
                    latitude: 1.2,
                    longitude: 3.4,
                },
                kind: WeatherKind::Clouds(Clouds::Light),
                temperature: 24.7,
                cloud_coverage: 47,
                humidity: 60,
            });
        let mut presenter = MockPresenter::new();
        presenter.expect_display().times(1).return_const(());

        let sut = WeatherReporter::new(
            Box::new(geolocation_provider),
            Box::new(weather_provider),
            Box::new(presenter),
        );

        sut.fetch_and_report(&Parameters);
    }
}
