//! Runtime asset, campaign, and application-state initialization.

use super::{AppState, Game};
use crate::campaign::CampaignState;
use crate::combat_feedback::CombatFeedback;
use crate::data::GameData;
use crate::formation::FormationKind;
use crate::grid_ui::WorldCamera;
use crate::phase_replay::PhaseReplay;
use crate::state::GameSession;
use crate::ui::UiAction;
use crate::ui_action::BattleLogFilter;
use crate::visual_assets::VisualCatalog;
use macroquad::prelude::Texture2D;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::NotificationManager;
use macroquad_toolkit::persistence::{slot_exists, AutoSaveManager};
use macroquad_toolkit::prelude::GamepadInput;

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load()
            .unwrap_or_else(|err| panic!("Mirexis embedded data failed to load: {}", err));
        let audio = crate::audio::AudioSystem::new(&data.config.game_name).await;
        let mut assets = AssetManager::new();
        let placeholder = crate::visual_assets::diagnostic_placeholder_image(32);
        assets.set_placeholder_texture_direct(Texture2D::from_image(&placeholder));
        if let Err(error) = assets.load_asset_pack("assets.zip").await {
            println!(
                "Mirexis runtime asset pack unavailable ({error}); loading loose assets for native development."
            );
        }
        let loaded_assets = assets.load_texture_configs(&data.texture_manifest).await;
        let visuals = VisualCatalog::load();
        let missing_visuals = visuals.validate_loaded(&assets);
        let visual_summary = visuals.diagnostic_summary(&assets);
        println!("Mirexis visual catalog: {visual_summary}");
        let campaign = CampaignState::new(&data);
        let active_mission = campaign
            .strategy
            .materialize_selected(&data, &campaign.colony);
        let session = GameSession::new(
            &data.config,
            &active_mission,
            &campaign.deployment_roster(&data, &active_mission),
        );
        let save_exists = slot_exists(&data.config.game_name, &data.config.save_slot);
        let mut notifications = NotificationManager::new();
        audio.report_failures(&mut notifications);
        notifications.info(format!(
            "Visual pipeline online; {} textures; {} sprite definitions",
            loaded_assets,
            visuals.units.len()
        ));
        if !missing_visuals.is_empty() {
            notifications.danger(format!(
                "MISSING VISUAL ASSETS: {}",
                missing_visuals.join(", ")
            ));
        }

        let tactical_camera = WorldCamera::tactical_start(session.tactical.selected_tile);
        let colony_camera = WorldCamera::colony_start(crate::colony::SETTLEMENT_CENTER);
        Self {
            data,
            session,
            campaign,
            active_mission,
            state: AppState::Title,
            assets,
            visuals,
            notifications,
            events: EventBus::<UiAction>::new(),
            save_exists,
            last_outcome: None,
            autosave: AutoSaveManager::default(),
            targeting: None,
            show_tactical_help: false,
            show_battle_log: false,
            battle_log_filter: BattleLogFilter::default(),
            combat_feedback: CombatFeedback::default(),
            observed_event_count: 0,
            phase_replay: PhaseReplay::default(),
            deployment_formation: FormationKind::default(),
            end_phase_armed: false,
            gamepad: GamepadInput::new(),
            gamepad_connected: false,
            title_focus_continue: false,
            title_focus_active: false,
            title_hover_preview: false,
            new_campaign_armed: false,
            delete_save_armed: false,
            tactical_camera,
            colony_camera,
            colony_explorer: crate::colony_exploration::ColonyExplorer::default(),
            colony_operations_open: false,
            facility_upgrade_open: false,
            salvage_open: false,
            audio,
            show_settings: false,
            show_field_notes: false,
            selected_field_note: 0,
        }
    }
}
