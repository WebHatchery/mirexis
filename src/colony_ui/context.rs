use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::visual_assets::VisualCatalog;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::ui::VirtualUi;

pub(crate) struct ColonyDrawContext<'a> {
    pub(crate) campaign: &'a CampaignState,
    pub(crate) data: &'a GameData,
    pub(crate) assets: &'a AssetManager,
    pub(crate) visuals: &'a VisualCatalog,
    pub(crate) ui: &'a VirtualUi,
    pub(crate) camera: &'a mut crate::grid_ui::WorldCamera,
    pub(crate) explorer: &'a mut crate::colony_exploration::ColonyExplorer,
    pub(crate) operations_open: &'a mut bool,
    pub(crate) facility_upgrade_open: &'a mut bool,
    pub(crate) salvage_open: &'a mut bool,
    pub(crate) settings_open: bool,
    pub(crate) field_notes_open: bool,
    pub(crate) memorial_open: bool,
    pub(crate) selected_field_note: usize,
}
