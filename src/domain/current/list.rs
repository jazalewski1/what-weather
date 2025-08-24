use crate::domain::ReportStrategy;
use crate::port::weather::WeatherProvider;
use crate::types::attributes::WeatherAttributeSet;
use crate::types::report::CurrentPartialReport;
use crate::types::units::Coordinates;
use crate::types::weather::*;

pub struct CurrentList<P: WeatherProvider> {
    weather_provider: P,
    attributes: WeatherAttributeSet,
}

impl<P: WeatherProvider> CurrentList<P> {
    pub fn new(weather_provider: P, attributes: WeatherAttributeSet) -> Self {
        Self {
            weather_provider,
            attributes,
        }
    }
}

impl<P: WeatherProvider> ReportStrategy for CurrentList<P> {
    type Report = CurrentPartialReport;

    fn fetch(&self, coordinates: &Coordinates) -> Self::Report {
        self.weather_provider
            .fetch_current_partial_report(coordinates, &self.attributes)
    }

    fn format(&self, report: &Self::Report) -> String {
        let mut builder = StringBuilder::default();

        builder.add("Coordinates", &format!("{:.5}", report.coordinates));
        if let Some(kind) = report.kind {
            builder.add("Weather", &describe_kind(&kind));
        }
        if let Some(temperature) = report.temperature {
            builder.add("Temperature", &format!("{temperature:.1}"));
        }
        if let Some(coverage) = report.cloud_coverage {
            builder.add("Cloud coverage", &format!("{coverage}"));
        }
        if let Some(humidity) = report.humidity {
            builder.add("Humidity", &format!("{humidity}"));
        }
        if let Some(wind) = &report.wind {
            builder.add("Wind", &describe_wind(wind));
        }
        if let Some(pressure) = report.pressure {
            builder.add("Pressure", &format!("{pressure}"));
        }
        builder.string()
    }
}

#[derive(Default)]
struct StringBuilder {
    string: String,
}

impl StringBuilder {
    fn add(&mut self, label: &str, value: &str) -> &mut StringBuilder {
        if self.string.is_empty() {
            self.string.push_str(&format!("{label}: {value}"));
        } else {
            self.string.push_str(&format!("\n{label}: {value}"));
        }
        self
    }
    fn string(self) -> String {
        self.string
    }
}

fn describe_kind(kind: &Kind) -> String {
    match kind {
        Kind::Clouds(clouds) => match clouds {
            Clouds::Clear => "clear sky".into(),
            Clouds::Light => "light clouds".into(),
            Clouds::Moderate => "cloudy".into(),
            Clouds::Dense => "overcast sky".into(),
        },
        Kind::Fog(fog) => match fog {
            Fog::Normal => "fog".into(),
            Fog::Rime => "rime fog".into(),
        },
        Kind::Precipitation(precipitation) => {
            let kind_desc = match precipitation.kind {
                PrecipitationKind::Rain => "rain",
                PrecipitationKind::Snow => "snow",
            };
            let intensity_desc = match precipitation.intensity {
                PrecipitationIntensity::Light => "light",
                PrecipitationIntensity::Moderate => "moderate",
                PrecipitationIntensity::Heavy => "heavy",
                PrecipitationIntensity::Shower => "shower",
            };
            if precipitation.heat == PrecipitationHeat::Freezing {
                format!("freezing {intensity_desc} {kind_desc}")
            } else {
                format!("{intensity_desc} {kind_desc}")
            }
        }
        Kind::Thunderstorm => "thunderstorm".into(),
    }
}

