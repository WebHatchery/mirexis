//! Manifest-backed sprite, portrait, terrain, and colony art definitions.

use crate::data::Team;
use crate::state::UnitState;
use crate::tactical::{UnitAnimationState, UnitFacing};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use serde::Deserialize;

const DEFINITIONS_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/sprite_definitions.json");

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AtlasDefinition {
    #[serde(default)]
    pub id: String,
    pub texture: String,
    pub columns: usize,
    pub rows: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct UnitSpriteDefinition {
    pub id: String,
    #[serde(default)]
    pub id_prefixes: Vec<String>,
    #[serde(default)]
    pub name_contains: Vec<String>,
    pub texture: String,
    pub portrait_texture: String,
    pub portrait_index: usize,
    pub portrait_columns: usize,
    pub scale: f32,
    pub pivot: [f32; 2],
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct FactionChannelDefinition {
    pub id: String,
    pub accent: [f32; 4],
    pub terrain_cell: usize,
    pub effect_cell: usize,
    pub hostile_notches: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct VisualCatalog {
    pub unit_columns: usize,
    pub facing_rows: usize,
    pub faction_channels: Vec<FactionChannelDefinition>,
    pub units: Vec<UnitSpriteDefinition>,
    pub terrain: AtlasDefinition,
    pub colony: AtlasDefinition,
    pub equipment: AtlasDefinition,
    pub effects: AtlasDefinition,
    #[serde(default)]
    pub concepts: Vec<AtlasDefinition>,
}

impl VisualCatalog {
    pub(crate) fn load() -> Self {
        serde_json::from_str(DEFINITIONS_JSON)
            .unwrap_or_else(|error| panic!("Mirexis sprite definitions are invalid: {error}"))
    }

    fn unit_definition_for(&self, unit_id: &str, name: &str) -> Option<&UnitSpriteDefinition> {
        if let Some(definition) = self
            .units
            .iter()
            .find(|definition| definition.id == unit_id)
        {
            return Some(definition);
        }
        let name = name.to_ascii_lowercase();
        if let Some(definition) = self.units.iter().find(|definition| {
            definition
                .name_contains
                .iter()
                .any(|needle| name.contains(needle))
        }) {
            return Some(definition);
        }
        self.units.iter().find(|definition| {
            definition
                .id_prefixes
                .iter()
                .any(|prefix| unit_id.starts_with(prefix))
        })
    }

    pub(crate) fn faction_channel(&self, faction: &str) -> &FactionChannelDefinition {
        self.faction_channels
            .iter()
            .find(|channel| faction.contains(&channel.id))
            .or_else(|| {
                self.faction_channels
                    .iter()
                    .find(|channel| channel.id == "colony")
            })
            .expect("visual catalog requires a colony faction channel")
    }

    pub(crate) fn unit_faction_channel(&self, unit: &UnitState) -> &FactionChannelDefinition {
        if unit.team == Team::Colony {
            self.faction_channel("colony")
        } else {
            self.faction_channel(unit.faction.as_deref().unwrap_or("brood"))
        }
    }

    pub(crate) fn faction_accent(&self, unit: &UnitState) -> Color {
        let [r, g, b, a] = self.unit_faction_channel(unit).accent;
        Color::new(r, g, b, a)
    }

    pub(crate) fn faction_terrain_cell(&self, faction: &str) -> usize {
        self.faction_channel(faction).terrain_cell
    }

    pub(crate) fn faction_effect_cell(&self, faction: &str) -> usize {
        self.faction_channel(faction).effect_cell
    }

    pub(crate) fn concept_atlas(&self, id: &str) -> &AtlasDefinition {
        self.concepts
            .iter()
            .find(|atlas| atlas.id == id)
            .unwrap_or_else(|| panic!("Mirexis concept atlas is undefined: {id}"))
    }

    pub(crate) fn validate_loaded(&self, assets: &AssetManager) -> Vec<String> {
        let mut missing = Vec::new();
        for definition in &self.units {
            for key in [&definition.texture, &definition.portrait_texture] {
                if !assets.has_texture(key) && !missing.contains(key) {
                    missing.push(key.clone());
                }
            }
        }
        for key in [
            &self.terrain.texture,
            &self.colony.texture,
            &self.equipment.texture,
            &self.effects.texture,
        ] {
            if !assets.has_texture(key) && !missing.contains(key) {
                missing.push(key.clone());
            }
        }
        for atlas in &self.concepts {
            if !assets.has_texture(&atlas.texture) && !missing.contains(&atlas.texture) {
                missing.push(atlas.texture.clone());
            }
        }
        missing
    }

    pub(crate) fn diagnostic_summary(&self, assets: &AssetManager) -> String {
        let describe = |atlas: &AtlasDefinition| {
            assets.get_texture(&atlas.texture).map_or_else(
                || format!("{}=MISSING", atlas.texture),
                |texture| {
                    format!(
                        "{}={}x{}:{}x{}",
                        atlas.texture,
                        texture.width() as i32,
                        texture.height() as i32,
                        atlas.columns,
                        atlas.rows
                    )
                },
            )
        };
        format!(
            "{} unit definitions // {} concept atlases // {} // {} // {} // {}",
            self.units.len(),
            self.concepts.len(),
            describe(&self.terrain),
            describe(&self.colony),
            describe(&self.equipment),
            describe(&self.effects)
        )
    }

    pub(crate) fn draw_unit_sprite(
        &self,
        assets: &AssetManager,
        unit: &UnitState,
        tile_bounds: Rect,
        tint: Color,
    ) -> bool {
        self.draw_unit_pose(
            assets,
            &unit.id,
            &unit.name,
            unit.facing,
            unit.visible_animation_state(),
            tile_bounds,
            tint,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw_unit_pose(
        &self,
        assets: &AssetManager,
        unit_id: &str,
        name: &str,
        facing: UnitFacing,
        state: UnitAnimationState,
        tile_bounds: Rect,
        tint: Color,
    ) -> bool {
        let Some(definition) = self.unit_definition_for(unit_id, name) else {
            draw_missing_asset(tile_bounds, &format!("unit:{unit_id}"));
            return false;
        };
        let Some(texture) = assets.get_texture(&definition.texture) else {
            draw_missing_asset(tile_bounds, &definition.texture);
            return false;
        };
        let columns = self.unit_columns.max(1);
        let rows = self.facing_rows.max(1);
        let cell = vec2(
            texture.width() / columns as f32,
            texture.height() / rows as f32,
        );
        let facing = facing_row(facing).min(rows - 1);
        let state = animation_column(state).min(columns - 1);
        let source = Rect::new(
            state as f32 * cell.x,
            facing as f32 * cell.y,
            cell.x,
            cell.y,
        );
        let height = tile_bounds.w * definition.scale;
        let width = height * cell.x / cell.y;
        let ground = vec2(
            tile_bounds.x + tile_bounds.w * 0.5,
            tile_bounds.y + tile_bounds.h * 0.72,
        );
        draw_texture_ex(
            texture,
            ground.x - width * definition.pivot[0],
            ground.y - height * definition.pivot[1],
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                source: Some(source),
                ..Default::default()
            },
        );
        true
    }

    pub(crate) fn draw_portrait(
        &self,
        assets: &AssetManager,
        unit_id: &str,
        name: &str,
        rect: Rect,
        accent: Color,
    ) -> bool {
        let definition = self.unit_definition_for(unit_id, name);
        let Some(definition) = definition else {
            draw_missing_asset(rect, &format!("portrait:{unit_id}"));
            return false;
        };
        let Some(texture) = assets.get_texture(&definition.portrait_texture) else {
            draw_missing_asset(rect, &definition.portrait_texture);
            return false;
        };
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.018, 0.035, 0.04, 1.0),
        );
        let cell_width = texture.width() / definition.portrait_columns.max(1) as f32;
        let source = Rect::new(
            definition.portrait_index as f32 * cell_width,
            0.0,
            cell_width,
            texture.height(),
        );
        let destination = aspect_fit(source.size(), rect);
        draw_texture_ex(
            texture,
            destination.x,
            destination.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(destination.size()),
                source: Some(source),
                ..Default::default()
            },
        );
        draw_rectangle(rect.x, rect.bottom() - 5.0, rect.w, 5.0, accent);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, accent);
        true
    }

    pub(crate) fn draw_atlas_cell(
        &self,
        assets: &AssetManager,
        atlas: &AtlasDefinition,
        index: usize,
        destination: Rect,
        tint: Color,
    ) -> bool {
        let Some(texture) = assets.get_texture(&atlas.texture) else {
            draw_missing_asset(destination, &atlas.texture);
            return false;
        };
        let columns = atlas.columns.max(1);
        let rows = atlas.rows.max(1);
        let cell = vec2(
            texture.width() / columns as f32,
            texture.height() / rows as f32,
        );
        let source = Rect::new(
            (index % columns) as f32 * cell.x,
            (index / columns) as f32 * cell.y,
            cell.x,
            cell.y,
        );
        draw_texture_ex(
            texture,
            destination.x,
            destination.y,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(destination.w, destination.h)),
                source: Some(source),
                ..Default::default()
            },
        );
        true
    }
}

