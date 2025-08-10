use crate::domain::types::Coordinates;

#[derive(Debug, PartialEq, Eq)]
pub enum WeatherKind {
    Sunny,
    Rain,
}

#[derive(Debug)]
pub struct WeatherReport {
    pub coordinates: Coordinates,
    pub kind: WeatherKind,
}
