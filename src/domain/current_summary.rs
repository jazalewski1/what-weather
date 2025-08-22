use super::ReportStrategy;
use crate::domain::common_format::*;
use crate::port::weather::WeatherProvider;
use crate::types::report::CurrentFullReport;
use crate::types::units::Coordinates;
use crate::types::units::*;
use crate::types::weather::*;

pub struct CurrentSummary<P: WeatherProvider> {
    weather_provider: P,
}

impl<P: WeatherProvider> CurrentSummary<P> {
    pub fn new(weather_provider: P) -> Self {
        Self { weather_provider }
    }
}

impl<P: WeatherProvider> ReportStrategy for CurrentSummary<P> {
    type Report = CurrentFullReport;

    fn fetch(&self, coordinates: &Coordinates) -> Self::Report {
        self.weather_provider.fetch_current_full_report(coordinates)
    }

    fn format(&self, report: &Self::Report) -> String {
        let temperature_desc = describe_temperature(&report.temperature);
        let weather_kind_desc = describe_weather_kind(&report.kind);
        let clouds_desc = describe_cloud_coverage(&report.cloud_coverage);
        let humidity_desc = describe_humidity(&report.humidity);
        let wind_desc = describe_wind(&report.wind);
        let pressure_desc = describe_pressure(&report.pressure);

        #[allow(clippy::uninlined_format_args)]
        {
            format!(
                "{} and {} with {}.\n{} with {}.\n{}.",
                temperature_desc,
                weather_kind_desc,
                clouds_desc,
                humidity_desc,
                wind_desc,
                pressure_desc,
            )
        }
    }
}

fn describe_weather_kind(kind: &Kind) -> String {
    let desc = prepare_kind_description(kind);
    match desc {
        KindDescription::Clouds { sky_adjective } => format!("the sky is {sky_adjective}"),
        KindDescription::Fog { description } => format!("{description} is covering the area"),
        KindDescription::Precipitation { description } => format!("{description} is falling"),
        KindDescription::Thunderstorm { description } => format!("{description} is raging"),
    }
}

fn describe_temperature(temperature: &Temperature) -> String {
    let adjective = describe_temperature_adjective(temperature);
    format!("It's {adjective} at {temperature:.1}")
}

fn describe_cloud_coverage(coverage: &Percentage) -> String {
    if coverage.value == 0 {
        "no clouds".into()
    } else {
        format!("clouds covering {coverage} of the sky")
    }
}

fn describe_humidity(percentage: &Percentage) -> String {
    let make_without_humidity = |adjective| format!("The air is {adjective} at {percentage}");
    let make_with_humidity = |adjective| format!("{} humidity", make_without_humidity(adjective));
    if percentage.value <= 15 {
        make_with_humidity("very dry")
    } else if percentage.value <= 30 {
        make_with_humidity("dry")
    } else if percentage.value <= 60 {
        make_without_humidity("humid")
    } else if percentage.value <= 85 {
        make_without_humidity("very humid")
    } else {
        make_with_humidity("heavy")
    }
}

fn describe_wind(wind: &Wind) -> String {
    enum SpeedLevel {
        NoWind,
        GentleBreeze,
        NormalWind,
        StrongWind,
        VeryStrongWind,
    }
    let speed_level = match wind.speed {
        Speed::MetersPerSecond(MetersPerSecond { value }) => {
            if value <= 0.2 {
                SpeedLevel::NoWind
            } else if value <= 3.3 {
                SpeedLevel::GentleBreeze
            } else if value <= 8.0 {
                SpeedLevel::NormalWind
            } else if value <= 13.8 {
                SpeedLevel::StrongWind
            } else {
                SpeedLevel::VeryStrongWind
            }
        }
    };
    let direction_definition = wind.direction.to_cardinal_direction().to_name();
    let adjective = match speed_level {
        SpeedLevel::NoWind => return "no wind".into(),
        SpeedLevel::GentleBreeze => format!("gentle {direction_definition} breeze"),
        SpeedLevel::NormalWind => format!("{direction_definition} wind"),
        SpeedLevel::StrongWind => format!("strong {direction_definition} wind"),
        SpeedLevel::VeryStrongWind => format!("very strong {direction_definition} wind"),
    };
    format!("{adjective} blowing at {:.1}", wind.speed)
}

