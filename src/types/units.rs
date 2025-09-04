use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Celsius {
    pub value: f32,
}

impl Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(1);
        write!(f, "{:.precision$}°C", self.value)
    }
}

impl From<f32> for Celsius {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl From<Celsius> for f32 {
    fn from(value: Celsius) -> Self {
        value.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Temperature {
    Celsius(Celsius),
}

impl Temperature {
    pub fn new_celsius(value: f32) -> Self {
        Self::Celsius(Celsius { value })
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Celsius(inner) => inner.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TemperatureRange {
    Celsius { min: Celsius, max: Celsius },
}

impl TemperatureRange {
    pub fn new_celsius(min: f32, max: f32) -> Self {
        assert!(min <= max, "Min temperature is greater than max");
        Self::Celsius {
            min: Celsius::from(min),
            max: Celsius::from(max),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Percentage {
    pub value: i8,
}

impl Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.value)
    }
}

impl From<i8> for Percentage {
    fn from(value: i8) -> Self {
        Self { value }
    }
}

impl From<Percentage> for i8 {
    fn from(percentage: Percentage) -> Self {
        percentage.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PercentageRange {
    pub min: Percentage,
    pub max: Percentage,
}

impl PercentageRange {
    pub fn new(min: i8, max: i8) -> Self {
        assert!(min <= max, "Min percentage is greater than max");
        Self {
            min: Percentage::from(min),
            max: Percentage::from(max),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetersPerSecond {
    pub value: f32,
}

impl Display for MetersPerSecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(1);
        write!(f, "{:.precision$} m/s", self.value)
    }
}

impl From<f32> for MetersPerSecond {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl From<MetersPerSecond> for f32 {
    fn from(speed: MetersPerSecond) -> Self {
        speed.value
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Speed {
    MetersPerSecond(MetersPerSecond),
}

impl Speed {
    pub fn new_meters_per_second(value: f32) -> Self {
        Self::MetersPerSecond(MetersPerSecond::from(value))
    }
}

impl Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MetersPerSecond(inner) => inner.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpeedRange {
    MetersPerSecond {
        min: MetersPerSecond,
        max: MetersPerSecond,
    },
}

impl SpeedRange {
    pub fn new_meters_per_second(min: f32, max: f32) -> Self {
        assert!(min <= max, "Min speed is greater than max");
        Self::MetersPerSecond {
            min: MetersPerSecond::from(min),
            max: MetersPerSecond::from(max),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Degrees {
    pub value: f32,
}

impl Degrees {
    pub fn raw(&self) -> f32 {
        self.value
    }
}

impl From<f32> for Degrees {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl From<Degrees> for f32 {
    fn from(degrees: Degrees) -> Self {
        degrees.value
    }
}

impl Display for Degrees {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(1);
        write!(f, "{:.precision$}°", self.value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Azimuth {
    pub angle: Degrees,
}

impl Azimuth {
    pub fn to_cardinal_direction(&self) -> CardinalDirection {
        let degrees = self.angle.value;
        if degrees <= 22.5 {
            CardinalDirection::North
        } else if degrees <= 67.5 {
            CardinalDirection::Northeast
        } else if degrees <= 112.5 {
            CardinalDirection::East
        } else if degrees <= 157.5 {
            CardinalDirection::Southeast
        } else if degrees <= 202.5 {
            CardinalDirection::South
        } else if degrees <= 247.5 {
            CardinalDirection::Southwest
        } else if degrees <= 292.5 {
            CardinalDirection::West
        } else if degrees <= 337.5 {
            CardinalDirection::Northwest
        } else {
            CardinalDirection::North
        }
    }
}

impl From<f32> for Azimuth {
    fn from(degrees: f32) -> Self {
        Self {
            angle: Degrees::from(degrees),
        }
    }
}

impl From<Azimuth> for f32 {
    fn from(azimuth: Azimuth) -> Self {
        azimuth.angle.value
    }
}

impl Display for Azimuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.angle.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardinalDirection {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl CardinalDirection {
    pub fn to_symbol(&self) -> String {
        match self {
            CardinalDirection::North => "N",
            CardinalDirection::Northeast => "NE",
            CardinalDirection::East => "E",
            CardinalDirection::Southeast => "SE",
            CardinalDirection::South => "S",
            CardinalDirection::Southwest => "SW",
            CardinalDirection::West => "W",
            CardinalDirection::Northwest => "NW",
        }
        .into()
    }

    pub fn to_name(&self) -> String {
        match self {
            CardinalDirection::North => "north",
            CardinalDirection::Northeast => "northeast",
            CardinalDirection::East => "east",
            CardinalDirection::Southeast => "southeast",
            CardinalDirection::South => "south",
            CardinalDirection::Southwest => "southwest",
            CardinalDirection::West => "west",
            CardinalDirection::Northwest => "northwest",
        }
        .into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hectopascal {
    pub value: f32,
}

impl From<f32> for Hectopascal {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl From<Hectopascal> for f32 {
    fn from(pressure: Hectopascal) -> Self {
        pressure.value
    }
}

impl Display for Hectopascal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(1);
        write!(f, "{:.precision$} hPa", self.value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pressure {
    Hpa(Hectopascal),
}

impl Pressure {
    pub fn new_hpa(value: f32) -> Self {
        Self::Hpa(Hectopascal::from(value))
    }
}

impl Display for Pressure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hpa(inner) => inner.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PressureRange {
    Hpa { min: Hectopascal, max: Hectopascal },
}

impl PressureRange {
    pub fn new_hpa(min: f32, max: f32) -> Self {
        assert!(min <= max, "Min pressure is greater than max");
        Self::Hpa {
            min: Hectopascal::from(min),
            max: Hectopascal::from(max),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinates {
    pub latitude: Degrees,
    pub longitude: Degrees,
}

impl Coordinates {
    pub fn new<T: Into<Degrees>>(latitude: T, longitude: T) -> Self {
        Self {
            latitude: latitude.into(),
            longitude: longitude.into(),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(1);
        write!(
            f,
            "{:.precision$}, {:.precision$}",
            self.latitude, self.longitude
        )
    }
}

pub type Date = chrono::NaiveDate;

pub type DayCount = u8;

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{UnwindSafe, catch_unwind};

    fn assert_panics<F, R>(func: F)
    where
        F: FnOnce() -> R + UnwindSafe,
    {
        let result = catch_unwind(func);
        assert!(result.is_err());
    }

    fn assert_no_panic<F, R>(func: F)
    where
        F: FnOnce() -> R + UnwindSafe,
    {
        let result = catch_unwind(func);
        assert!(result.is_ok());
    }

    #[test]
    fn displays_temperature_in_celsius() {
        let temperature = Temperature::new_celsius(-4_f32);
        assert_eq!(format!("{temperature}"), "-4.0°C");
        let temperature = Temperature::new_celsius(0.000);
        assert_eq!(format!("{temperature}"), "0.0°C");
        let temperature = Temperature::new_celsius(1.234);
        assert_eq!(format!("{temperature:.1}"), "1.2°C");
        let temperature = Temperature::new_celsius(34.56);
        assert_eq!(format!("{temperature:.3}"), "34.560°C");
    }

    #[test]
    fn validates_temperature_range() {
        assert_panics(|| TemperatureRange::new_celsius(32.0, 31.0));
        assert_no_panic(|| TemperatureRange::new_celsius(32.0, 32.0));
        assert_no_panic(|| TemperatureRange::new_celsius(32.0, 33.0));
    }

    #[test]
    fn displays_percentage() {
        let percentage = Percentage::from(27);
        assert_eq!(format!("{percentage}"), "27%");
    }

    #[test]
    fn validates_percentage_range() {
        assert_panics(|| PercentageRange::new(32, 31));
        assert_no_panic(|| PercentageRange::new(32, 32));
        assert_no_panic(|| PercentageRange::new(32, 33));
    }

    #[test]
    fn displays_speed_in_meters_per_second() {
        let speed = Speed::new_meters_per_second(0.0);
        assert_eq!(format!("{speed}"), "0.0 m/s");
        let speed = Speed::new_meters_per_second(12.345);
        assert_eq!(format!("{speed:.2}"), "12.35 m/s");
    }

    #[test]
    fn validates_speed_range() {
        assert_panics(|| SpeedRange::new_meters_per_second(32.0, 31.0));
        assert_no_panic(|| SpeedRange::new_meters_per_second(32.0, 32.0));
        assert_no_panic(|| SpeedRange::new_meters_per_second(32.0, 33.0));
    }

    #[test]
    fn displays_degrees() {
        let degrees = Degrees::from(-30.5);
        assert_eq!(format!("{degrees}"), "-30.5°");
        let degrees = Degrees::from(0.0);
        assert_eq!(format!("{degrees}"), "0.0°");
        let degrees = Degrees::from(12.456);
        assert_eq!(format!("{degrees:.2}"), "12.46°");
    }

    #[test]
    fn displays_azimuth() {
        let azimuth = Azimuth::from(-30.5);
        assert_eq!(format!("{azimuth}"), "-30.5°");
        let azimuth = Azimuth::from(0.0);
        assert_eq!(format!("{azimuth}"), "0.0°");
        let azimuth = Azimuth::from(12.456);
        assert_eq!(format!("{azimuth:.2}"), "12.46°");
    }

    #[test]
    fn converts_azimuth_to_cardinal_direction() {
        let convert = |value| Azimuth::from(value).to_cardinal_direction();

        assert_eq!(convert(337.6), CardinalDirection::North);
        assert_eq!(convert(345.0), CardinalDirection::North);
        assert_eq!(convert(359.9), CardinalDirection::North);
        assert_eq!(convert(0.0), CardinalDirection::North);
        assert_eq!(convert(13.1), CardinalDirection::North);
        assert_eq!(convert(22.5), CardinalDirection::North);

        assert_eq!(convert(22.6), CardinalDirection::Northeast);
        assert_eq!(convert(65.2), CardinalDirection::Northeast);
        assert_eq!(convert(67.5), CardinalDirection::Northeast);

        assert_eq!(convert(67.6), CardinalDirection::East);
        assert_eq!(convert(100.1), CardinalDirection::East);
        assert_eq!(convert(112.5), CardinalDirection::East);

        assert_eq!(convert(112.6), CardinalDirection::Southeast);
        assert_eq!(convert(121.9), CardinalDirection::Southeast);
        assert_eq!(convert(157.5), CardinalDirection::Southeast);

        assert_eq!(convert(157.6), CardinalDirection::South);
        assert_eq!(convert(200.0), CardinalDirection::South);
        assert_eq!(convert(202.5), CardinalDirection::South);

        assert_eq!(convert(202.6), CardinalDirection::Southwest);
        assert_eq!(convert(213.3), CardinalDirection::Southwest);
        assert_eq!(convert(247.5), CardinalDirection::Southwest);

        assert_eq!(convert(247.6), CardinalDirection::West);
        assert_eq!(convert(281.4), CardinalDirection::West);
        assert_eq!(convert(292.5), CardinalDirection::West);

        assert_eq!(convert(292.6), CardinalDirection::Northwest);
        assert_eq!(convert(293.5), CardinalDirection::Northwest);
        assert_eq!(convert(337.5), CardinalDirection::Northwest);
    }

    #[test]
    fn converts_cardinal_direction_to_symbol() {
        assert_eq!(CardinalDirection::North.to_symbol(), "N");
        assert_eq!(CardinalDirection::Northeast.to_symbol(), "NE");
        assert_eq!(CardinalDirection::East.to_symbol(), "E");
        assert_eq!(CardinalDirection::Southeast.to_symbol(), "SE");
        assert_eq!(CardinalDirection::South.to_symbol(), "S");
        assert_eq!(CardinalDirection::Southwest.to_symbol(), "SW");
        assert_eq!(CardinalDirection::West.to_symbol(), "W");
        assert_eq!(CardinalDirection::Northwest.to_symbol(), "NW");
    }

    #[test]
    fn converts_cardinal_direction_to_name() {
        assert_eq!(CardinalDirection::North.to_name(), "north");
        assert_eq!(CardinalDirection::Northeast.to_name(), "northeast");
        assert_eq!(CardinalDirection::East.to_name(), "east");
        assert_eq!(CardinalDirection::Southeast.to_name(), "southeast");
        assert_eq!(CardinalDirection::South.to_name(), "south");
        assert_eq!(CardinalDirection::Southwest.to_name(), "southwest");
        assert_eq!(CardinalDirection::West.to_name(), "west");
        assert_eq!(CardinalDirection::Northwest.to_name(), "northwest");
    }

    #[test]
    fn displays_hectopascals() {
        let pressure = Hectopascal::from(0.0);
        assert_eq!(format!("{pressure}"), "0.0 hPa");
        let pressure = Hectopascal::from(999.99);
        assert_eq!(format!("{pressure}"), "1000.0 hPa");
        let pressure = Hectopascal::from(1002.1234);
        assert_eq!(format!("{pressure:.2}"), "1002.12 hPa");
    }

    #[test]
    fn validates_pressure_range() {
        assert_panics(|| PressureRange::new_hpa(32.0, 31.0));
        assert_no_panic(|| PressureRange::new_hpa(32.0, 32.0));
        assert_no_panic(|| PressureRange::new_hpa(32.0, 33.0));
    }

    #[test]
    fn displays_coordinates() {
        let coordinates = Coordinates::new(1.234, -56.78);
        assert_eq!(format!("{coordinates:.5}"), "1.23400°, -56.78000°");
    }
}
