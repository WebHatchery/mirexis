//! Colony hub presentation and strategic intent production.

use crate::campaign::{Availability, CampaignState};
use crate::colony::BuildingKind;
use crate::data::GameData;
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::VirtualUi;

// Dense late-campaign hubs can exhaust Macroquad's per-font-size glyph atlas
// when every label shares the toolkit font. The hub's buttons and map labels
// already use the built-in font, so keep all colony text on that stable atlas.
fn draw_ui_text_ex<'a>(text: &str, x: f32, y: f32, mut params: TextParams<'a>) -> TextDimensions {
    params.font = None;
    draw_text_ex(text, x, y, params)
}

pub fn draw_colony(
    campaign: &CampaignState,
    data: &GameData,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    ui: &VirtualUi,
    camera: &mut crate::grid_ui::WorldCamera,
) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = crate::ui::pointer_position(ui);
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    let suppress_actions =
        crate::colony_map_ui::draw(campaign, assets, visuals, ui, camera, mouse, &mut actions);
    crate::colony_header_ui::draw(campaign, mouse, &mut actions);
    draw_operations(campaign, data, assets, visuals, mouse, &mut actions);
    draw_ui_text_ex(
        "PAD // D-PAD SELECT MISSION · A BRIEF · X ROSTER · B TITLE  //  TAP // TITLE · BUILD · RESEARCH · CHOOSE",
        28.0,
        707.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    crate::ui::suppress_map_release_actions(&mut actions, suppress_actions);
    actions
}

