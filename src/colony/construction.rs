//! Shared construction-category rules used by placement and colony affordances.

use super::BuildingKind;

pub(super) fn is_unique_kind(kind: BuildingKind) -> bool {
    matches!(
        kind,
        BuildingKind::GeneLab
            | BuildingKind::ResearchAnnex
            | BuildingKind::SalvageYard
            | BuildingKind::Waystation
            | BuildingKind::Commons
            | BuildingKind::RelayMast
            | BuildingKind::Watchtower
    )
}
