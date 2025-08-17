use crate::port::WeatherProvider;
use crate::types::query::*;
use crate::types::report::*;
use crate::types::units::*;
use crate::types::weather::*;

pub struct FakeWeatherProvider;

impl WeatherProvider for FakeWeatherProvider {
    fn fetch_all(&self, _query: &FullQuery) -> FullReport {
        FullReport {
            kind: generate_random_weather_kind(),
            temperature: generate_random_temperature(),
            cloud_coverage: generate_random_cloud_coverage(),
            humidity: generate_random_humidity(),
            wind: generate_random_wind(),
            pressure: generate_random_pressure(),
        }
    }

    fn fetch_selected(&self, query: &PartialQuery) -> PartialReport {
        let mut report = PartialReport::default();
        for param in query.parameters.iter() {
            match param {
                WeatherParameter::WeatherKind => {
                    report.kind.replace(generate_random_weather_kind());
                }
                WeatherParameter::Temperature => {
                    report.temperature.replace(generate_random_temperature());
                }
                WeatherParameter::CloudCoverage => {
                    report
                        .cloud_coverage
                        .replace(generate_random_cloud_coverage());
                }
                WeatherParameter::Humidity => {
                    report.humidity.replace(generate_random_humidity());
                }
                WeatherParameter::Wind => {
                    report.wind.replace(generate_random_wind());
                }
                WeatherParameter::Pressure => {
                    report.pressure.replace(generate_random_pressure());
                }
            }
        }
        report
    }
}

fn generate_random_weather_kind() -> Kind {
    let weather_kinds = [
        Kind::Clouds(Clouds::Clear),
        Kind::Clouds(Clouds::Light),
        Kind::Clouds(Clouds::Dense),
        Kind::Fog(Fog::Normal),
        Kind::Precipitation(Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        }),
        Kind::Precipitation(Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Freezing,
        }),
        Kind::Precipitation(Precipitation {
            kind: PrecipitationKind::Snow,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        }),
        Kind::Precipitation(Precipitation {
            kind: PrecipitationKind::Snow,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Normal,
        }),
        Kind::Thunderstorm,
    ];
    let weather_kind_index = rnd::generate_integer(0..weather_kinds.len() as i64) as usize;
    weather_kinds[weather_kind_index]
}

fn generate_random_temperature() -> Temperature {
    Temperature::new_celsius(rnd::generate_float(-10..40, 1))
}

fn generate_random_cloud_coverage() -> Percentage {
    Percentage::from(rnd::generate_integer(0..101) as i8)
}

fn generate_random_humidity() -> Percentage {
    Percentage::from(rnd::generate_integer(0..101) as i8)
}

fn generate_random_wind() -> Wind {
    Wind {
        speed: Speed::new_meters_per_second(rnd::generate_float(0..16, 2)),
        direction: Azimuth::from(rnd::generate_float(0..360, 1)),
    }
}

fn generate_random_pressure() -> Hectopascal {
    Hectopascal::from(rnd::generate_float(990..1040, 1))
}

/// This module uses obviously naive random generators, but is good enough for fake data,
/// and will be removed in the future anyway.
mod rnd {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn generate_integer(range: std::ops::Range<i64>) -> i64 {
        let span = (range.end - range.start) as u128;
        let random_base = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get current time")
            .as_nanos();
        (random_base % span) as i64 + range.start
    }

    pub fn generate_float(range: std::ops::Range<i64>, precision: u8) -> f32 {
        let random_integer = {
            let multiplier = 10_i64.pow(precision.into());
            let range = (range.start * multiplier)..(range.end * multiplier);
            generate_integer(range)
        };
        let divider = 10_f32.powi(precision.into());
        random_integer as f32 / divider
    }
}
