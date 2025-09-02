use crate::port::geolocation::*;
use crate::types::error::FetchError;
use crate::types::units::*;

pub struct FakeGeolocationProvider;

impl GeolocationProvider for FakeGeolocationProvider {
    fn fetch(&self) -> Result<Coordinates, FetchError> {
        // Temporary solution to simulate failures in demo.
        if std::env::var_os("WHAT_WEATHER_GP_ERROR").is_some() {
            Err(FetchError::ConnectionFailure)
        } else {
            Ok(Coordinates::new(51.10694, 17.07731))
        }
    }
}
