use crate::collections::create_set;

pub use ascot::hazards::{Category, Hazard, HazardData, ALL_HAZARDS};

create_set!(Hazards, Hazard, hazard, hazards);

#[cfg(test)]
mod tests {
    use ascot::hazards::Hazard;
    use serde_json::json;

    use crate::serialize;

    use super::Hazards;

    #[test]
    fn test_one_hazard() {
        assert_eq!(
            serialize(Hazards::one(Hazard::AirPoisoning)),
            json!(["AirPoisoning"])
        );
    }

    #[test]
    fn test_two_hazards() {
        assert_eq!(
            serialize(Hazards::two((Hazard::AirPoisoning, Hazard::Asphyxia,))),
            json!(["AirPoisoning", "Asphyxia",])
        );
    }

    #[test]
    fn test_three_hazards() {
        assert_eq!(
            serialize(Hazards::three((
                Hazard::AirPoisoning,
                Hazard::Asphyxia,
                Hazard::AudioVideoDisplay,
            ))),
            json!(["AirPoisoning", "Asphyxia", "AudioVideoDisplay",])
        );
    }

    #[test]
    fn test_four_hazards() {
        assert_eq!(
            serialize(Hazards::four((
                Hazard::AirPoisoning,
                Hazard::Asphyxia,
                Hazard::AudioVideoDisplay,
                Hazard::AudioVideoRecordAndStore,
            ))),
            json!([
                "AirPoisoning",
                "Asphyxia",
                "AudioVideoDisplay",
                "AudioVideoRecordAndStore",
            ])
        );
    }

    #[test]
    fn test_five_hazards() {
        assert_eq!(
            serialize(Hazards::five((
                Hazard::AirPoisoning,
                Hazard::Asphyxia,
                Hazard::AudioVideoDisplay,
                Hazard::AudioVideoRecordAndStore,
                Hazard::ElectricEnergyConsumption,
            ))),
            json!([
                "AirPoisoning",
                "Asphyxia",
                "AudioVideoDisplay",
                "AudioVideoRecordAndStore",
                "ElectricEnergyConsumption",
            ])
        );
    }

    #[test]
    fn test_six_hazards() {
        assert_eq!(
            serialize(Hazards::six((
                Hazard::AirPoisoning,
                Hazard::Asphyxia,
                Hazard::AudioVideoDisplay,
                Hazard::AudioVideoRecordAndStore,
                Hazard::ElectricEnergyConsumption,
                Hazard::Explosion,
            ))),
            json!([
                "AirPoisoning",
                "Asphyxia",
                "AudioVideoDisplay",
                "AudioVideoRecordAndStore",
                "ElectricEnergyConsumption",
                "Explosion",
            ])
        );
    }

    #[test]
    fn test_seven_hazards() {
        assert_eq!(
            serialize(Hazards::seven((
                Hazard::AirPoisoning,
                Hazard::Asphyxia,
                Hazard::AudioVideoDisplay,
                Hazard::AudioVideoRecordAndStore,
                Hazard::ElectricEnergyConsumption,
                Hazard::Explosion,
                Hazard::FireHazard,
            ))),
            json!([
                "AirPoisoning",
                "Asphyxia",
                "AudioVideoDisplay",
                "AudioVideoRecordAndStore",
                "ElectricEnergyConsumption",
                "Explosion",
                "FireHazard",
            ])
        );
    }

    #[test]
    fn test_eight_hazards() {
        assert_eq!(
            serialize(Hazards::eight((
                Hazard::AirPoisoning,
                Hazard::Asphyxia,
                Hazard::AudioVideoDisplay,
                Hazard::AudioVideoRecordAndStore,
                Hazard::ElectricEnergyConsumption,
                Hazard::Explosion,
                Hazard::FireHazard,
                Hazard::GasConsumption
            ))),
            json!([
                "AirPoisoning",
                "Asphyxia",
                "AudioVideoDisplay",
                "AudioVideoRecordAndStore",
                "ElectricEnergyConsumption",
                "Explosion",
                "FireHazard",
                "GasConsumption"
            ])
        );
    }
}
