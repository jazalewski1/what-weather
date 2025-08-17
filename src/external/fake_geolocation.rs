use crate::port::geolocation::*;
use crate::types::units::Coordinates;

pub struct FakeGeolocationProvider;

impl GeolocationProvider for FakeGeolocationProvider {
    fn get_current_coordinates(&self) -> Coordinates {
        Coordinates {
            latitude: 51.10694,
            longitude: 17.07731,
        }
    }
}
