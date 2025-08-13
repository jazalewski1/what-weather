use crate::domain::port::WeatherProvider;
use crate::domain::types::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct FakeWeatherProvider;

impl WeatherProvider for FakeWeatherProvider {
    fn fetch(&self, query: &WeatherQuery) -> WeatherReport {
        WeatherReport {
            coordinates: query.coordinates,
            kind: generate_random_weather_kind(),
            temperature: generate_random_temperature(),
            cloud_coverage: generate_random_cloud_coverage(),
            humidity: generate_random_humidity(),
            wind: generate_random_wind(),
        }
    }
}

fn generate_random_number(range: std::ops::Range<usize>) -> usize {
    let count = (range.end - range.start) as u128;
    let milliseconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get current time")
        .as_millis();
    (milliseconds % count) as usize + range.start
}

fn generate_random_weather_kind() -> WeatherKind {
    let weather_kinds = [
        WeatherKind::Clouds(Clouds::Clear),
        WeatherKind::Clouds(Clouds::Light),
        WeatherKind::Clouds(Clouds::Dense),
        WeatherKind::Fog(Fog::Normal),
        WeatherKind::Precipitation(Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Moderate,
            heat: PrecipitationHeat::Normal,
        }),
        WeatherKind::Precipitation(Precipitation {
            kind: PrecipitationKind::Rain,
            intensity: PrecipitationIntensity::Heavy,
            heat: PrecipitationHeat::Freezing,
        }),
        WeatherKind::Precipitation(Precipitation {
            kind: PrecipitationKind::Snow,
            intensity: PrecipitationIntensity::Light,
            heat: PrecipitationHeat::Normal,
        }),
        WeatherKind::Precipitation(Precipitation {
            kind: PrecipitationKind::Snow,
            intensity: PrecipitationIntensity::Shower,
            heat: PrecipitationHeat::Normal,
        }),
        WeatherKind::Thunderstorm,
    ];
    let weather_kind_index = generate_random_number(0..weather_kinds.len());
    weather_kinds[weather_kind_index]
}

fn generate_random_temperature() -> Temperature {
    let fractional = (generate_random_number(0..10) as f32) / 10.0;
    let integral = (generate_random_number(0..40) as f32) - 10.0;
    integral + fractional + 1.3
}

fn generate_random_cloud_coverage() -> CloudCoverage {
    generate_random_number(0..101) as i8
}

fn generate_random_humidity() -> i8 {
    generate_random_number(0..101) as i8
}

fn generate_random_wind() -> Wind {
    let speed = {
        let fractional = (generate_random_number(0..100) as f32) / 100.0;
        let integral = generate_random_number(0..15) as f32;
        integral + fractional
    };
    let direction = {
        let fractional = (generate_random_number(0..10) as f32) / 10.0;
        let integral = generate_random_number(0..360) as f32;
        integral + fractional
    };
    Wind { speed, direction }
}
