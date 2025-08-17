use crate::port::weather::{FullResponse, PartialResponse};
use crate::types::units::Coordinates;

pub struct FullReport {
    pub response: FullResponse,
}

pub struct PartialReport {
    pub coordinates: Coordinates,
    pub response: PartialResponse,
}
