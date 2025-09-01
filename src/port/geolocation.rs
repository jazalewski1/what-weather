use crate::types::error::FetchError;
use crate::types::units::Coordinates;

#[mockall::automock]
pub trait GeolocationProvider {
    fn fetch(&self) -> Result<Coordinates, FetchError>;
}
