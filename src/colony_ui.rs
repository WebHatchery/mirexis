//! Colony hub presentation and strategic intent production.

use crate::campaign::{Availability, CampaignState};
use crate::colony::{BuildingKind, COLONY_HEIGHT, COLONY_WIDTH};
use crate::data::GameData;
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::VirtualUi;

// Dense late-campaign hubs can exhaust Macroquad's per-font-size glyph atlas
// when every label shares the toolkit font. The hub's buttons and map labels
// already use the built-in font, so keep all colony text on that stable atlas.
fn draw_ui_text_ex<'a>(text: &str, x: f32, y: f32, mut params: TextParams<'a>) -> TextDimensions {
    params.font = None;
    draw_text_ex(text, x, y, params)
}

pub fn draw_colony(campaign: &CampaignState, data: &GameData, ui: &VirtualUi) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ui.mouse_position();
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    draw_header(campaign);
    draw_layout(campaign, mouse, &mut actions);
    draw_operations(campaign, data, mouse, &mut actions);
    actions
}

fn draw_header(campaign: &CampaignState) {
    draw_surface(
        Rect::new(18.0, 16.0, LOGICAL_WIDTH - 36.0, 66.0),
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.085, 0.98))
            .with_border(1.0, Color::new(0.22, 0.62, 0.56, 0.8))
            .with_left_accent(5.0, Color::new(0.34, 0.86, 0.68, 1.0)),
    );
    draw_ui_text_ex(
        "MIREXIS COLONY",
        42.0,
        57.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &campaign.strategy.phase_name,
        292.0,
        54.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    draw_ui_text_ex(
        &format!("OPERATIONS COMPLETED  {}", campaign.operations_completed),
        940.0,
        54.0,
        TextStyle::new(16.0, dark::TEXT_DIM).params(),
    );
}

fn draw_layout(campaign: &CampaignState, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let panel = Rect::new(18.0, 96.0, 820.0, 580.0);
    draw_surface_with_title(
        panel,
        Some("SETTLEMENT LAYOUT // PLAN CONSTRUCTION // CLICK DAMAGED BUILDINGS TO REPAIR"),
        &SurfaceStyle::new(Color::new(0.035, 0.052, 0.062, 0.98))
            .with_border(1.0, Color::new(0.19, 0.40, 0.40, 0.9))
            .with_header(42.0, Color::new(0.06, 0.10, 0.11, 1.0)),
        TextStyle::new(15.0, dark::TEXT),
    );
    let origin = vec2(105.0, 172.0);
    let tile_size = 78.0;
    for y in 0..COLONY_HEIGHT {
        for x in 0..COLONY_WIDTH {
            let rect = Rect::new(
                origin.x + x as f32 * tile_size,
                origin.y + y as f32 * tile_size,
                tile_size - 4.0,
                tile_size - 4.0,
            );
            let building = campaign
                .colony
                .buildings
                .iter()
                .find(|building| building.position == [x, y]);
            let project = campaign
                .colony
                .construction_queue
                .iter()
                .find(|project| project.position == [x, y]);
            let damaged = building.is_some_and(|building| building.damaged);
            let unpowered = building.is_some_and(|building| {
                !building.damaged && !campaign.colony.building_is_powered(&building.id)
            });
            let fill = if damaged {
                Color::new(0.34, 0.10, 0.10, 1.0)
            } else if unpowered {
                Color::new(0.13, 0.15, 0.20, 1.0)
            } else if building.is_some() {
                Color::new(0.10, 0.30, 0.26, 1.0)
            } else if project.is_some() {
                Color::new(0.36, 0.27, 0.10, 1.0)
            } else if rect.contains_point(mouse) {
                Color::new(0.12, 0.22, 0.21, 1.0)
            } else {
                Color::new(0.07, 0.12, 0.125, 1.0)
            };
            draw_surface(
                rect,
                &SurfaceStyle::new(fill).with_border(
                    1.0,
                    if damaged {
                        dark::NEGATIVE
                    } else {
                        Color::new(0.18, 0.38, 0.36, 0.8)
                    },
                ),
            );
            if let Some(building) = building {
                draw_building_label(rect, building.kind.name());
                if building.damaged {
                    draw_text(
                        format!("REPAIR {} MAT", building.kind.repair_cost()),
                        rect.x + 7.0,
                        rect.y + 66.0,
                        10.0,
                        dark::NEGATIVE,
                    );
                    if rect.contains_point(mouse) && is_mouse_button_released(MouseButton::Left) {
                        actions.push(UiAction::RepairBuilding(building.id.clone()));
                    }
                } else if unpowered {
                    draw_text("NO POWER", rect.x + 7.0, rect.y + 66.0, 10.0, dark::WARNING);
                } else if building.kind == BuildingKind::GeneLab {
                    draw_text("OPEN LAB", rect.x + 7.0, rect.y + 66.0, 10.0, dark::ACCENT);
                    if rect.contains_point(mouse) && is_mouse_button_released(MouseButton::Left) {
                        actions.push(UiAction::OpenGeneLab);
                    }
                }
            } else if let Some(project) = project {
                draw_building_label(rect, &format!("{}\nPLANNED", project.kind.name()));
            } else if rect.contains_point(mouse) && is_mouse_button_released(MouseButton::Left) {
                actions.push(UiAction::ConstructBuilding(
                    campaign.colony.planned_construction,
                    [x, y],
                ));
            }
        }
    }
    let mut construction_kinds = vec![BuildingKind::Barricade, BuildingKind::PowerPlant];
    if campaign.strategy.contact_complete
        && !campaign
            .colony
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::GeneLab)
        && !campaign
            .colony
            .construction_queue
            .iter()
            .any(|project| project.kind == BuildingKind::GeneLab)
    {
        construction_kinds.push(BuildingKind::GeneLab);
    }
    for (index, kind) in construction_kinds.into_iter().enumerate() {
        let selected = campaign.colony.planned_construction == kind;
        if colony_button(
            Rect::new(105.0 + index as f32 * 210.0, 640.0, 200.0, 30.0),
            &format!(
                "{}PLAN {} // {} MAT",
                if selected { "> " } else { "" },
                kind.name().to_uppercase(),
                kind.material_cost()
            ),
            true,
            mouse,
        ) {
            actions.push(UiAction::SelectConstruction(kind));
        }
    }
}

