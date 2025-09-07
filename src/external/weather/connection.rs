use crate::types::error::FetchError;
use serde::de::DeserializeOwned;

pub type Params = Vec<(String, String)>;

#[derive(Default)]
pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn fetch_response<R: DeserializeOwned>(&self, params: &Params) -> Result<R, FetchError> {
        self.client
            .get("https://api.open-meteo.com/v1/forecast")
            .query(&params)
            .send()
            .map_err(|_| FetchError::ConnectionFailure)?
            .json()
            .map_err(|_| FetchError::DecodingFailure)
    }
}
