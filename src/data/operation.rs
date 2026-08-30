//! Tactical effects applied by faction pressure and the chosen Mirexis path.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationModifier {
    #[default]
    None,
    DirectorateFireControl,
    BroodFrenzy,
    AscendantInterference,
    EscalationCrossfire,
    MirexisRedoubt,
    MirexisCommonwealth,
    MirexisThreshold,
}

impl OperationModifier {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "NO ESCALATION",
            Self::DirectorateFireControl => "DIRECTORATE FIRE-CONTROL",
            Self::BroodFrenzy => "BROOD FRENZY",
            Self::AscendantInterference => "ASCENDANT INTERFERENCE",
            Self::EscalationCrossfire => "THREE-POWER CROSSFIRE",
            Self::MirexisRedoubt => "REDOUBT SHELTER",
            Self::MirexisCommonwealth => "COMMONWEALTH RESONANCE",
            Self::MirexisThreshold => "THRESHOLD GUIDANCE",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::None => "No faction-pressure modifier is active.",
            Self::DirectorateFireControl => "Hostile attacks gain 10 accuracy.",
            Self::BroodFrenzy => "Hostile units gain 1 movement.",
            Self::AscendantInterference => "Colonist attacks lose 10 accuracy.",
            Self::EscalationCrossfire => {
                "Hostiles gain 5 accuracy and 1 movement; colonists lose 5 accuracy."
            }
            Self::MirexisRedoubt => "Colonists gain 2 armour from the refuge's hardened perimeter.",
            Self::MirexisCommonwealth => {
                "Colonists recover 1 additional health at the start of their phases."
            }
            Self::MirexisThreshold => {
                "Colonists gain 1 movement from an open route through Mirexis."
            }
        }
    }

    pub fn is_engine_effect(self) -> bool {
        matches!(
            self,
            Self::MirexisRedoubt | Self::MirexisCommonwealth | Self::MirexisThreshold
        )
    }
}
