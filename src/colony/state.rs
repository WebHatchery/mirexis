//! Colony state production, construction, and repair behavior.

use super::*;
use crate::data::StartingResources;

impl Default for ColonyState {
    fn default() -> Self {
        Self::new()
    }
}

impl ColonyState {
    pub fn new() -> Self {
        Self::new_with_resources(&StartingResources {
            materials: 120,
            power: 4,
            food: 24,
            biomass: 12,
            alien_components: 0,
        })
    }

    pub fn new_with_resources(starting: &StartingResources) -> Self {
        Self {
            resources: Resources {
                materials: starting.materials,
                power: starting.power,
                food: starting.food,
                biomass: starting.biomass,
                alien_components: starting.alien_components,
            },
            buildings: vec![
                building(
                    "command_centre",
                    BuildingKind::CommandCentre,
                    SETTLEMENT_CENTER,
                ),
                building("barracks", BuildingKind::Barracks, [6, 8]),
                building("infirmary", BuildingKind::Infirmary, [10, 6]),
                building("workshop", BuildingKind::Workshop, [14, 8]),
                building("hydroponics", BuildingKind::Hydroponics, [7, 13]),
                building("power_plant", BuildingKind::PowerPlant, [13, 13]),
            ],
            construction_queue: Vec::new(),
            facility_upgrade_queue: Vec::new(),
            facility_upgrades: Vec::new(),
            planned_construction: BuildingKind::Barricade,
            next_building_serial: 1,
        }
    }

    pub fn has_facility(&self, kind: BuildingKind) -> bool {
        self.buildings.iter().any(|building| {
            building.kind == kind && !building.damaged && self.building_is_powered(&building.id)
        })
    }

    pub fn migrate_legacy_spatial_layout(&mut self) -> bool {
        let legacy = self
            .buildings
            .iter()
            .any(|building| building.id == "command_centre" && building.position == [3, 2]);
        if !legacy {
            return false;
        }

        let mut occupied = std::collections::HashSet::new();
        for building in &mut self.buildings {
            if let Some(position) = founding_building_position(&building.id) {
                building.position = position;
                reserve_anchor(&mut occupied, position);
            }
        }
        for building in &mut self.buildings {
            if founding_building_position(&building.id).is_some() {
                continue;
            }
            building.position = migrated_open_plot(building.position, &occupied);
            reserve_anchor(&mut occupied, building.position);
        }
        for project in &mut self.construction_queue {
            project.position = migrated_open_plot(project.position, &occupied);
            reserve_anchor(&mut occupied, project.position);
        }
        true
    }

    pub fn power_supply(&self) -> i32 {
        let upgrades = &self.facility_upgrades;
        self.resources.power
            + self
                .buildings
                .iter()
                .filter_map(|building| {
                    if !building.damaged {
                        let hot_core = upgrades.iter().any(|upgrade| {
                            upgrade.building_id == building.id
                                && upgrade.upgrade_id == HOT_CORE_UPGRADE
                        });
                        Some(building.kind.power_output() + i32::from(hot_core) * 3)
                    } else if building.kind == BuildingKind::PowerPlant
                        && upgrades.iter().any(|upgrade| {
                            upgrade.building_id == building.id
                                && upgrade.upgrade_id == REDUNDANT_GRID_UPGRADE
                        })
                    {
                        Some(2)
                    } else {
                        None
                    }
                })
                .sum::<i32>()
    }

    pub fn power_demand(&self) -> i32 {
        self.buildings
            .iter()
            .filter(|building| !building.damaged)
            .map(|building| building.kind.power_demand())
            .sum()
    }

    pub fn building_is_powered(&self, building_id: &str) -> bool {
        let mut remaining = self.power_supply();
        for building in self.buildings.iter().filter(|building| !building.damaged) {
            let demand = building.kind.power_demand();
            let powered = demand <= remaining;
            if powered {
                remaining -= demand;
            }
            if building.id == building_id {
                return powered;
            }
        }
        false
    }

    pub fn select_construction(&mut self, kind: BuildingKind) -> Result<(), String> {
        if !matches!(
            kind,
            BuildingKind::Barricade
                | BuildingKind::PowerPlant
                | BuildingKind::GeneLab
                | BuildingKind::ResearchAnnex
                | BuildingKind::SalvageYard
                | BuildingKind::Waystation
                | BuildingKind::Commons
                | BuildingKind::RelayMast
                | BuildingKind::Watchtower
        ) {
            return Err(format!("{} cannot be planned here", kind.name()));
        }
        self.planned_construction = kind;
        Ok(())
    }

