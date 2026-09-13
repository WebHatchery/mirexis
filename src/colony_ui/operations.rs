//! Operations drawer rendering and campaign decision surfaces.

use super::*;

pub(super) struct OperationsDrawContext<'a> {
    pub(super) campaign: &'a CampaignState,
    pub(super) data: &'a GameData,
    pub(super) assets: &'a AssetManager,
    pub(super) visuals: &'a VisualCatalog,
    pub(super) mouse: Vec2,
    pub(super) facility_upgrade_open: &'a mut bool,
    pub(super) salvage_open: &'a mut bool,
    pub(super) actions: &'a mut Vec<UiAction>,
}

pub(super) fn draw_operations(context: OperationsDrawContext<'_>) {
    let OperationsDrawContext {
        campaign,
        data,
        assets,
        visuals,
        mouse,
        facility_upgrade_open,
        salvage_open,
        actions,
    } = context;
    if *facility_upgrade_open {
        actions.clear();
        upgrades::draw_modal(campaign, mouse, actions);
        return;
    }
    if *salvage_open {
        actions.clear();
        salvage::draw_modal(campaign, mouse, actions);
        return;
    }
    let panel = Rect::new(862.0, 74.0, 408.0, 608.0);
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
    crate::field_notes_ui::draw_launcher(&campaign.colony_story, mouse, actions);
    crate::memorial_ui::draw_launcher(campaign, mouse, actions);
    if campaign.first_hour.stage == crate::first_hour::FirstHourStage::MakeInvestment {
        crate::first_hour_investment_ui::draw(campaign, mouse, actions);
        return;
    }
    let (pending_research, defense) = draw_operations_summary(campaign, data, mouse, actions);
    let (decision, choosing_contact, choosing_escalation, choosing_mirexis) =
        draw_decision_panel(campaign, data, assets, visuals, mouse, actions);
    let surfaces = draw_campaign_surfaces(CampaignSurfacesContext {
        campaign,
        data,
        assets,
        visuals,
        mouse,
        actions,
        pending_research,
        decision,
    });
    draw_operations_footer(OperationsFooterContext {
        campaign,
        data,
        assets,
        visuals,
        mouse,
        actions,
        choosing_contact,
        choosing_escalation,
        choosing_mirexis,
        evolution_pending: surfaces.evolution_pending,
        research_surface_visible: surfaces.research_surface_visible,
        defense,
    });
}

fn draw_operations_summary(
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) -> (bool, crate::colony::ColonyDefenseMap) {
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
    let pending_research = research::has_pending(campaign);
    draw_phase_progress(campaign, data, doctrine_complete);
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
        Rect::new(878.0, 312.0, 176.0, 32.0),
        &medical::treatment_button_label(campaign),
        campaign.can_treat_first_injury(),
        mouse,
    ) {
        actions.push(UiAction::TreatInjury);
    }
    upgrades::draw_launcher(campaign, mouse, actions);
    salvage::draw_launcher(campaign, mouse, actions);
    (pending_research, defense)
}

