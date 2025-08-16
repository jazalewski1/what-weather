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
}