fn draw_operations(
    campaign: &CampaignState,
    data: &GameData,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let panel = Rect::new(920.0, 74.0, 350.0, 608.0);
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
    let choosing_contact =
        campaign.strategy.isolation_complete && campaign.strategy.contact_protocol_id.is_empty();
    let choosing_escalation = campaign.strategy.escalation_operation_completed
        && campaign.strategy.escalation_response_id.is_empty();
    let choosing_mirexis =
        campaign.strategy.escalation_complete && campaign.strategy.mirexis_path_id.is_empty();
    let decision = if choosing_contact {
        Some(crate::colony_decision_ui::DecisionKind::Contact)
    } else if choosing_escalation {
        Some(crate::colony_decision_ui::DecisionKind::Escalation)
    } else if choosing_mirexis {
        Some(crate::colony_decision_ui::DecisionKind::Mirexis)
    } else {
        None
    };
    if let Some(decision) = decision {
        crate::colony_decision_ui::draw_decision_dossier(decision, campaign, assets, visuals);
    } else {
        draw_ui_text_ex(
            "MISSION OFFERS",
            878.0,
            372.0,
            TextStyle::new(15.0, dark::ACCENT).params(),
        );
        for (index, mission) in campaign.strategy.mission_offers.iter().take(2).enumerate() {
            let selected = mission.id == campaign.strategy.selected_mission_id;
            let danger = crate::danger_rating::for_instance(mission, data);
            if colony_button(
                Rect::new(878.0, 384.0 + index as f32 * 38.0, 362.0, 32.0),
                &format!(
                    "{}{} // {}",
                    if selected { "> " } else { "" },
                    mission.name,
                    danger.label()
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
    }
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
            draw_ending_card(campaign, assets, visuals);
            draw_ui_text_ex(
                &path.ending_title,
                968.0,
                526.0,
                TextStyle::new(14.0, dark::POSITIVE).params(),
            );
            draw_ui_text_ex(
                &path.revelation,
                968.0,
                548.0,
                TextStyle::new(9.5, dark::TEXT).params(),
            );
            draw_ui_text_ex(
                &path.legacy,
                968.0,
                565.0,
                TextStyle::new(9.5, dark::TEXT_DIM).params(),
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
        if let Some(event) = campaign.strategy.available_event() {
            draw_character_event(campaign, data, assets, visuals, event, mouse, actions);
        } else if let Some(research) = campaign
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
            let icon = match research.id.as_str() {
                "xeno_triage" => 8,
                "salvage_doctrine" => 10,
                _ => 5,
            };
            visuals.draw_atlas_cell(
                assets,
                &visuals.terrain,
                icon,
                Rect::new(878.0, 548.0, 48.0, 42.0),
                WHITE,
            );
            draw_rectangle_lines(878.0, 548.0, 48.0, 42.0, 1.0, dark::ACCENT);
            for (index, line) in wrap_words(&research.description, 46)
                .into_iter()
                .take(2)
                .enumerate()
            {
                draw_ui_text_ex(
                    &line,
                    936.0,
                    558.0 + index as f32 * 14.0,
                    TextStyle::new(10.0, dark::TEXT_DIM).params(),
                );
            }
            draw_ui_text_ex(
                &format!(
                    "PROJECT READY // COST {} MAT // GRID POWER +{}",
                    research.materials_cost, research.power_reward
                ),
                936.0,
                588.0,
                TextStyle::new(9.5, dark::POSITIVE).params(),
            );
        }
    }
    if !choosing_contact
        && !choosing_escalation
        && !choosing_mirexis
        && !evolution_pending
        && campaign.strategy.available_event().is_none()
    {
        draw_ui_text_ex(
            "ACTIVE DOCTRINES",
            878.0,
            600.0,
            TextStyle::new(12.0, dark::ACCENT).params(),
        );
        let completed = campaign
            .strategy
            .research
            .iter()
            .filter(|entry| entry.completed)
            .collect::<Vec<_>>();
        if completed.is_empty() {
            draw_ui_text_ex(
                "No completed field doctrine",
                878.0,
                616.0,
                TextStyle::new(10.0, dark::TEXT_DIM).params(),
            );
        } else {
            for (index, research) in completed.into_iter().take(3).enumerate() {
                let rect = Rect::new(878.0 + index as f32 * 120.0, 606.0, 114.0, 32.0);
                draw_rectangle(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    Color::new(0.045, 0.105, 0.11, 1.0),
                );
                visuals.draw_atlas_cell(
                    assets,
                    &visuals.terrain,
                    [5, 8, 10][index],
                    Rect::new(rect.x + 2.0, rect.y + 2.0, 30.0, 28.0),
                    WHITE,
                );
                draw_ui_text_ex(
                    &research.name.to_uppercase(),
                    rect.x + 36.0,
                    rect.y + 20.0,
                    TextStyle::new(10.0, dark::TEXT).params(),
                );
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, dark::POSITIVE);
            }
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
    assets: &AssetManager,
    visuals: &VisualCatalog,
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
    draw_ui_text_ex(
        "COLONY EVENT // DECISION",
        878.0,
        510.0,
        TextStyle::new(12.0, dark::WARNING).params(),
    );
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
    if let Some(definition) = definition {
        for (index, participant) in definition.participants.iter().take(2).enumerate() {
            if let Some(character) = campaign
                .roster
                .iter()
                .find(|character| &character.id == participant)
            {
                crate::portrait_ui::draw_character_portrait(
                    assets,
                    visuals,
                    Rect::new(878.0 + index as f32 * 56.0, 518.0, 50.0, 58.0),
                    &character.id,
                    &character.name,
                    dark::WARNING,
                );
            }
        }
        draw_ui_text_ex(
            &event.title.to_uppercase(),
            994.0,
            526.0,
            TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
        );
        for (index, line) in wrap_words(&definition.description, 38)
            .into_iter()
            .take(3)
            .enumerate()
        {
            draw_ui_text_ex(
                &line,
                994.0,
                545.0 + index as f32 * 14.0,
                TextStyle::new(10.0, dark::TEXT_DIM).params(),
            );
        }
    }
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
        592.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    if colony_button(
        Rect::new(878.0, 606.0, 362.0, 32.0),
        &format!("RESOLVE // {}", event.title.to_uppercase()),
        true,
        mouse,
    ) {
        actions.push(UiAction::ResolveCharacterEvent);
    }
}

fn wrap_words(value: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in value.split_whitespace() {
        if !line.is_empty() && line.len() + word.len() + 1 > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

fn draw_ending_card(campaign: &CampaignState, assets: &AssetManager, visuals: &VisualCatalog) {
    let (cell, accent) = match campaign.strategy.mirexis_path_id.as_str() {
        "human_redoubt" => (10, Color::new(0.36, 0.70, 0.88, 1.0)),
        "living_commonwealth" => (9, Color::new(0.64, 0.92, 0.38, 1.0)),
        "open_threshold" => (11, Color::new(0.72, 0.52, 1.0, 1.0)),
        _ => (2, dark::ACCENT),
    };
    let rect = Rect::new(878.0, 512.0, 80.0, 72.0);
    visuals.draw_atlas_cell(assets, &visuals.terrain, cell, rect, WHITE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, accent);
    draw_line(
        rect.x + 8.0,
        rect.bottom() - 8.0,
        rect.right() - 8.0,
        rect.y + 8.0,
        2.0,
        accent,
    );
}

fn colony_button(rect: Rect, label: &str, enabled: bool, mouse: Vec2) -> bool {
    crate::ui_widgets::button(rect, label, enabled, mouse)
}
