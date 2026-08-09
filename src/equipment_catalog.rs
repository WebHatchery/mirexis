//! Equipment-definition invariants shared by loading and roster crafting.

use crate::data::EquipmentDef;

pub(crate) fn validate_definitions(equipment: &[EquipmentDef]) -> Result<(), String> {
    for item in equipment {
        if item.description.trim().is_empty() {
            return Err(format!("Equipment {} has no description", item.id));
        }
        if item.slot != "primary"
            && (item.weapon_range_override > 0 || item.weapon_ap_cost_override > 0)
        {
            return Err(format!(
                "Non-primary equipment {} overrides weapon handling",
                item.id
            ));
        }
    }
    Ok(())
}
