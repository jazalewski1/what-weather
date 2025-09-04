use crate::port::geolocation::*;
use crate::types::error::FetchError;
use crate::types::units::*;
use serde::{Deserialize, Serialize};

pub struct ConcreteGeolocationProvider;

impl GeolocationProvider for ConcreteGeolocationProvider {
    fn fetch(&self) -> Result<Coordinates, FetchError> {
        const URL: &str = "http://ip-api.com/json/?fields=status,message,lat,lon";
        let response = match reqwest::blocking::get(URL) {
            Ok(result) => result,
            Err(_) => return Err(FetchError::ConnectionFailure),
        };
        match response.json::<CoordinatesResponse>() {
            Ok(result) => Ok(Coordinates::new(result.lat, result.lon)),
            Err(_) => Err(FetchError::DecodingFailure),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CoordinatesResponse {
    pub status: String,
    pub lat: f32,
    pub lon: f32,
}
