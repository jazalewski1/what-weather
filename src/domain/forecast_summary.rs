use super::ReportStrategy;
use crate::domain::common_format::*;
use crate::port::weather::WeatherProvider;
use crate::types::report::ForecastFullReport;
use crate::types::units::*;
use crate::types::weather::*;

pub struct ForecastSummary<P: WeatherProvider> {
    weather_provider: P,
}

impl<P: WeatherProvider> ForecastSummary<P> {
    pub fn new(weather_provider: P) -> Self {
        Self { weather_provider }
    }
}

impl<P: WeatherProvider> ReportStrategy for ForecastSummary<P> {
    type Report = ForecastFullReport;

    fn fetch(&self, coordinates: &Coordinates) -> Self::Report {
        self.weather_provider
            .fetch_forecast_full_report(coordinates)
    }

    fn format(&self, report: &Self::Report) -> String {
        let temperature_desc = describe_temperature_range(&report.temperature_range);
        let _kind_desc = describe_kind(&report.kind);
        format!("Today {temperature_desc}.")
    }
}

fn describe_kind(kind: &Kind) -> String {
    let desc = prepare_kind_description(kind);
    match desc {
        KindDescription::Clouds { sky_adjective } => format!("The sky will be {sky_adjective}"),
        KindDescription::Fog { description } => {
            format!("A {description} will be covering the area")
        }
        KindDescription::Precipitation { description } => {
            format!("There will be {description} falling")
        }
        KindDescription::Thunderstorm { description } => format!("A {description} will be raging"),
    }
}

fn describe_temperature_range(temperature_range: &TemperatureRange) -> String {
    match temperature_range {
        TemperatureRange::Celsius { min, max } => {
            let adjective = describe_temperature_adjective(&Temperature::Celsius(*max));
            format!("it will be {adjective} with temperatures starting at {min} and reaching {max}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::MockWeatherProvider;
    use mockall::predicate::eq;

    #[test]
    fn fetches_forecast_full_report() {
        let mut weather_provider = MockWeatherProvider::new();
        let coordinates = Coordinates::new(1.23, 45.67);
        let report = ForecastFullReport {
            kind: Kind::Clouds(Clouds::Dense),
            temperature_range: TemperatureRange::new_celsius(12.3, 23.4),
        };
        weather_provider
            .expect_fetch_forecast_full_report()
            .once()
            .with(eq(coordinates))
            .return_const(report);

        let sut = ForecastSummary::new(weather_provider);
        sut.fetch(&coordinates);
    }

    #[test]
    fn describes_temperature_range() {
        let range = TemperatureRange::new_celsius(15.1, 33.3);
        let result = describe_temperature_range(&range);
        let expected = "it will be hot with temperatures starting at 15.1°C and reaching 33.3°C";
        assert_eq!(result, expected);
    }

    #[test]
    fn describes_cloud_kind() {
        let kind = Kind::Clouds(Clouds::Dense);
        let result = describe_kind(&kind);
        let expected = "The sky will be overcast";
        assert_eq!(result, expected);
    }

    #[test]
    fn describes_fog_kind() {
        let kind = Kind::Fog(Fog::Normal);
        let result = describe_kind(&kind);
        let expected = "A fog will be covering the area";
        assert_eq!(result, expected);
    }

    #[test]
    fn describes_precipitation_kind() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        };
        let kind = Kind::Precipitation(precipitation);
        let result = describe_kind(&kind);
        let expected = "There will be moderate rain falling";
        assert_eq!(result, expected);
    }

    #[test]
    fn describes_thunderstorm_kind() {
        let kind = Kind::Thunderstorm;
        let result = describe_kind(&kind);
        let expected = "A thunderstorm will be raging";
        assert_eq!(result, expected);
    }

    #[test]
    fn describes_entire_report() {
        let sut = ForecastSummary::new(MockWeatherProvider::new());
        let report = ForecastFullReport {
            kind: Kind::Clouds(Clouds::Dense),
            temperature_range: TemperatureRange::new_celsius(12.3, 23.4),
        };
        let result = sut.format(&report);
        let expected =
            "Today it will be warm with temperatures starting at 12.3°C and reaching 23.4°C.";
        assert_eq!(result, expected);
    }
}
