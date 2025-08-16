use crate::types::units::Coordinates;

#[derive(Clone, Debug, PartialEq)]
pub struct FullQuery {
    pub coordinates: Coordinates,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartialQuery {
    pub coordinates: Coordinates,
    pub parameter_selection: ParameterSelection,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ParameterSelection {
    pub with_kind: bool,
    pub with_temperature: bool,
    pub with_cloud_coverage: bool,
    pub with_humidity: bool,
    pub with_wind: bool,
    pub with_pressure: bool,
}
