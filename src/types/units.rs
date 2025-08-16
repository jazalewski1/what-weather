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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_celsius() {
        let temperature = Celsius::from(-4_f32);
        assert_eq!(format!("{temperature}"), "-4.0°C");
        let temperature = Celsius::from(0.000);
        assert_eq!(format!("{temperature}"), "0.0°C");
        let temperature = Celsius::from(1.234);
        assert_eq!(format!("{temperature:.1}"), "1.2°C");
        let temperature = Celsius::from(34.56);
        assert_eq!(format!("{temperature:.3}"), "34.560°C");
    }
}
