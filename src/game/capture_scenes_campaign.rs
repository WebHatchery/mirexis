//! Capture builders for colony, campaign, and roster screens.

use super::*;
use crate::data::OperationModifier;

impl Game {
    pub(super) fn capture_campaign_scene(&mut self, scene: &str) -> bool {
        match scene {
            "title" => {
                self.state = AppState::Title;
                true
            }
            "title_controller" => {
                self.state = AppState::Title;
                self.save_exists = true;
                self.title_focus_continue = true;
                self.title_focus_active = true;
                true
            }
            "title_hover" => {
                self.state = AppState::Title;
                self.title_hover_preview = true;
                true
            }
            "colony" => {
                self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::MeetCoordinator;
                self.state = AppState::Colony;
                true
            }
            "facility_upgrades" => {
                self.capture_facility_upgrades();
                true
            }
            "settings" => {
                self.state = AppState::Colony;
                self.show_settings = true;
                true
            }
            "field_notes" => {
                self.capture_field_notes();
                true
            }
            "memorial" => {
                self.capture_memorial();
                true
            }
            "memorial_page_two" => {
                self.capture_memorial_page_two();
                true
            }
            "contact" => {
                self.capture_contact();
                true
            }
            "contact_gear" => {
                self.capture_contact_gear();
                true
            }
            "contact_event" => {
                self.capture_contact_event();
                true
            }
            "ninth_recruitment" => {
                self.capture_ninth_recruitment();
                true
            }
            "adaptation" => {
                self.capture_adaptation();
                true
            }
            "gene_lab" => {
                self.capture_gene_lab();
                true
            }
            "evolution" => {
                self.capture_evolution();
                true
            }
            "mara_evolution" => {
                self.capture_mara_evolution();
                true
            }
            "ilya_evolution" => {
                self.capture_ilya_evolution();
                true
            }
            "sol_evolution" => {
                self.capture_sol_evolution();
                true
            }
            "nadi_evolution" => {
                self.capture_nadi_evolution();
                true
            }
            "escalation" => {
                self.capture_escalation();
                true
            }
            "escalation_operation" => {
                self.capture_escalation_operation();
                true
            }
            "escalation_response" => {
                self.capture_escalation_response();
                true
            }
            "mirexis" => {
                self.capture_mirexis();
                true
            }
            "mirexis_path" => {
                self.capture_mirexis_path();
                true
            }
            "redoubt_end" => {
                self.capture_mirexis_end("human_redoubt");
                true
            }
            "commonwealth_end" => {
                self.capture_mirexis_end("living_commonwealth");
                true
            }
            "threshold_end" => {
                self.capture_mirexis_end("open_threshold");
                true
            }
            "finale_debrief" => {
                self.capture_finale_debrief();
                true
            }
            "adaptation_operation" => {
                self.capture_adaptation_operation();
                true
            }
            "glass_nerve" => {
                self.capture_template_operation(
                    "adaptation_glass_nerve",
                    14,
                    OperationModifier::AscendantInterference,
                );
                true
            }
            "three_knives" => {
                self.capture_template_operation(
                    "escalation_three_knives",
                    16,
                    OperationModifier::EscalationCrossfire,
                );
                true
            }
            "door_of_light" => {
                self.capture_template_operation(
                    "mirexis_threshold_door_of_light",
                    24,
                    OperationModifier::AscendantInterference,
                );
                true
            }
            "research" => {
                self.capture_research();
                true
            }
            "legacy" => {
                self.capture_legacy();
                true
            }
            "roster" => {
                self.state = AppState::Roster;
                true
            }
            "recruited_roster" => {
                self.capture_recruited_roster_screen();
                true
            }
            "ninth_roster" => {
                self.capture_ninth_roster();
                true
            }
            "roster_info" => {
                self.capture_recruited_roster_info();
                true
            }
            "recruited_gene_lab" => {
                self.capture_recruited_gene_lab();
                true
            }
            "advanced_roster" => {
                self.capture_advanced_roster();
                true
            }
            "relationships" => {
                self.capture_relationships(AppState::Roster);
                true
            }
            "trauma" => {
                self.capture_trauma();
                true
            }
            "bonded_briefing" => {
                self.capture_relationships(AppState::MissionBriefing);
                true
            }
            "briefing" => {
                self.campaign.first_hour.stage = crate::first_hour::FirstHourStage::FirstBriefing;
                self.state = AppState::MissionBriefing;
                true
            }
            "recruited_briefing" => {
                self.capture_recruited_briefing();
                true
            }
            "threat_briefing" => {
                self.capture_template_operation(
                    "sporefield_extraction",
                    4,
                    OperationModifier::BroodFrenzy,
                );
                self.state = AppState::MissionBriefing;
                true
            }
            "loadout_briefing" => {
                self.capture_trauma();
                self.state = AppState::MissionBriefing;
                true
            }
            "first_hour_guide" => {
                self.capture_first_hour_guide();
                true
            }
            "first_hour_return" => {
                self.capture_first_hour_return();
                true
            }
            "first_hour_dialogue" => {
                self.capture_first_hour_dialogue();
                true
            }
            "first_hour_promise" => {
                self.capture_first_hour_promise();
                true
            }
            "first_hour_operations" => {
                self.capture_first_hour_operations();
                true
            }
            _ => false,
        }
    }
}