fn describe_pressure(pressure: &Hectopascal) -> String {
    let adjective = if pressure.value <= 1000.0 {
        "Very low"
    } else if pressure.value <= 1010.0 {
        "Low"
    } else if pressure.value <= 1020.0 {
        "Normal"
    } else if pressure.value <= 1030.0 {
        "High"
    } else {
        "Very high"
    };
    format!("{adjective} pressure stands at {pressure:.1}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::port::mocks::MockWeatherProvider;
    use crate::types::units::*;
    use mockall::predicate::eq;

    #[test]
    fn fetches_current_full_report_with_provider() {
        let coordinates = Coordinates::new(1.23, 45.67);
        let report = CurrentFullReport {
            kind: Kind::Clouds(Clouds::Light),
            temperature: Temperature::new_celsius(24.7),
            cloud_coverage: Percentage::from(47),
            humidity: Percentage::from(60),
            wind: Wind {
                speed: Speed::new_meters_per_second(2.35),
                direction: Azimuth::from(225.3),
            },
            pressure: Hectopascal::from(1001.5),
        };

        let mut weather_provider = MockWeatherProvider::new();
        weather_provider
            .expect_fetch_current_full_report()
            .with(eq(coordinates))
            .times(1)
            .return_const(report);

        let sut = CurrentSummary::new(weather_provider);
        let _report = sut.fetch(&coordinates);
    }

    #[test]
    fn describes_values_of_clouds_kind() {
        let string = describe_weather_kind(&Kind::Clouds(Clouds::Moderate));
        assert_eq!(string, "the sky is cloudy");
    }

    #[test]
    fn describes_values_of_fog() {
        let string = describe_weather_kind(&Kind::Fog(Fog::Rime));
        assert_eq!(string, "rime fog is covering the area");
    }

    #[test]
    fn describes_values_of_precipitation() {
        let precipitation = Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        };
        let string = describe_weather_kind(&Kind::Precipitation(precipitation));
        assert_eq!(string, "light rain is falling");
    }

    #[test]
    fn describes_thunderstorm() {
        let string = describe_weather_kind(&Kind::Thunderstorm);
        assert_eq!(&string, "thunderstorm is raging");
    }

    #[test]
    fn describes_values_of_temperature_in_celsius() {
        let string = describe_temperature(&Temperature::new_celsius(24.5));
        assert_eq!(string, "It's hot at 24.5°C");
    }

    #[test]
    fn describes_values_of_cloud_coverage() {
        assert_eq!(describe_cloud_coverage(&Percentage::from(0)), "no clouds");
        assert_eq!(
            describe_cloud_coverage(&Percentage::from(27)),
            "clouds covering 27% of the sky"
        );
    }

    #[test]
    fn describes_values_of_humidity() {
        let describe = |value| describe_humidity(&Percentage::from(value));

        assert_eq!(describe(0), "The air is very dry at 0% humidity");
        assert_eq!(describe(15), "The air is very dry at 15% humidity");

        assert_eq!(describe(16), "The air is dry at 16% humidity");
        assert_eq!(describe(30), "The air is dry at 30% humidity");

        assert_eq!(describe(31), "The air is humid at 31%");
        assert_eq!(describe(60), "The air is humid at 60%");

        assert_eq!(describe(61), "The air is very humid at 61%");
        assert_eq!(describe(85), "The air is very humid at 85%");

        assert_eq!(describe(86), "The air is heavy at 86% humidity");
        assert_eq!(describe(100), "The air is heavy at 100% humidity");
    }

    #[test]
    fn describes_values_of_wind_speed_in_meters_per_second() {
        let describe = |value| {
            let wind = Wind {
                speed: Speed::new_meters_per_second(value),
                direction: Azimuth::from(42.0),
            };
            describe_wind(&wind)
        };

        assert_eq!(describe(0.0), "no wind");
        assert_eq!(describe(0.2), "no wind");

        assert_eq!(describe(0.21), "gentle northeast breeze blowing at 0.2 m/s");
        assert_eq!(describe(2.9), "gentle northeast breeze blowing at 2.9 m/s");
        assert_eq!(describe(3.3), "gentle northeast breeze blowing at 3.3 m/s");

        assert_eq!(describe(3.31), "northeast wind blowing at 3.3 m/s");
        assert_eq!(describe(5.57), "northeast wind blowing at 5.6 m/s");
        assert_eq!(describe(8.0), "northeast wind blowing at 8.0 m/s");

        assert_eq!(describe(8.01), "strong northeast wind blowing at 8.0 m/s");
        assert_eq!(describe(10.3), "strong northeast wind blowing at 10.3 m/s");
        assert_eq!(describe(13.8), "strong northeast wind blowing at 13.8 m/s");

        assert_eq!(
            describe(13.81),
            "very strong northeast wind blowing at 13.8 m/s"
        );
        assert_eq!(
            describe(15.0),
            "very strong northeast wind blowing at 15.0 m/s"
        );
    }

    #[test]
    fn describes_values_of_pressure() {
        let describe = |value| describe_pressure(&Hectopascal::from(value));

        assert_eq!(describe(995.0), "Very low pressure stands at 995.0 hPa");
        assert_eq!(describe(1000.0), "Very low pressure stands at 1000.0 hPa");

        assert_eq!(describe(1000.1), "Low pressure stands at 1000.1 hPa");
        assert_eq!(describe(1005.3), "Low pressure stands at 1005.3 hPa");
        assert_eq!(describe(1010.0), "Low pressure stands at 1010.0 hPa");

        assert_eq!(describe(1010.1), "Normal pressure stands at 1010.1 hPa");
        assert_eq!(describe(1018.7), "Normal pressure stands at 1018.7 hPa");
        assert_eq!(describe(1020.0), "Normal pressure stands at 1020.0 hPa");

        assert_eq!(describe(1020.1), "High pressure stands at 1020.1 hPa");
        assert_eq!(describe(1026.1), "High pressure stands at 1026.1 hPa");
        assert_eq!(describe(1030.0), "High pressure stands at 1030.0 hPa");

        assert_eq!(describe(1030.1), "Very high pressure stands at 1030.1 hPa");
        assert_eq!(describe(1035.0), "Very high pressure stands at 1035.0 hPa");
    }

    #[test]
    fn describes_full_report() {
        let report = CurrentFullReport {
            kind: Kind::Clouds(Clouds::Light),
            temperature: Temperature::new_celsius(22.4),
            cloud_coverage: Percentage::from(43),
            humidity: Percentage::from(81),
            wind: Wind {
                speed: Speed::new_meters_per_second(1.12),
                direction: Azimuth::from(140.3),
            },
            pressure: Hectopascal::from(1009.3),
        };

        let sut = CurrentSummary::new(MockWeatherProvider::new());
        let result = sut.format(&report);
        let expected: String = "It's warm at 22.4°C \
             and the sky is mostly clear \
             with clouds covering 43% of the sky.\n\
             The air is very humid at 81% \
             with gentle southeast breeze blowing at 1.1 m/s.\n\
             Low pressure stands at 1009.3 hPa."
            .into();
        assert_eq!(result, expected);
    }
}
