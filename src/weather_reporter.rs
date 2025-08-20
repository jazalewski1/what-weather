use crate::output::View;
use crate::port::geolocation::GeolocationProvider;
use crate::port::weather::*;
use crate::types::attributes::*;
use crate::types::units::Coordinates;

#[derive(Debug, Clone, PartialEq)]
pub enum ReportType {
    Summary,
    List(WeatherAttributeSet),
}

pub struct Parameters {
    pub report_type: ReportType,
    pub coordinates: Option<Coordinates>,
}

pub struct WeatherReporter<GP: GeolocationProvider, WP: WeatherProvider, V: View> {
    geolocation_provider: GP,
    weather_provider: WP,
    view: V,
}

impl<GP: GeolocationProvider, WP: WeatherProvider, V: View> WeatherReporter<GP, WP, V> {
    pub fn new(geolocation_provider: GP, weather_provider: WP, view: V) -> Self {
        Self {
            geolocation_provider,
            weather_provider,
            view,
        }
    }

    pub fn run(&self, parameters: Parameters) {
        let coordinates = if let Some(coords) = parameters.coordinates {
            coords
        } else {
            self.geolocation_provider.get_current_coordinates()
        };
        let string: String = match parameters.report_type {
            ReportType::Summary => unreachable!("Moved to strategy"),
            ReportType::List(_attributes) => unreachable!("Moved to strategy"),
        };
        self.view.display(&string);
    }
}
