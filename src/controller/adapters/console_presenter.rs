use crate::domain::port::Presenter;
use crate::domain::types::{WeatherKind, WeatherReport};

pub struct ConsolePresenter;

fn format(report: &WeatherReport) -> String {
    let kind_text = match report.kind {
        WeatherKind::Sunny => "It is sunny",
        WeatherKind::Rain => "It rains",
    };
    format!("{kind_text} at {:?}", report.coordinates)
}

impl Presenter for ConsolePresenter {
    fn display(&self, report: &WeatherReport) {
        let text = format(report);
        println!("{text}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::Coordinates;

    #[test]
    fn format_sunny_weather() {
        let report = WeatherReport {
            coordinates: Coordinates {
                latitude: 1.2,
                longitude: 3.4,
            },
            kind: WeatherKind::Sunny,
        };
        let string = format(&report);
        assert!(string.starts_with("It is sunny at "));
    }

    #[test]
    fn format_rainy_weather() {
        let report = WeatherReport {
            coordinates: Coordinates {
                latitude: 1.2,
                longitude: 3.4,
            },
            kind: WeatherKind::Rain,
        };
        let string = format(&report);
        assert!(string.starts_with("It rains at "));
    }
}
