use crate::domain::{
    port::GeolocationProvider,
    types::Coordinates,
};

pub struct FakeGeolocationProvider;

impl GeolocationProvider for FakeGeolocationProvider {
    fn get_current_coordinates(&self) -> Coordinates {
        Coordinates { latitude: 51.106941500927306, longitude: 17.077312228247923 }
    }
}
