use crate::port::weather::*;
use crate::types::attributes::*;
use crate::types::report::CurrentFullReport;
use crate::types::report::CurrentPartialReport;
use crate::types::units::*;
use crate::types::weather::*;

pub struct FakeWeatherProvider;

impl WeatherProvider for FakeWeatherProvider {
    fn fetch_current_full_report(&self, coordinates: &Coordinates) -> CurrentFullReport {
        CurrentFullReport {
            kind: generate_random_weather_kind(),
            temperature: generate_random_temperature(coordinates),
            cloud_coverage: generate_random_cloud_coverage(),
            humidity: generate_random_humidity(),
            wind: generate_random_wind(),
            pressure: generate_random_pressure(),
        }
    }

    fn fetch_current_partial_report(
        &self,
        coordinates: &Coordinates,
        attributes: &WeatherAttributeSet,
    ) -> CurrentPartialReport {
        let mut report = CurrentPartialReport::new_empty(*coordinates);
        for attribute in attributes.iter() {
            match attribute {
                WeatherAttribute::WeatherKind => {
                    report.kind.replace(generate_random_weather_kind());
                }
                WeatherAttribute::Temperature => {
                    report
                        .temperature
                        .replace(generate_random_temperature(coordinates));
                }
                WeatherAttribute::CloudCoverage => {
                    report
                        .cloud_coverage
                        .replace(generate_random_cloud_coverage());
                }
                WeatherAttribute::Humidity => {
                    report.humidity.replace(generate_random_humidity());
                }
                WeatherAttribute::Wind => {
                    report.wind.replace(generate_random_wind());
                }
                WeatherAttribute::Pressure => {
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

fn generate_random_temperature(coordinates: &Coordinates) -> Temperature {
    let normal = coordinates.latitude.value.abs() / 90.0;
    let min = (20.0 - (50.0 * normal)) as i64;
    let max = (40.0 - (35.0 * normal.powi(2).powf(1.5))) as i64;
    Temperature::new_celsius(rnd::generate_float(min..max, 1))
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
