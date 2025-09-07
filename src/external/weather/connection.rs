use crate::types::error::FetchError;
use serde::de::DeserializeOwned;

pub type Params = Vec<(String, String)>;

pub fn fetch_response<R: DeserializeOwned>(params: &Params) -> Result<R, FetchError> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&params)
        .send()
        .map_err(|_| FetchError::ConnectionFailure)?;
    response.json().map_err(|_| FetchError::DecodingFailure)
}