fn draw_building_label(rect: Rect, label: &str) {
    let short = match label {
        "Command Centre" => "COMMAND\nCENTRE",
        "Power Plant" => "POWER\nPLANT",
        "Hydroponics" => "HYDRO\nPONICS",
        "Gene Lab" => "GENE\nLAB",
        other => other,
    };
    for (index, line) in short.lines().enumerate() {
        draw_text(
            line,
            rect.x + 7.0,
            rect.y + 30.0 + index as f32 * 18.0,
            14.0,
            dark::TEXT,
        );
    }
}

fn draw_operations(
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let panel = Rect::new(856.0, 96.0, 406.0, 580.0);
    draw_surface_with_title(
        panel,
        Some("COLONY OPERATIONS"),
        &SurfaceStyle::new(Color::new(0.055, 0.07, 0.08, 0.98))
            .with_border(1.0, Color::new(0.25, 0.38, 0.40, 0.8))
            .with_header(42.0, Color::new(0.08, 0.105, 0.115, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let resources = &campaign.colony.resources;
    draw_ui_text_ex(
        &format!(
            "MATERIALS {:>3}  //  POWER {:>2}/{:>2}  //  FOOD {:>3}",
            resources.materials,
            campaign.colony.power_supply(),
            campaign.colony.power_demand(),
            resources.food
        ),
        878.0,
        166.0,
        TextStyle::new(15.0, dark::TEXT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "BIOMASS {:>2}  //  ALIEN COMPONENTS {:>2}",
            resources.biomass, resources.alien_components
        ),
        878.0,
        192.0,
        TextStyle::new(15.0, dark::TEXT).params(),
    );
    let attention = campaign
        .strategy
        .factions
        .iter()
        .map(|faction| format!("{} {}", faction.name, faction.attention))
        .collect::<Vec<_>>()
        .join("  /  ");
    draw_ui_text_ex(
        &attention,
        878.0,
        218.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    let doctrine_complete = campaign
        .strategy
        .research
        .iter()
        .any(|research| research.completed);
    let phase_progress = if campaign.strategy.campaign_complete {
        data.campaign
            .mirexis_paths
            .iter()
            .find(|path| path.id == campaign.strategy.mirexis_path_id)
            .map_or_else(
                || "MIREXIS // FINAL OPERATION WON // COLONY ENDURES".to_owned(),
                |path| {
                    format!(
                        "MIREXIS // {} // FINAL OPERATION WON",
                        path.name.to_uppercase()
                    )
                },
            )
    } else if campaign.strategy.escalation_complete {
        data.campaign
            .mirexis_paths
            .iter()
            .find(|path| path.id == campaign.strategy.mirexis_path_id)
            .map_or_else(
                || {
                    "ESCALATION // ADAPTATION COMPLETE // THREE POWERS CLOSING // PHASE FIVE"
                        .to_owned()
                },
                |path| {
                    format!(
                        "ESCALATION // ADAPTATION COMPLETE // PHASE FIVE // {}",
                        path.name.to_uppercase()
                    )
                },
            )
    } else if campaign.strategy.adaptation_complete {
        if !campaign.strategy.escalation_operation_completed {
            "ESCALATION // ADAPTATION COMPLETE // THREE POWERS CLOSING".to_owned()
        } else {
            data.campaign
                .escalation_responses
                .iter()
                .find(|response| response.id == campaign.strategy.escalation_response_id)
                .map_or_else(
                    || "ESCALATION // THREE KNIVES WON // RESPONSE REQUIRED".to_owned(),
                    |response| {
                        format!(
                            "ESCALATION // {} // BRANCH {}",
                            response.name.to_uppercase(),
                            if campaign.strategy.escalation_branch_completed {
                                "WON"
                            } else {
                                "PENDING"
                            }
                        )
                    },
                )
        }
    } else if campaign.strategy.contact_complete {
        let (operation, evolved, lab) = campaign.adaptation_completion_progress();
        format!(
            "ADAPTATION // GLASS {} // EVOLVED {}/2 // LAB {}",
            if operation { "WON" } else { "PENDING" },
            evolved.min(2),
            if lab { "READY" } else { "PENDING" }
        )
    } else if campaign.strategy.isolation_complete {
        data.campaign
            .contact_protocols
            .iter()
            .find(|protocol| protocol.id == campaign.strategy.contact_protocol_id)
            .map_or_else(
                || "CONTACT // CHOOSE A PROTOCOL // 2 COMPONENTS AVAILABLE".to_owned(),
                |_protocol| {
                    let (trace, aftermath, prototype) = campaign.contact_completion_progress(data);
                    format!(
                        "CONTACT // TRACE {} // AFTERMATH {} // PROTOTYPE {}",
                        if trace { "READY" } else { "PENDING" },
                        if aftermath { "READY" } else { "PENDING" },
                        if prototype { "READY" } else { "PENDING" }
                    )
                },
            )
    } else {
        format!(
            "ISOLATION // VICTORIES {}/3 // DOCTRINE {} // ASSAULT {}",
            campaign.strategy.isolation_victories,
            if doctrine_complete {
                "READY"
            } else {
                "PENDING"
            },
            if campaign.strategy.first_assault_repulsed {
                "REPELLED"
            } else {
                "PENDING"
            }
        )
    };
    draw_ui_text_ex(
        &phase_progress,
        878.0,
        232.0,
        TextStyle::new(
            10.0,
            if campaign.strategy.isolation_complete {
                dark::POSITIVE
            } else {
                dark::TEXT_DIM
            },
        )
        .params(),
    );
    if let Some(threat) = campaign.strategy.active_threat() {
        draw_ui_text_ex(
            &format!(
                "{}  //  {} OPERATIONS  //  STRENGTH {}",
                threat.name, threat.operations_until, threat.strength
            ),
            878.0,
            248.0,
            TextStyle::new(12.0, dark::WARNING).params(),
        );
    }
    let ready = campaign
        .roster
        .iter()
        .filter(|character| character.availability == Availability::Ready)
        .count();
    let recovering = campaign.roster.len() - ready;
    let defense = campaign.colony.defense_map();
    draw_ui_text_ex(
        &format!(
            "ROSTER READY {} / RECOVERING {}  //  DEFENCE {} COVER / {} CRITICAL",
            ready,
            recovering,
            defense.cover_tiles.len(),
            defense.critical_objectives.len()
        ),
        878.0,
        262.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    if colony_button(
        Rect::new(878.0, 274.0, 362.0, 32.0),
        "MANAGE ROSTER // TRAINING & GEAR",
        true,
        mouse,
    ) {
        actions.push(UiAction::OpenRoster);
    }
    if colony_button(
        Rect::new(878.0, 312.0, 362.0, 32.0),
        "INFIRMARY: PRIORITY TREATMENT",
        recovering > 0,
        mouse,
    ) {
        actions.push(UiAction::TreatInjury);
    }
    draw_ui_text_ex(
        "MISSION OFFERS",
        878.0,
        372.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    for (index, mission) in campaign.strategy.mission_offers.iter().take(2).enumerate() {
        let selected = mission.id == campaign.strategy.selected_mission_id;
        let pressure = if mission.operation_modifier == crate::data::OperationModifier::None {
            ""
        } else {
            " [PRESSURE]"
        };
        if colony_button(
            Rect::new(878.0, 384.0 + index as f32 * 38.0, 362.0, 32.0),
            &format!(
                "{}{}{}",
                if selected { "> " } else { "" },
                mission.name,
                pressure
            ),
            true,
            mouse,
        ) {
            actions.push(UiAction::SelectMission(mission.id.clone()));
        }
    }
    if colony_button(
        Rect::new(878.0, 462.0, 362.0, 36.0),
        "BRIEF SELECTED MISSION",
        campaign.strategy.selected_mission().is_some(),
        mouse,
    ) {
        actions.push(UiAction::OpenMissionBriefing);
    }
    let choosing_contact =
        campaign.strategy.isolation_complete && campaign.strategy.contact_protocol_id.is_empty();
    let choosing_escalation = campaign.strategy.escalation_operation_completed
        && campaign.strategy.escalation_response_id.is_empty();
    let choosing_mirexis =
        campaign.strategy.escalation_complete && campaign.strategy.mirexis_path_id.is_empty();
    let evolution_pending = campaign.strategy.contact_complete
        && campaign.roster.iter().any(|character| {
            character.mutation_evolution_id.is_empty()
                && data
                    .mutations
                    .iter()
                    .find(|mutation| mutation.id == character.mutation_id)
                    .is_some_and(|mutation| !mutation.evolutions.is_empty())
        });
    if campaign.strategy.campaign_complete {
        if let Some(path) = data
            .campaign
            .mirexis_paths
            .iter()
            .find(|path| path.id == campaign.strategy.mirexis_path_id)
        {
            draw_ui_text_ex(
                &path.ending_title,
                878.0,
                520.0,
                TextStyle::new(14.0, dark::POSITIVE).params(),
            );
            draw_ui_text_ex(
                &path.revelation,
                878.0,
                544.0,
                TextStyle::new(11.0, dark::TEXT).params(),
            );
            draw_ui_text_ex(
                &path.legacy,
                878.0,
                564.0,
                TextStyle::new(11.0, dark::TEXT_DIM).params(),
            );
        }
    } else if choosing_contact {
        draw_ui_text_ex(
            "FIRST CONTACT // SPEND 2 COMPONENTS // CHOOSE ONE",
            878.0,
            514.0,
            TextStyle::new(11.0, dark::ACCENT).params(),
        );
        for (index, protocol) in data.campaign.contact_protocols.iter().enumerate() {
            let bonus = if protocol.materials_bonus > 0 {
                format!("+{} MATERIALS", protocol.materials_bonus)
            } else if protocol.biomass_bonus > 0 {
                format!("+{} BIOMASS", protocol.biomass_bonus)
            } else {
                format!("+{} POWER", protocol.power_bonus)
            };
            if colony_button(
                Rect::new(878.0, 516.0 + index as f32 * 31.0, 362.0, 30.0),
                &format!("{} // {}", protocol.name.to_uppercase(), bonus),
                campaign.colony.resources.alien_components >= protocol.alien_components_cost,
                mouse,
            ) {
                actions.push(UiAction::ChooseContactProtocol(protocol.id.clone()));
            }
        }
    } else if choosing_escalation {
        draw_ui_text_ex(
            "CONVERGENCE CAPTURED // CHOOSE ONE RESPONSE",
            878.0,
            514.0,
            TextStyle::new(11.0, dark::ACCENT).params(),
        );
        for (index, response) in data.campaign.escalation_responses.iter().enumerate() {
            let effect = if response.threat_delay > 0 {
                format!(
                    "{} MAT // DELAY ASSAULT +{}",
                    response.materials_cost, response.threat_delay
                )
            } else if response.attention_change_all < 0 {
                format!(
                    "{} BIOMASS // ATTENTION {}",
                    response.biomass_cost, response.attention_change_all
                )
            } else {
                format!(
                    "{} POWER // +{} MAT / ATTENTION +{}",
                    response.power_cost, response.materials_bonus, response.attention_change_all
                )
            };
            let affordable = campaign.colony.resources.materials >= response.materials_cost
                && campaign.colony.resources.biomass >= response.biomass_cost
                && campaign.colony.resources.power >= response.power_cost;
            if colony_button(
                Rect::new(878.0, 516.0 + index as f32 * 31.0, 362.0, 30.0),
                &format!("{} // {}", response.name.to_uppercase(), effect),
                affordable,
                mouse,
            ) {
                actions.push(UiAction::ChooseEscalationResponse(response.id.clone()));
            }
        }
    } else if choosing_mirexis {
        draw_ui_text_ex(
            "MIREXIS REVEALED // CHOOSE WHAT THE COLONY BECOMES",
            878.0,
            514.0,
            TextStyle::new(11.0, dark::ACCENT).params(),
        );
        for (index, path) in data.campaign.mirexis_paths.iter().enumerate() {
            let effect = if path.defense_cover_bonus > 0 {
                format!(
                    "{} MAT // DEFENCE COVER +{}",
                    path.materials_cost, path.defense_cover_bonus
                )
            } else if path.deployment_food_discount > 0 {
                format!(
                    "{} BIO // FOOD -{}",
                    path.biomass_cost, path.deployment_food_discount
                )
            } else {
                format!(
                    "{} POWER // VICTORY POWER +{}",
                    path.power_cost, path.power_bonus
                )
            };
            let affordable = campaign.colony.resources.materials >= path.materials_cost
                && campaign.colony.resources.biomass >= path.biomass_cost
                && campaign.colony.resources.power >= path.power_cost;
            if colony_button(
                Rect::new(878.0, 516.0 + index as f32 * 31.0, 362.0, 30.0),
                &format!("{} // {}", path.name.to_uppercase(), effect),
                affordable,
                mouse,
            ) {
                actions.push(UiAction::ChooseMirexisPath(path.id.clone()));
            }
        }
    } else if evolution_pending {
        let lab_exists = campaign
            .colony
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::GeneLab);
        let lab_ready = campaign.colony.has_facility(BuildingKind::GeneLab);
        draw_ui_text_ex(
            if lab_ready {
                "GENE LAB READY // CLICK THE FACILITY TO EVOLVE A COLONIST"
            } else if lab_exists {
                "GENE LAB OFFLINE // RESTORE POWER OR REPAIR THE FACILITY"
            } else {
                "GENE LAB REQUIRED // PLAN IT ON AN OPEN COLONY PLOT"
            },
            878.0,
            514.0,
            TextStyle::new(11.0, dark::ACCENT).params(),
        );
    } else {
        if let Some(research) = campaign
            .strategy
            .research
            .iter()
            .find(|entry| !entry.completed)
        {
            if colony_button(
                Rect::new(878.0, 510.0, 362.0, 32.0),
                &format!(
                    "RESEARCH {} // {} MAT",
                    research.name, research.materials_cost
                ),
                true,
                mouse,
            ) {
                actions.push(UiAction::CompleteResearch(research.id.clone()));
            }
        }
        if let Some(event) = campaign.strategy.available_event() {
            draw_character_event(campaign, data, event, mouse, actions);
        }
    }
    if !choosing_contact && !choosing_escalation && !choosing_mirexis && !evolution_pending {
        draw_ui_text_ex(
            "ACTIVE DOCTRINES",
            878.0,
            600.0,
            TextStyle::new(12.0, dark::ACCENT).params(),
        );
        let mut doctrine_y = 614.0;
        for research in campaign
            .strategy
            .research
            .iter()
            .filter(|entry| entry.completed)
        {
            draw_ui_text_ex(
                &format!("{} // {}", research.name, research.description),
                878.0,
                doctrine_y,
                TextStyle::new(10.0, dark::TEXT_DIM).params(),
            );
            doctrine_y += 11.0;
        }
        if doctrine_y == 614.0 {
            draw_ui_text_ex(
                "No completed field doctrine",
                878.0,
                doctrine_y,
                TextStyle::new(10.0, dark::TEXT_DIM).params(),
            );
        }
    }
    draw_ui_text_ex(
        &format!(
            "Plan: {} // {} materials // one operation // {} structures mapped",
            campaign.colony.planned_construction.name(),
            campaign.colony.planned_construction.material_cost(),
            defense.blocked_tiles.len()
        ),
        878.0,
        650.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
}

fn draw_character_event(
    campaign: &CampaignState,
    data: &GameData,
    event: &crate::strategy::CharacterEventState,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let definition = data
        .campaign
        .events
        .iter()
        .find(|definition| definition.id == event.id);
    let legacy_character_id = if event.legacy_character_id.is_empty() {
        definition.map_or("", |definition| definition.legacy_character_id.as_str())
    } else {
        event.legacy_character_id.as_str()
    };
    let legacy_stat = if event.legacy_stat.is_empty() {
        definition.map_or("", |definition| definition.legacy_stat.as_str())
    } else {
        event.legacy_stat.as_str()
    };
    let legacy_amount = if event.legacy_amount == 0 {
        definition.map_or(0, |definition| definition.legacy_amount)
    } else {
        event.legacy_amount
    };
    let attention_faction = if event.attention_faction.is_empty() {
        "directorate"
    } else {
        event.attention_faction.as_str()
    };
    let attention_name = campaign
        .strategy
        .factions
        .iter()
        .find(|faction| faction.id == attention_faction)
        .map_or(attention_faction, |faction| faction.name.as_str());
    if colony_button(
        Rect::new(878.0, 548.0, 362.0, 32.0),
        &format!("EVENT: {}", event.title),
        true,
        mouse,
    ) {
        actions.push(UiAction::ResolveCharacterEvent);
    }
    let recipient = campaign
        .roster
        .iter()
        .find(|character| character.id == legacy_character_id)
        .map_or("UNKNOWN", |character| {
            character
                .name
                .split_whitespace()
                .next()
                .unwrap_or("UNKNOWN")
        });
    draw_ui_text_ex(
        &format!(
            "CHOICE EFFECT // {} {:+} {} // BOND +2 // {} FOOD // {} {:+}",
            recipient.to_uppercase(),
            legacy_amount,
            legacy_stat.to_uppercase(),
            event.food_cost,
            attention_name.to_uppercase(),
            event.attention_change
        ),
        878.0,
        588.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
}

fn colony_button(rect: Rect, label: &str, enabled: bool, mouse: Vec2) -> bool {
    let hovered = enabled && rect.contains_point(mouse);
    let fill = if !enabled {
        Color::new(0.08, 0.10, 0.11, 1.0)
    } else if hovered {
        Color::new(0.18, 0.43, 0.37, 1.0)
    } else {
        Color::new(0.10, 0.28, 0.25, 1.0)
    };
    draw_surface(
        rect,
        &SurfaceStyle::new(fill).with_border(1.0, Color::new(0.33, 0.72, 0.60, 1.0)),
    );
    draw_text(
        label,
        rect.x + 14.0,
        rect.y + 27.0,
        15.0,
        if enabled { dark::TEXT } else { dark::TEXT_DIM },
    );
    hovered && is_mouse_button_released(MouseButton::Left)
}
