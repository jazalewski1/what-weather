#[derive(Debug, PartialEq, Eq)]
pub enum WeatherKind {
    Sunny,
    Rain,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WeatherReport {
    pub kind: WeatherKind,
}
