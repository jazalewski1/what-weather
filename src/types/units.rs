use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct Azimuth {
    pub angle: f32,
}

impl Azimuth {
    pub fn to_cardinal_direction(&self) -> CardinalDirection {
        if self.angle <= 22.5 {
            CardinalDirection::North
        } else if self.angle <= 67.5 {
            CardinalDirection::Northeast
        } else if self.angle <= 112.5 {
            CardinalDirection::East
        } else if self.angle <= 157.5 {
            CardinalDirection::Southeast
        } else if self.angle <= 202.5 {
            CardinalDirection::South
        } else if self.angle <= 247.5 {
            CardinalDirection::Southwest
        } else if self.angle <= 292.5 {
            CardinalDirection::West
        } else if self.angle <= 337.5 {
            CardinalDirection::Northwest
        } else {
            CardinalDirection::North
        }
    }
}

impl From<f32> for Azimuth {
    fn from(angle: f32) -> Self {
        Self { angle }
    }
}

impl From<Azimuth> for f32 {
    fn from(speed: Azimuth) -> Self {
        speed.angle
    }
}

impl Display for Azimuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(1);
        write!(f, "{:.precision$}°", self.angle)
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
        let s = match self {
            CardinalDirection::North => "N",
            CardinalDirection::Northeast => "NE",
            CardinalDirection::East => "E",
            CardinalDirection::Southeast => "SE",
            CardinalDirection::South => "S",
            CardinalDirection::Southwest => "SW",
            CardinalDirection::West => "W",
            CardinalDirection::Northwest => "NW",
        };
        s.into()
    }

    pub fn to_name(&self) -> String {
        let s = match self {
            CardinalDirection::North => "north",
            CardinalDirection::Northeast => "northeast",
            CardinalDirection::East => "east",
            CardinalDirection::Southeast => "southeast",
            CardinalDirection::South => "south",
            CardinalDirection::Southwest => "southwest",
            CardinalDirection::West => "west",
            CardinalDirection::Northwest => "northwest",
        };
        s.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_temperature_in_celsius() {
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
    fn display_percentage() {
        let percentage = Percentage::from(27);
        assert_eq!(format!("{percentage}"), "27%");
    }

    #[test]
    fn display_meters_per_second() {
        let speed = Speed::new_meters_per_second(0.0);
        assert_eq!(format!("{speed}"), "0.0 m/s");
        let speed = Speed::new_meters_per_second(12.345);
        assert_eq!(format!("{speed:.2}"), "12.35 m/s");
    }

    #[test]
    fn display_azimuth() {
        let azimuth = Azimuth::from(-30.5);
        assert_eq!(format!("{azimuth}"), "-30.5°");
        let azimuth = Azimuth::from(0.0);
        assert_eq!(format!("{azimuth}"), "0.0°");
        let azimuth = Azimuth::from(12.456);
        assert_eq!(format!("{azimuth:.2}"), "12.46°");
    }

    #[test]
    fn describe_wind_direction() {
        assert_eq!(
            Azimuth::from(337.6).to_cardinal_direction(),
            CardinalDirection::North
        );
        assert_eq!(
            Azimuth::from(345.0).to_cardinal_direction(),
            CardinalDirection::North
        );
        assert_eq!(
            Azimuth::from(359.9).to_cardinal_direction(),
            CardinalDirection::North
        );
        assert_eq!(
            Azimuth::from(0.0).to_cardinal_direction(),
            CardinalDirection::North
        );
        assert_eq!(
            Azimuth::from(13.1).to_cardinal_direction(),
            CardinalDirection::North
        );
        assert_eq!(
            Azimuth::from(22.5).to_cardinal_direction(),
            CardinalDirection::North
        );

        assert_eq!(
            Azimuth::from(22.6).to_cardinal_direction(),
            CardinalDirection::Northeast
        );
        assert_eq!(
            Azimuth::from(65.2).to_cardinal_direction(),
            CardinalDirection::Northeast
        );
        assert_eq!(
            Azimuth::from(67.5).to_cardinal_direction(),
            CardinalDirection::Northeast
        );

        assert_eq!(
            Azimuth::from(67.6).to_cardinal_direction(),
            CardinalDirection::East
        );
        assert_eq!(
            Azimuth::from(100.1).to_cardinal_direction(),
            CardinalDirection::East
        );
        assert_eq!(
            Azimuth::from(112.5).to_cardinal_direction(),
            CardinalDirection::East
        );

        assert_eq!(
            Azimuth::from(112.6).to_cardinal_direction(),
            CardinalDirection::Southeast
        );
        assert_eq!(
            Azimuth::from(121.9).to_cardinal_direction(),
            CardinalDirection::Southeast
        );
        assert_eq!(
            Azimuth::from(157.5).to_cardinal_direction(),
            CardinalDirection::Southeast
        );

        assert_eq!(
            Azimuth::from(157.6).to_cardinal_direction(),
            CardinalDirection::South
        );
        assert_eq!(
            Azimuth::from(200.0).to_cardinal_direction(),
            CardinalDirection::South
        );
        assert_eq!(
            Azimuth::from(202.5).to_cardinal_direction(),
            CardinalDirection::South
        );

        assert_eq!(
            Azimuth::from(202.6).to_cardinal_direction(),
            CardinalDirection::Southwest
        );
        assert_eq!(
            Azimuth::from(213.3).to_cardinal_direction(),
            CardinalDirection::Southwest
        );
        assert_eq!(
            Azimuth::from(247.5).to_cardinal_direction(),
            CardinalDirection::Southwest
        );

        assert_eq!(
            Azimuth::from(247.6).to_cardinal_direction(),
            CardinalDirection::West
        );
        assert_eq!(
            Azimuth::from(281.4).to_cardinal_direction(),
            CardinalDirection::West
        );
        assert_eq!(
            Azimuth::from(292.5).to_cardinal_direction(),
            CardinalDirection::West
        );

        assert_eq!(
            Azimuth::from(292.6).to_cardinal_direction(),
            CardinalDirection::Northwest
        );
        assert_eq!(
            Azimuth::from(293.5).to_cardinal_direction(),
            CardinalDirection::Northwest
        );
        assert_eq!(
            Azimuth::from(337.5).to_cardinal_direction(),
            CardinalDirection::Northwest
        );
    }

    #[test]
    fn cardinal_direction_as_symbol() {
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
    fn cardinal_direction_as_name() {
        assert_eq!(CardinalDirection::North.to_name(), "north");
        assert_eq!(CardinalDirection::Northeast.to_name(), "northeast");
        assert_eq!(CardinalDirection::East.to_name(), "east");
        assert_eq!(CardinalDirection::Southeast.to_name(), "southeast");
        assert_eq!(CardinalDirection::South.to_name(), "south");
        assert_eq!(CardinalDirection::Southwest.to_name(), "southwest");
        assert_eq!(CardinalDirection::West.to_name(), "west");
        assert_eq!(CardinalDirection::Northwest.to_name(), "northwest");
    }
}
