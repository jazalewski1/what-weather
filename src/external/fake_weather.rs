use crate::port::weather::*;
use crate::types::attributes::*;
use crate::types::report::*;
use crate::types::units::*;
use crate::types::weather::*;

pub struct FakeWeatherProvider;

impl WeatherProvider for FakeWeatherProvider {
    fn fetch_current_full_report(&self, coordinates: &Coordinates) -> CurrentFullReport {
        CurrentFullReport {
            kind: generate_random_weather_kind(),
            temperature: generate_random_temperature(coordinates),
            cloud_coverage: generate_random_percentage(),
            humidity: generate_random_percentage(),
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
                    report.cloud_coverage.replace(generate_random_percentage());
                }
                WeatherAttribute::Humidity => {
                    report.humidity.replace(generate_random_percentage());
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

    fn fetch_forecast_full_report(&self, coordinates: &Coordinates) -> TodayForecastFullReport {
        TodayForecastFullReport {
            kind: generate_random_weather_kind(),
            temperature_range: generate_random_temperature_range(coordinates),
            cloud_coverage_range: generate_random_perecentage_range(),
            humidity_range: generate_random_perecentage_range(),
            wind: generate_random_wind_scope(),
            pressure_range: generate_random_pressure_range(),
        }
    }

    fn fetch_daily_forecast_full_report(
        &self,
        coordinates: &Coordinates,
        period: &Period,
    ) -> DailyForecastFullReport {
        let mut data = Vec::with_capacity(period.length as usize);
        for date in period.start.iter_days().take(period.length as usize) {
            let single_data = DailyFullData {
                date,
                kind: generate_random_weather_kind(),
                temperature_range: generate_random_temperature_range(coordinates),
                cloud_coverage_range: generate_random_perecentage_range(),
                humidity_range: generate_random_perecentage_range(),
                wind: generate_random_wind_scope(),
                pressure_range: generate_random_pressure_range(),
            };
            data.push(single_data);
        }
        DailyForecastFullReport { data }
    }

    fn fetch_today_forecast_partial_report(
        &self,
        coordinates: &Coordinates,
        attributes: &WeatherAttributeSet,
    ) -> TodayForecastPartialReport {
        let mut report = TodayForecastPartialReport {
            coordinates: *coordinates,
            kind: None,
            temperature_range: None,
            cloud_coverage_range: None,
            humidity_range: None,
            wind: None,
            pressure_range: None,
        };
        for attribute in attributes {
            match attribute {
                WeatherAttribute::WeatherKind => {
                    report.kind.replace(generate_random_weather_kind());
                }
                WeatherAttribute::Temperature => {
                    report
                        .temperature_range
                        .replace(generate_random_temperature_range(coordinates));
                }
                WeatherAttribute::CloudCoverage => {
                    report
                        .cloud_coverage_range
                        .replace(generate_random_perecentage_range());
                }
                WeatherAttribute::Humidity => {
                    report
                        .humidity_range
                        .replace(generate_random_perecentage_range());
                }
                WeatherAttribute::Wind => {
                    report.wind.replace(generate_random_wind_scope());
                }
                WeatherAttribute::Pressure => {
                    report
                        .pressure_range
                        .replace(generate_random_pressure_range());
                }
            }
        }
        report
    }

    fn fetch_daily_forecast_partial_report(
        &self,
        coordinates: &Coordinates,
        period: &Period,
        attributes: &WeatherAttributeSet,
    ) -> DailyForecastPartialReport {
        let mut report = DailyForecastPartialReport {
            coordinates: *coordinates,
            data: Vec::new(),
        };
        for date in period.start.iter_days().take(period.length as usize) {
            let mut day_data = DailyPartialData {
                date,
                kind: None,
                temperature_range: None,
                cloud_coverage_range: None,
                humidity_range: None,
                wind: None,
                pressure_range: None,
            };
            for attribute in attributes {
                match attribute {
                    WeatherAttribute::WeatherKind => {
                        day_data.kind.replace(generate_random_weather_kind());
                    }
                    WeatherAttribute::Temperature => {
                        day_data
                            .temperature_range
                            .replace(generate_random_temperature_range(coordinates));
                    }
                    WeatherAttribute::CloudCoverage => {
                        day_data
                            .cloud_coverage_range
                            .replace(generate_random_perecentage_range());
                    }
                    WeatherAttribute::Humidity => {
                        day_data
                            .humidity_range
                            .replace(generate_random_perecentage_range());
                    }
                    WeatherAttribute::Wind => {
                        day_data.wind.replace(generate_random_wind_scope());
                    }
                    WeatherAttribute::Pressure => {
                        day_data
                            .pressure_range
                            .replace(generate_random_pressure_range());
                    }
                }
            }
            report.data.push(day_data);
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

fn generate_random_celsius(coordinates: &Coordinates) -> Celsius {
    let normal = coordinates.latitude.value.abs() / 90.0;
    let lower = (20.0 - (50.0 * normal)) as i64;
    let upper = (40.0 - (35.0 * normal.powi(2).powf(1.4))) as i64;
    Celsius::from(rnd::generate_float(lower..upper, 1))
}

fn generate_random_temperature(coordinates: &Coordinates) -> Temperature {
    Temperature::Celsius(generate_random_celsius(coordinates))
}

fn generate_random_temperature_range(coordinates: &Coordinates) -> TemperatureRange {
    let diff = rnd::generate_float(2..6, 1);
    let base = generate_random_celsius(coordinates);
    let min = Celsius::from(base.value - diff);
    let max = Celsius::from(base.value + diff);
    TemperatureRange::Celsius { min, max }
}

fn generate_random_percentage() -> Percentage {
    Percentage::from(rnd::generate_integer(0..101) as i8)
}

fn generate_random_perecentage_range() -> PercentageRange {
    let max = rnd::generate_integer(0..101);
    let min = rnd::generate_integer(0..max);
    PercentageRange::new(min as i8, max as i8)
}

fn generate_random_wind_direction() -> Azimuth {
    Azimuth::from(rnd::generate_float(0..360, 1))
}

fn generate_random_wind() -> Wind {
    Wind {
        speed: Speed::new_meters_per_second(rnd::generate_float(0..16, 2)),
        direction: generate_random_wind_direction(),
    }
}

fn generate_random_wind_scope() -> WindScope {
    let speed_range = {
        let max = rnd::generate_float(0..16, 2);
        let min = (max - rnd::generate_float(0..4, 2)).clamp(0.0, max);
        SpeedRange::new_meters_per_second(min, max)
    };
    WindScope {
        speed_range,
        dominant_direction: generate_random_wind_direction(),
    }
}

fn generate_random_pressure() -> Hectopascal {
    Hectopascal::from(rnd::generate_float(990..1040, 1))
}

fn generate_random_pressure_range() -> PressureRange {
    let min = rnd::generate_float(990..1000, 1);
    let max = rnd::generate_float(1000..1040, 1);
    PressureRange::new(min, max)
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