fn aspect_fit(source: Vec2, destination: Rect) -> Rect {
    let scale = (destination.w / source.x).min(destination.h / source.y);
    let size = source * scale;
    Rect::new(
        destination.x + (destination.w - size.x) * 0.5,
        destination.y + destination.h - size.y,
        size.x,
        size.y,
    )
}

pub(crate) fn equipment_index(id: &str) -> Option<usize> {
    [
        "frontier_rifle",
        "mire_lmg",
        "service_pistol",
        "breach_scattergun",
        "needle_carbine",
        "survey_harness",
        "chitin_plate",
        "field_medkit",
        "field_toolkit",
        "directorate_smartlink",
        "brood_living_plate",
        "ascendant_phase_lens",
    ]
    .iter()
    .position(|candidate| *candidate == id)
}

pub(crate) fn diagnostic_placeholder_image(size: u16) -> Image {
    let mut image = Image::gen_image_color(size, size, MAGENTA);
    for y in 0..size {
        for x in 0..size {
            if (x / 4 + y / 4) % 2 == 0 {
                image.set_pixel(x.into(), y.into(), BLACK);
            }
        }
    }
    image
}

pub(crate) fn draw_missing_asset(rect: Rect, id: &str) {
    let block = 8.0;
    for y in 0..=((rect.h / block) as i32) {
        for x in 0..=((rect.w / block) as i32) {
            draw_rectangle(
                rect.x + x as f32 * block,
                rect.y + y as f32 * block,
                block,
                block,
                if (x + y) % 2 == 0 { MAGENTA } else { BLACK },
            );
        }
    }
    if rect.w >= 70.0 {
        let label = format!("MISSING {id}");
        draw_text(&label, rect.x + 4.0, rect.y + 14.0, 10.0, WHITE);
    }
}

fn facing_row(facing: UnitFacing) -> usize {
    match facing {
        UnitFacing::SouthEast => 0,
        UnitFacing::SouthWest => 1,
        UnitFacing::NorthEast => 2,
        UnitFacing::NorthWest => 3,
    }
}

fn animation_column(state: UnitAnimationState) -> usize {
    match state {
        UnitAnimationState::Idle => 0,
        UnitAnimationState::Move => 1,
        UnitAnimationState::AttackAnticipation => 2,
        UnitAnimationState::AttackRelease => 3,
        UnitAnimationState::Hit => 4,
        UnitAnimationState::Incapacitated => 5,
    }
}

#[cfg(test)]
mod tests;
