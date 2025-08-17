use crate::port::geolocation::*;
use crate::types::units::*;

pub struct FakeGeolocationProvider;

impl GeolocationProvider for FakeGeolocationProvider {
    fn get_current_coordinates(&self) -> Coordinates {
        Coordinates {
            latitude: Degrees::from(51.10694),
            longitude: Degrees::from(17.07731),
        }
    }
}
