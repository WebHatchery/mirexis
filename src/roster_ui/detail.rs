//! Selected-character roster detail rendering.

use super::*;

pub fn draw_selected_character(
    campaign: &CampaignState,
    data: &GameData,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    mouse: Vec2,
    roster_inspection_id: Option<&str>,
    actions: &mut Vec<UiAction>,
) {
    let panel = Rect::new(316.0, 96.0, 946.0, 580.0);
    draw_surface_with_title(
        panel,
        Some("TRAINING // WORKSHOP"),
        &SurfaceStyle::new(Color::new(0.055, 0.07, 0.08, 0.98))
            .with_border(1.0, Color::new(0.25, 0.38, 0.40, 0.8))
            .with_header(42.0, Color::new(0.08, 0.105, 0.115, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let Some(character) = campaign.selected_character() else {
        return;
    };
    draw_character_profile(data, assets, visuals, character);
    draw_character_history(campaign, character);
    let inspected_class = draw_training_controls(campaign, data, character, mouse, actions);
    let active_class = data
        .classes
        .iter()
        .find(|class| class.id == character.active_class);
    let inspected_technique = draw_techniques(campaign, character, active_class, mouse, actions);
    let hovered_equipment = data
        .equipment
        .iter()
        .enumerate()
        .find_map(|(index, item)| equipment_row_rect(index).contains(mouse).then_some(item));
    let inspected_equipment = hovered_equipment.or_else(|| {
        roster_inspection_id.and_then(|id| data.equipment.iter().find(|item| item.id == id))
    });
    if let Some(item) = inspected_equipment {
        draw_text_block(
            &format!(
                "{} // {} // TOUCH INFO (?) KEEPS THIS READOUT OPEN",
                item.name.to_uppercase(),
                item.description
            ),
            344.0,
            516.0,
            870.0,
            28.0,
            11.0,
            2.0,
            dark::TEXT_DIM,
        );
    } else if let Some(technique) = inspected_technique {
        draw_text_block(
            &format!(
                "{} // {} // REQUIRES {} XP",
                technique.name.to_uppercase(),
                technique.description,
                campaign.skill_experience_required(technique.experience_required)
            ),
            344.0,
            516.0,
            870.0,
            20.0,
            11.0,
            2.0,
            dark::TEXT_DIM,
        );
    } else if let Some(class) = inspected_class {
        draw_text_block(
            &format!("{} // {}", class.name.to_uppercase(), class.description),
            344.0,
            516.0,
            870.0,
            28.0,
            11.0,
            2.0,
            dark::TEXT_DIM,
        );
    }
    draw_equipment_controls(EquipmentControlsContext {
        campaign,
        character,
        data,
        assets,
        visuals,
        mouse,
        roster_inspection_id,
        actions,
    });
}

fn draw_character_profile(
    data: &GameData,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    character: &crate::campaign::CharacterRecord,
) {
    let mutation = data
        .mutations
        .iter()
        .find(|mutation| mutation.id == character.mutation_id)
        .map_or(character.mutation_id.as_str(), |mutation| {
            mutation.name.as_str()
        });
    let evolution = data
        .mutations
        .iter()
        .find(|entry| entry.id == character.mutation_id)
        .and_then(|mutation| {
            mutation
                .evolutions
                .iter()
                .find(|evolution| evolution.id == character.mutation_evolution_id)
        });
    let mutation = evolution.map_or_else(
        || mutation.to_owned(),
        |evolution| format!("{} / {}", mutation, evolution.name),
    );
    crate::portrait_ui::draw_character_portrait(
        assets,
        visuals,
        Rect::new(344.0, 150.0, 120.0, 110.0),
        &character.id,
        &character.name,
        Color::new(0.28, 0.88, 0.72, 1.0),
    );
    draw_ui_text_ex(
        &character.name,
        484.0,
        172.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_block(
        &character.biography,
        484.0,
        190.0,
        730.0,
        52.0,
        17.0,
        4.0,
        dark::TEXT_DIM,
    );
    draw_ui_text_ex(
        &format!(
            "LEVEL {} · {} XP · {} · {:?} · ORIGIN {}",
            character.level,
            character.experience,
            mutation,
            character.availability,
            if character.origin.is_empty() {
                "COLONY FOUNDER"
            } else {
                character.origin.as_str()
            }
        ),
        344.0,
        276.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    let equipment = character
        .equipment_ids
        .iter()
        .filter_map(|id| data.equipment.iter().find(|item| &item.id == id))
        .map(|item| item.name.as_str())
        .collect::<Vec<_>>()
        .join(" · ");
    draw_ui_text_ex(
        &format!("LOADOUT  {}", equipment),
        344.0,
        298.0,
        TextStyle::new(14.0, dark::TEXT_DIM).params(),
    );
}

fn draw_character_history(campaign: &CampaignState, character: &crate::campaign::CharacterRecord) {
    let mut history = Vec::new();
    if !character.event_legacies.is_empty() {
        history.push(
            character
                .event_legacies
                .iter()
                .map(|legacy| format!("{} ({:+} {})", legacy.name, legacy.amount, legacy.stat))
                .collect::<Vec<_>>()
                .join(" · "),
        );
    }
    if !character.traumas.is_empty() {
        let trauma_ward_active = campaign
            .colony
            .has_active_upgrade(BuildingKind::Infirmary, TRAUMA_WARD_UPGRADE);
        history.push(
            character
                .traumas
                .iter()
                .map(|trauma| {
                    format!(
                        "{} ({})",
                        trauma.name,
                        crate::trauma::effective_effect(trauma, trauma_ward_active)
                    )
                })
                .collect::<Vec<_>>()
                .join(" · "),
        );
    }
    if !history.is_empty() {
        draw_ui_text_ex(
            &format!("HISTORY  {}", history.join("  //  ")),
            344.0,
            306.0,
            TextStyle::new(11.0, dark::ACCENT).params(),
        );
    }
    draw_ui_text_ex(
        "BARRACKS // CLASS TRAINING · ADVANCED DISCIPLINES REQUIRE LV3 + ADAPTATION",
        344.0,
        322.0,
        TextStyle::new(13.0, dark::ACCENT).params(),
    );
}

fn draw_training_controls<'a>(
    campaign: &CampaignState,
    data: &'a GameData,
    character: &crate::campaign::CharacterRecord,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) -> Option<&'a crate::data::ClassDef> {
    let mut inspected_class = data
        .classes
        .iter()
        .find(|class| class.id == character.active_class);
    for (index, class) in data.classes.iter().enumerate() {
        let cost = campaign.training_cost(&character.id, class).unwrap_or(100);
        let lock = campaign.class_training_lock_reason(&character.id, class);
        let has_barracks = campaign.colony.has_facility(BuildingKind::Barracks);
        let affordable = campaign.colony.resources.materials >= cost as i32;
        let enabled =
            has_barracks && character.active_class != class.id && lock.is_none() && affordable;
        let label = class_training_label(
            character.active_class == class.id,
            lock.as_deref(),
            has_barracks,
            affordable,
            &class.name,
            cost,
        );
        let column = index % 3;
        let row = index / 3;
        let rect = Rect::new(
            344.0 + column as f32 * 296.0,
            334.0 + row as f32 * 36.0,
            282.0,
            30.0,
        );
        if rect.contains(mouse) {
            inspected_class = Some(class);
        }
        if button(rect, &label, enabled, mouse) {
            actions.push(UiAction::TrainSelected(class.id.clone()));
        }
    }
    inspected_class
}

fn draw_techniques<'a>(
    campaign: &CampaignState,
    character: &crate::campaign::CharacterRecord,
    active_class: Option<&'a crate::data::ClassDef>,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) -> Option<&'a crate::data::TechniqueDef> {
    let mut inspected_technique = None;
    draw_ui_text_ex(
        &format!(
            "TECHNIQUES // {} SLOT{} · OPERATION XP UNLOCKS NEW TECHNIQUES{}",
            active_class.map_or(0, |class| class.technique_slots),
            if active_class.is_some_and(|class| class.technique_slots == 1) {
                ""
            } else {
                "S"
            },
            if campaign
                .colony
                .has_active_upgrade(BuildingKind::Barracks, SIMULATION_HALL_UPGRADE)
            {
                " · SIMULATION HALL -10 XP TRIALS"
            } else {
                ""
            }
        ),
        344.0,
        478.0,
        TextStyle::new(12.0, dark::ACCENT).params(),
    );
    if let Some(class) = active_class {
        for (index, technique) in class.techniques.iter().enumerate() {
            let required_experience =
                campaign.skill_experience_required(technique.experience_required);
            let rect = Rect::new(344.0 + index as f32 * 434.0, 486.0, 426.0, 28.0);
            if rect.contains(mouse) {
                inspected_technique = Some(technique);
            }
            let learned = character.learned_skills.contains(&technique.id);
            let active = character.active_skills.contains(&technique.id);
            let equipped_count = character
                .active_skills
                .iter()
                .filter(|skill| class.techniques.iter().any(|entry| &entry.id == *skill))
                .count();
            let has_barracks = campaign.colony.has_facility(BuildingKind::Barracks);
            let has_experience = character.experience >= required_experience;
            let has_slot = equipped_count < class.technique_slots as usize;
            let enabled = if learned {
                active || has_slot
            } else {
                has_barracks && has_experience
            };
            let label = technique_label(
                active,
                learned,
                has_barracks,
                has_experience,
                has_slot,
                &technique.name,
                required_experience,
            );
            if button(rect, "", enabled, mouse) {
                actions.push(if learned {
                    UiAction::ToggleSelectedSkill(technique.id.clone())
                } else {
                    UiAction::LearnSelectedSkill(technique.id.clone())
                });
            }
            draw_ui_text_ex(
                &label.to_uppercase(),
                rect.x + 10.0,
                rect.y + 19.0,
                TextStyle::new(
                    11.0,
                    if enabled || active {
                        dark::TEXT
                    } else {
                        dark::TEXT_DIM
                    },
                )
                .params(),
            );
        }
    }
    inspected_technique
}

struct EquipmentControlsContext<'a> {
    campaign: &'a CampaignState,
    character: &'a crate::campaign::CharacterRecord,
    data: &'a GameData,
    assets: &'a AssetManager,
    visuals: &'a VisualCatalog,
    mouse: Vec2,
    roster_inspection_id: Option<&'a str>,
    actions: &'a mut Vec<UiAction>,
}

fn draw_equipment_controls(context: EquipmentControlsContext<'_>) {
    let EquipmentControlsContext {
        campaign,
        character,
        data,
        assets,
        visuals,
        mouse,
        roster_inspection_id,
        actions,
    } = context;
    draw_ui_text_ex(
        "WORKSHOP // EQUIPMENT",
        344.0,
        540.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    for (index, item) in data.equipment.iter().enumerate() {
        let cost = equipment_cost(&item.slot, &data.config);
        let equipped = character.equipment_ids.contains(&item.id);
        let unlocked = campaign.equipment_is_unlocked(item);
        let prototype_available =
            campaign.salvage_prototypes > 0 && item.required_protocol.is_empty();
        let has_workshop = campaign.colony.has_facility(BuildingKind::Workshop);
        let affordable = prototype_available || campaign.colony.resources.materials >= cost as i32;
        let enabled = has_workshop && unlocked && !equipped && affordable;
        let rect = equipment_row_rect(index);
        let label = equipment_label(
            unlocked,
            equipped,
            prototype_available,
            has_workshop,
            affordable,
            &item.name,
            cost,
        );
        let craft_clicked = button(rect, "", enabled, mouse);
        let info_clicked = button(
            equipment_info_rect(rect),
            if roster_inspection_id == Some(item.id.as_str()) {
                "!"
            } else {
                "?"
            },
            true,
            mouse,
        );
        if craft_clicked && !info_clicked {
            actions.push(UiAction::CraftSelected(item.id.clone()));
        }
        if info_clicked {
            actions.push(UiAction::InspectRosterEquipment(item.id.clone()));
        }
        if let Some(index) = crate::visual_assets::equipment_index(&item.id) {
            visuals.draw_atlas_cell(
                assets,
                &visuals.equipment,
                index,
                Rect::new(rect.x + 5.0, rect.y + 1.0, 30.0, 22.0),
                if unlocked {
                    WHITE
                } else {
                    Color::new(0.35, 0.38, 0.40, 1.0)
                },
            );
        }
        draw_ui_text_ex(
            &label.to_uppercase(),
            rect.x + 42.0,
            rect.y + 17.0,
            TextStyle::new(
                9.5,
                if enabled || equipped {
                    dark::TEXT
                } else {
                    dark::TEXT_DIM
                },
            )
            .params(),
        );
    }
    if button(
        Rect::new(1020.0, 103.0, 214.0, 28.0),
        "RETURN TO COLONY",
        true,
        mouse,
    ) {
        actions.push(UiAction::ReturnToColony);
    }
}