    pub fn can_start_construction(&self, kind: BuildingKind) -> bool {
        (!construction::is_unique_kind(kind)
            || (!self.buildings.iter().any(|building| building.kind == kind)
                && !self
                    .construction_queue
                    .iter()
                    .any(|project| project.kind == kind)))
            && self.resources.materials >= kind.material_cost()
    }

    pub fn place_construction(
        &mut self,
        kind: BuildingKind,
        position: [i32; 2],
    ) -> Result<String, String> {
        self.validate_construction_site(position)?;
        if construction::is_unique_kind(kind)
            && (self.buildings.iter().any(|building| building.kind == kind)
                || self
                    .construction_queue
                    .iter()
                    .any(|project| project.kind == kind))
        {
            return Err(format!("The colony can support only one {}", kind.name()));
        }
        let cost = kind.material_cost();
        if self.resources.materials < cost {
            return Err(format!("Construction requires {} materials", cost));
        }
        self.resources.materials -= cost;
        let id = format!("{:?}_{}", kind, self.next_building_serial).to_lowercase();
        self.next_building_serial += 1;
        self.construction_queue.push(ConstructionProject {
            id: id.clone(),
            kind,
            position,
            operations_remaining: 1,
        });
        Ok(id)
    }

    pub fn advance_operation(&mut self) {
        for project in &mut self.construction_queue {
            project.operations_remaining = project.operations_remaining.saturating_sub(1);
        }
        let completed = self
            .construction_queue
            .iter()
            .filter(|project| project.operations_remaining == 0)
            .cloned()
            .collect::<Vec<_>>();
        self.construction_queue
            .retain(|project| project.operations_remaining > 0);
        self.buildings
            .extend(completed.into_iter().map(|project| BuildingState {
                id: project.id,
                kind: project.kind,
                position: project.position,
                level: 1,
                damaged: false,
            }));
        for project in &mut self.facility_upgrade_queue {
            project.operations_remaining = project.operations_remaining.saturating_sub(1);
        }
        let completed_upgrades = self
            .facility_upgrade_queue
            .iter()
            .filter(|project| project.operations_remaining == 0)
            .cloned()
            .collect::<Vec<_>>();
        self.facility_upgrade_queue
            .retain(|project| project.operations_remaining > 0);
        for project in completed_upgrades {
            if let Some(building) = self
                .buildings
                .iter_mut()
                .find(|building| building.id == project.building_id)
            {
                building.level = 2;
                self.facility_upgrades.push(FacilityUpgradeState {
                    building_id: project.building_id,
                    upgrade_id: project.upgrade_id,
                });
            }
        }
        let hydroponics_online = self.has_facility(BuildingKind::Hydroponics);
        self.resources.food += if hydroponics_online {
            3 + i32::from(
                self.has_active_upgrade(BuildingKind::Hydroponics, COMMUNITY_KITCHEN_UPGRADE),
            ) * 2
        } else if self.has_facility(BuildingKind::CommandCentre) {
            1
        } else {
            0
        };
        if hydroponics_online
            && self.has_active_upgrade(BuildingKind::Hydroponics, CULTURE_BEDS_UPGRADE)
        {
            self.resources.biomass += 2;
        }
        if self.has_facility(BuildingKind::ChoirGarden) {
            self.resources.biomass += 1;
        }
    }

    pub fn has_upgrade(&self, building_id: &str, upgrade_id: &str) -> bool {
        self.facility_upgrades
            .iter()
            .any(|upgrade| upgrade.building_id == building_id && upgrade.upgrade_id == upgrade_id)
    }

    pub fn has_active_upgrade(&self, kind: BuildingKind, upgrade_id: &str) -> bool {
        self.buildings.iter().any(|building| {
            building.kind == kind
                && !building.damaged
                && self.has_upgrade(&building.id, upgrade_id)
                && self.building_is_powered(&building.id)
        })
    }

    pub fn queue_facility_upgrade(
        &mut self,
        building_id: &str,
        upgrade_id: &str,
    ) -> Result<String, String> {
        let (kind, level, damaged, id) = self
            .buildings
            .iter()
            .find(|building| building.id == building_id)
            .map(|building| {
                (
                    building.kind,
                    building.level,
                    building.damaged,
                    building.id.clone(),
                )
            })
            .ok_or_else(|| format!("Unknown colony building: {building_id}"))?;
        let option = kind
            .upgrade_options()
            .iter()
            .find(|option| option.id == upgrade_id)
            .ok_or_else(|| format!("{} has no {} upgrade", kind.name(), upgrade_id))?;
        if level >= 2 || self.has_upgrade(building_id, upgrade_id) {
            return Err(format!("{} is already upgraded", kind.name()));
        }
        if self
            .facility_upgrade_queue
            .iter()
            .any(|project| project.building_id == building_id)
        {
            return Err(format!(
                "{} already has an upgrade in progress",
                kind.name()
            ));
        }
        if damaged || !self.building_is_powered(building_id) {
            return Err(format!("An operational {} is required", kind.name()));
        }
        let cost = kind.upgrade_cost();
        if self.resources.materials < cost {
            return Err(format!("Upgrade requires {} materials", cost));
        }
        self.resources.materials -= cost;
        self.facility_upgrade_queue.push(FacilityUpgradeProject {
            building_id: id,
            upgrade_id: option.id.to_owned(),
            operations_remaining: 1,
        });
        Ok(format!("{} upgrade queued // {}", kind.name(), option.name))
    }