fn phase_progress_label(
    campaign: &CampaignState,
    data: &GameData,
    doctrine_complete: bool,
) -> String {
    if campaign.strategy.campaign_complete {
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
        let progress = campaign.adaptation_completion_progress();
        format!(
            "ADAPTATION // GLASS {} // EVOLVED {}/2 // LAB {}",
            if progress.operation_completed {
                "WON"
            } else {
                "PENDING"
            },
            progress.evolved_count.min(2),
            if progress.gene_lab_ready {
                "READY"
            } else {
                "PENDING"
            }
        )
    } else if campaign.strategy.isolation_complete {
        data.campaign
            .contact_protocols
            .iter()
            .find(|protocol| protocol.id == campaign.strategy.contact_protocol_id)
            .map_or_else(
                || "CONTACT // CHOOSE A PROTOCOL // 2 COMPONENTS AVAILABLE".to_owned(),
                |_protocol| {
                    let progress = campaign.contact_completion_progress(data);
                    format!(
                        "CONTACT // TRACE {} // AFTERMATH {} // PROTOTYPE {}",
                        if progress.trace_completed {
                            "READY"
                        } else {
                            "PENDING"
                        },
                        if progress.aftermath_resolved {
                            "READY"
                        } else {
                            "PENDING"
                        },
                        if progress.prototype_equipped {
                            "READY"
                        } else {
                            "PENDING"
                        }
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
    }
}

fn draw_phase_progress(campaign: &CampaignState, data: &GameData, doctrine_complete: bool) {
    let phase_progress = phase_progress_label(campaign, data, doctrine_complete);
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
}

fn draw_decision_panel(
    campaign: &CampaignState,
    data: &GameData,
    assets: &AssetManager,
    visuals: &VisualCatalog,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) -> (
    Option<crate::colony_decision_ui::DecisionKind>,
    bool,
    bool,
    bool,
) {
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
    commons::draw(campaign, decision.is_none(), mouse, actions);
    relay::draw(campaign, decision.is_none(), mouse, actions);
    if let Some(decision) = decision {
        crate::colony_decision_ui::draw_decision_dossier(decision, campaign, assets, visuals);
    } else {
        draw_ui_text_ex(
            if campaign
                .colony
                .has_active_upgrade(BuildingKind::CommandCentre, SIGNAL_CARTOGRAPHY_UPGRADE)
            {
                "MISSION ROUTES // SIGNAL CARTOGRAPHY ONLINE"
            } else {
                "MISSION OFFERS"
            },
            878.0,
            380.0,
            TextStyle::new(15.0, dark::ACCENT).params(),
        );
        crate::first_hour_investment_ui::draw_active_summary(campaign, vec2(1002.0, 380.0));
        for (index, mission) in campaign.strategy.mission_offers.iter().take(3).enumerate() {
            let selected = mission.id == campaign.strategy.selected_mission_id;
            let danger = crate::danger_rating::for_instance(mission, data);
            if colony_button(
                Rect::new(878.0, 386.0 + index as f32 * 28.0, 362.0, 26.0),
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
            mission_briefing_bounds(),
            "BRIEF SELECTED MISSION",
            campaign.strategy.selected_mission().is_some(),
            mouse,
        ) {
            actions.push(UiAction::OpenMissionBriefing);
        }
    }
    (
        decision,
        choosing_contact,
        choosing_escalation,
        choosing_mirexis,
    )
}

struct CampaignSurfacesContext<'a> {
    campaign: &'a CampaignState,
    data: &'a GameData,
    assets: &'a AssetManager,
    visuals: &'a VisualCatalog,
    mouse: Vec2,
    actions: &'a mut Vec<UiAction>,
    pending_research: bool,
    decision: Option<crate::colony_decision_ui::DecisionKind>,
}

struct CampaignSurfacesResult {
    evolution_pending: bool,
    research_surface_visible: bool,
}

fn draw_campaign_surfaces(context: CampaignSurfacesContext<'_>) -> CampaignSurfacesResult {
    let CampaignSurfacesContext {
        campaign,
        data,
        assets,
        visuals,
        mouse,
        actions,
        pending_research,
        decision,
    } = context;
    let evolution_pending = campaign.strategy.contact_complete
        && campaign.roster.iter().any(|character| {
            character.mutation_evolution_id.is_empty()
                && data
                    .mutations
                    .iter()
                    .find(|mutation| mutation.id == character.mutation_id)
                    .is_some_and(|mutation| !mutation.evolutions.is_empty())
        });
    let research_surface_visible = pending_research
        && !evolution_pending
        && campaign.strategy.available_event().is_none()
        && !campaign.outsider_recruit_available(data)
        && decision.is_none();
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
            if let Some(dossier) = crate::epilogue::derive(campaign) {
                draw_epilogue_dossier(&dossier);
            }
        }
    } else if let Some(decision) = decision {
        decisions::draw_choice_panel(decision, campaign, data, mouse, actions);
    } else if evolution_pending {
        let lab_exists = campaign
            .colony
            .buildings
            .iter()
            .any(|building| building.kind == BuildingKind::GeneLab);
        let lab_ready = campaign.colony.has_facility(BuildingKind::GeneLab);
        draw_ui_text_ex(
            if lab_ready {
                "GENE LAB READY // TAP THE FACILITY TO EVOLVE A COLONIST"
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
        } else if campaign.outsider_arc_available() {
            crate::outsider_ui::draw(campaign, assets, visuals, mouse, actions);
        } else if !campaign.outsider_recruit_available(data) {
            research::draw_available(campaign, assets, visuals, mouse, actions);
        }
    }
    CampaignSurfacesResult {
        evolution_pending,
        research_surface_visible,
    }
}

struct OperationsFooterContext<'a> {
    campaign: &'a CampaignState,
    data: &'a GameData,
    assets: &'a AssetManager,
    visuals: &'a VisualCatalog,
    mouse: Vec2,
    actions: &'a mut Vec<UiAction>,
    choosing_contact: bool,
    choosing_escalation: bool,
    choosing_mirexis: bool,
    evolution_pending: bool,
    research_surface_visible: bool,
    defense: crate::colony::ColonyDefenseMap,
}

fn draw_operations_footer(context: OperationsFooterContext<'_>) {
    let OperationsFooterContext {
        campaign,
        data,
        assets,
        visuals,
        mouse,
        actions,
        choosing_contact,
        choosing_escalation,
        choosing_mirexis,
        evolution_pending,
        research_surface_visible,
        defense,
    } = context;
    if !choosing_contact
        && !choosing_escalation
        && !choosing_mirexis
        && !evolution_pending
        && campaign.strategy.available_event().is_none()
        && !campaign.outsider_recruit_available(data)
    {
        research::draw_completed_summary(campaign, assets, visuals, research_surface_visible);
    }
    recruitment::draw(
        campaign,
        data,
        mouse,
        choosing_contact,
        choosing_escalation,
        choosing_mirexis,
        actions,
    );
    if should_draw_colony_plan(campaign) {
        draw_ui_text_ex(
            &format!(
                "Plan: {} // {} materials // one operation // {} structures mapped",
                campaign.colony.planned_construction.name(),
                campaign.colony.planned_construction.material_cost(),
                defense.blocked_tiles.len()
            ),
            878.0,
            colony_plan_baseline(campaign, research_surface_visible),
            TextStyle::new(12.0, dark::TEXT_DIM).params(),
        );
    }
}
