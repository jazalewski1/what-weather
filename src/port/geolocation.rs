use crate::types::units::Coordinates;

#[mockall::automock]
pub trait GeolocationProvider {
    fn get_current_coordinates(&self) -> Coordinates;
}
