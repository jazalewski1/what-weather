use crate::port::geolocation::*;
use crate::types::units::*;

pub struct FakeGeolocationProvider;

impl GeolocationProvider for FakeGeolocationProvider {
    fn get_current_coordinates(&self) -> Coordinates {
        Coordinates::new(51.10694, 17.07731)
    }
}