fn describe_wind(wind: &Wind) -> String {
    format!(
        "{:.1}, {} ({})",
        wind.speed,
        wind.direction,
        wind.direction.to_cardinal_direction().to_symbol()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::MockWeatherProvider;
    use crate::types::attributes::*;
    use crate::types::units::*;
    use mockall::predicate::eq;

    #[test]
    fn fetches_current_partial_report_with_provider() {
        let coordinates = Coordinates::new(1.23, 45.67);
        let weather_attributes = WeatherAttributeSet::from([
            WeatherAttribute::Temperature,
            WeatherAttribute::Humidity,
            WeatherAttribute::Pressure,
        ]);
        let report = CurrentPartialReport {
            coordinates,
            kind: None,
            temperature: Some(Temperature::new_celsius(36.6)),
            cloud_coverage: None,
            humidity: Some(Percentage::from(27)),
            wind: None,
            pressure: Some(Hectopascal::from(1001.2)),
        };

        let mut weather_provider = MockWeatherProvider::new();
        weather_provider
            .expect_fetch_current_partial_report()
            .with(eq(coordinates), eq(weather_attributes.clone()))
            .times(1)
            .return_const(report);

        let sut = CurrentList::new(weather_provider, weather_attributes);
        let _report = sut.fetch(&coordinates);
    }

    #[test]
    fn describes_values_of_clouds_kind() {
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Clear)), "clear sky");
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Light)), "light clouds");
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Moderate)), "cloudy");
        assert_eq!(describe_kind(&Kind::Clouds(Clouds::Dense)), "overcast sky");
    }

    #[test]
    fn describes_values_of_fog_kind() {
        assert_eq!(describe_kind(&Kind::Fog(Fog::Normal)), "fog");
        assert_eq!(describe_kind(&Kind::Fog(Fog::Rime)), "rime fog");
    }

    #[test]
    fn describes_values_of_precipitation_kind() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            describe_kind(&Kind::Precipitation(precipitation)),
            "light rain"
        );
        let precipitation = Precipitation {
            kind: PrecipitationKind::Snow,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            describe_kind(&Kind::Precipitation(precipitation)),
            "light snow"
        );
    }

    #[test]
    fn describes_values_of_precipitation_intensity() {
        let make_rain = |intensity| {
            let precipitation = Precipitation {
                kind: PrecipitationKind::Rain,
                intensity,
                heat: PrecipitationHeat::Normal,
            };
            Kind::Precipitation(precipitation)
        };
        assert_eq!(
            describe_kind(&make_rain(PrecipitationIntensity::Light)),
            "light rain"
        );
        assert_eq!(
            describe_kind(&make_rain(PrecipitationIntensity::Moderate)),
            "moderate rain"
        );
        assert_eq!(
            describe_kind(&make_rain(PrecipitationIntensity::Heavy)),
            "heavy rain"
        );
        assert_eq!(
            describe_kind(&make_rain(PrecipitationIntensity::Shower)),
            "shower rain"
        );
    }

    #[test]
    fn describes_values_of_precipitation_heat() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        assert_eq!(
            describe_kind(&Kind::Precipitation(precipitation)),
            "light rain"
        );
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Freezing,
        };
        assert_eq!(
            describe_kind(&Kind::Precipitation(precipitation)),
            "freezing light rain"
        );
    }

    #[test]
    fn describes_value_of_thunderstorm() {
        assert_eq!(describe_kind(&Kind::Thunderstorm), "thunderstorm");
    }

    #[test]
    fn describes_values_of_wind() {
        let wind = Wind {
            speed: Speed::new_meters_per_second(42.5),
            direction: Azimuth::from(200.2),
        };
        assert_eq!(describe_wind(&wind), "42.5 m/s, 200.2° (S)");
    }

    #[test]
    fn describes_all_attributes() {
        let coordinates = Coordinates::new(1.2345, 67.89);
        let report = CurrentPartialReport {
            coordinates,
            kind: Some(Kind::Clouds(Clouds::Light)),
            temperature: Some(Temperature::new_celsius(22.4)),
            cloud_coverage: Some(Percentage::from(43)),
            humidity: Some(Percentage::from(81)),
            wind: Some(Wind {
                speed: Speed::new_meters_per_second(1.07),
                direction: Azimuth::from(155.5),
            }),
            pressure: Some(Hectopascal::from(1009.3)),
        };
        let sut = CurrentList::new(MockWeatherProvider::new(), WeatherAttributeSet::new());
        let result = sut.format(&report);
        let expected = "Coordinates: 1.23450°, 67.89000°\n\
            Weather: light clouds\n\
            Temperature: 22.4°C\n\
            Cloud coverage: 43%\n\
            Humidity: 81%\n\
            Wind: 1.1 m/s, 155.5° (SE)\n\
            Pressure: 1009.3 hPa";
        assert_eq!(result, expected);
    }

    #[test]
    fn describes_only_selected_attributes() {
        let coordinates = Coordinates::new(1.2345, 67.89);
        let report = CurrentPartialReport {
            coordinates,
            kind: None,
            temperature: Some(Temperature::new_celsius(22.4)),
            cloud_coverage: None,
            humidity: Some(Percentage::from(81)),
            wind: Some(Wind {
                speed: Speed::new_meters_per_second(1.07),
                direction: Azimuth::from(155.5),
            }),
            pressure: None,
        };
        let sut = CurrentList::new(MockWeatherProvider::new(), WeatherAttributeSet::new());
        let result = sut.format(&report);
        let expected = "Coordinates: 1.23450°, 67.89000°\n\
            Temperature: 22.4°C\n\
            Humidity: 81%\n\
            Wind: 1.1 m/s, 155.5° (SE)";
        assert_eq!(result, expected);
    }
}
