use crate::types::attributes::*;
use crate::types::units::Coordinates;
use crate::types::units::*;
use crate::types::weather::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FullRequest {
    pub coordinates: Coordinates,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartialRequest {
    pub coordinates: Coordinates,
    pub attributes: WeatherAttributeSet,
}

#[derive(Clone, Debug)]
pub struct FullResponse {
    pub kind: Kind,
    pub temperature: Temperature,
    pub cloud_coverage: Percentage,
    pub humidity: Percentage,
    pub wind: Wind,
    pub pressure: Hectopascal,
}

#[derive(Default, Clone, Debug)]
pub struct PartialResponse {
    pub kind: Option<Kind>,
    pub temperature: Option<Temperature>,
    pub cloud_coverage: Option<Percentage>,
    pub humidity: Option<Percentage>,
    pub wind: Option<Wind>,
    pub pressure: Option<Hectopascal>,
}

#[mockall::automock]
pub trait WeatherProvider {
    fn fetch_all(&self, request: &FullRequest) -> FullResponse;

    fn fetch_selected(&self, request: &PartialRequest) -> PartialResponse;
}
