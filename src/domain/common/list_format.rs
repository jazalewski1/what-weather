use crate::types::weather::*;

pub fn describe_kind(kind: &Kind) -> String {
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
