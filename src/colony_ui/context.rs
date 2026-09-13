//! Colony draw inputs and the state snapshot returned as UI intents.

use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::visual_assets::VisualCatalog;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::ui::VirtualUi;

use crate::colony_exploration::ColonyExplorer;
use crate::grid_ui::WorldCamera;

pub struct ColonyDrawContext<'a> {
    pub campaign: &'a CampaignState,
    pub data: &'a GameData,
    pub assets: &'a AssetManager,
    pub visuals: &'a VisualCatalog,
    pub ui: &'a VirtualUi,
    pub camera: WorldCamera,
    pub explorer: ColonyExplorer,
    pub operations_open: bool,
    pub facility_upgrade_open: bool,
    pub salvage_open: bool,
    pub settings_open: bool,
    pub field_notes_open: bool,
    pub memorial_open: bool,
    pub memorial_page: usize,
    pub selected_field_note: usize,
}

pub struct ColonyDrawResult {
    pub actions: Vec<crate::ui::UiAction>,
    pub camera: WorldCamera,
    pub explorer: ColonyExplorer,
    pub operations_open: bool,
    pub facility_upgrade_open: bool,
    pub salvage_open: bool,
}
