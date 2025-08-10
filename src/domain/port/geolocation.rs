use crate::domain::types::Coordinates;

pub trait GeolocationProvider {
    fn get_current_coordinates(&self) -> Coordinates;
}
