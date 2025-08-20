use crate::port::weather::PartialResponse;
use crate::types::units::Coordinates;
use crate::types::units::*;
use crate::types::weather::*;

pub struct PartialReport {
    pub coordinates: Coordinates,
    pub response: PartialResponse,
}

#[derive(Clone, Debug)]
pub struct CurrentFullReport {
    pub kind: Kind,
    pub temperature: Temperature,
    pub cloud_coverage: Percentage,
    pub humidity: Percentage,
    pub wind: Wind,
    pub pressure: Hectopascal,
}
