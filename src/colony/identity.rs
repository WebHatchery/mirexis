//! Path-specific identity buildings established when Mirexis chooses its future.

use super::{building, BuildingKind, ColonyState};

impl BuildingKind {
    pub fn is_identity(self) -> bool {
        matches!(
            self,
            Self::RedoubtArsenal | Self::ChoirGarden | Self::ThresholdSpire
        )
    }

    pub fn identity_for_path(path_id: &str) -> Option<Self> {
        match path_id {
            "human_redoubt" => Some(Self::RedoubtArsenal),
            "living_commonwealth" => Some(Self::ChoirGarden),
            "open_threshold" => Some(Self::ThresholdSpire),
            _ => None,
        }
    }
}

impl ColonyState {
    pub fn ensure_identity_building(&mut self, path_id: &str) -> Result<String, String> {
        let kind = BuildingKind::identity_for_path(path_id)
            .ok_or_else(|| format!("Unknown Mirexis identity path: {path_id}"))?;
        if let Some(existing) = self
            .buildings
            .iter()
            .find(|building| building.kind.is_identity())
        {
            if existing.kind != kind {
                return Err("The colony has already built its identity project".to_owned());
            }
            return Ok(existing.kind.name().to_owned());
        }
        let (id, preferred) = match kind {
            BuildingKind::RedoubtArsenal => ("redoubt_arsenal", [4, 4]),
            BuildingKind::ChoirGarden => ("choir_garden", [4, 15]),
            BuildingKind::ThresholdSpire => ("threshold_spire", [16, 4]),
            _ => unreachable!("identity path resolved to an identity building"),
        };
        let position = self.first_open_plot(preferred);
        self.buildings.push(building(id, kind, position));
        Ok(kind.name().to_owned())
    }
}