    pub fn damage_for_failed_defense(&mut self, seed: u64) -> Option<String> {
        let mut candidates = self
            .buildings
            .iter()
            .enumerate()
            .filter(|(_, building)| {
                !building.damaged
                    && !matches!(
                        building.kind,
                        BuildingKind::CommandCentre | BuildingKind::Barricade
                    )
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            candidates = self
                .buildings
                .iter()
                .enumerate()
                .filter(|(_, building)| !building.damaged)
                .map(|(index, _)| index)
                .collect();
        }
        let index = *candidates.get(seed as usize % candidates.len().max(1))?;
        let building = &mut self.buildings[index];
        building.damaged = true;
        Some(building.kind.name().to_owned())
    }

    pub fn repair_building(&mut self, building_id: &str) -> Result<RepairResult, String> {
        let (kind, damaged) = self
            .buildings
            .iter()
            .find(|building| building.id == building_id)
            .map(|building| (building.kind, building.damaged))
            .ok_or_else(|| format!("Unknown colony building: {}", building_id))?;
        if !damaged {
            return Err(format!("{} does not need repair", kind.name()));
        }
        let cost = self
            .repair_cost_for(building_id)
            .expect("damaged building has a repair cost");
        if self.resources.materials < cost {
            return Err(format!("Repair requires {} materials", cost));
        }
        let building = self
            .buildings
            .iter_mut()
            .find(|building| building.id == building_id)
            .expect("building was validated before repair mutation");
        self.resources.materials -= cost;
        building.damaged = false;
        Ok(RepairResult {
            building_name: kind.name().to_owned(),
            materials_spent: cost,
        })
    }
    pub fn is_occupied(&self, position: [i32; 2]) -> bool {
        self.buildings
            .iter()
            .any(|building| building.kind.occupies(building.position, position))
            || self
                .construction_queue
                .iter()
                .any(|project| project.kind.occupies(project.position, position))
    }

    pub fn validate_construction_site(&self, position: [i32; 2]) -> Result<(), String> {
        if !clearance_in_bounds(position) {
            return Err("Site needs one in-bounds clearance tile on every side".to_owned());
        }
        if self
            .buildings
            .iter()
            .any(|building| clearance_zones_overlap(position, building.position))
            || self
                .construction_queue
                .iter()
                .any(|project| clearance_zones_overlap(position, project.position))
        {
            return Err("Site's 3x3 clearance zone overlaps another structure".to_owned());
        }
        Ok(())
    }

    pub fn building_at(&self, position: [i32; 2]) -> Option<&BuildingState> {
        self.buildings
            .iter()
            .find(|building| building.kind.occupies(building.position, position))
    }

    pub fn project_at(&self, position: [i32; 2]) -> Option<&ConstructionProject> {
        self.construction_queue
            .iter()
            .find(|project| project.kind.occupies(project.position, position))
    }

    pub fn ensure_phase_one_infrastructure(&mut self, migrate_power: bool) {
        if !self
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::Hydroponics)
        {
            let position = self.first_open_plot([7, 13]);
            self.buildings
                .push(building("hydroponics", BuildingKind::Hydroponics, position));
        }
        if !self
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::PowerPlant)
        {
            let position = self.first_open_plot([13, 13]);
            self.buildings
                .push(building("power_plant", BuildingKind::PowerPlant, position));
            if migrate_power {
                self.resources.power = (self.resources.power - 4).max(0);
            }
        }
    }

    pub fn ensure_gene_lab(&mut self) {
        if self
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::GeneLab)
        {
            return;
        }
        let position = self.first_open_plot([10, 14]);
        self.buildings
            .push(building("gene_lab", BuildingKind::GeneLab, position));
    }

    pub fn first_open_plot(&self, preferred: [i32; 2]) -> [i32; 2] {
        let open = |position: [i32; 2]| self.validate_construction_site(position).is_ok();
        if open(preferred) {
            return preferred;
        }
        (0..COLONY_HEIGHT)
            .flat_map(|y| (0..COLONY_WIDTH).map(move |x| [x, y]))
            .find(|position| open(*position))
            .expect("the colony has room for required Phase One infrastructure")
    }
}
