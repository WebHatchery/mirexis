//! Manifest-backed sprite, portrait, terrain, and colony art definitions.

use crate::data::Team;
use crate::state::UnitState;
use crate::tactical::{UnitAnimationState, UnitFacing};
use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use serde::Deserialize;

const DEFINITIONS_JSON: &str = include_str!("../assets/data/sprite_definitions.json");

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AtlasDefinition {
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
            "{} unit definitions // {} // {} // {} // {}",
            self.units.len(),
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
        draw_texture_ex(
            texture,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rect.w, rect.h)),
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
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};
    use std::hash::{DefaultHasher, Hash, Hasher};

    #[test]
    fn production_catalog_has_every_required_asset_family() {
        let catalog = VisualCatalog::load();
        assert_eq!(catalog.unit_columns, 6);
        assert_eq!(catalog.facing_rows, 4);
        assert_eq!(catalog.units.len(), 11);
        assert_eq!(catalog.terrain.columns * catalog.terrain.rows, 12);
        assert_eq!(catalog.colony.columns * catalog.colony.rows, 16);
        assert_eq!(catalog.equipment.columns * catalog.equipment.rows, 12);
        assert_eq!(catalog.effects.columns * catalog.effects.rows, 8);
        let data = crate::data::GameData::load().expect("embedded game data loads");
        for equipment in &data.equipment {
            assert!(
                equipment_index(&equipment.id).is_some(),
                "{} has no equipment atlas cell",
                equipment.id
            );
        }
    }

    #[test]
    fn faction_channels_are_complete_distinct_and_within_their_atlases() {
        let catalog = VisualCatalog::load();
        let channels = catalog
            .faction_channels
            .iter()
            .map(|channel| channel.id.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            channels,
            BTreeSet::from(["ascendant", "brood", "colony", "directorate"])
        );
        assert_eq!(
            catalog
                .faction_channels
                .iter()
                .map(|channel| channel.terrain_cell)
                .collect::<BTreeSet<_>>()
                .len(),
            catalog.faction_channels.len()
        );
        assert_eq!(
            catalog
                .faction_channels
                .iter()
                .map(|channel| channel.effect_cell)
                .collect::<BTreeSet<_>>()
                .len(),
            catalog.faction_channels.len()
        );
        for channel in &catalog.faction_channels {
            assert!(channel.terrain_cell < catalog.terrain.columns * catalog.terrain.rows);
            assert!(channel.effect_cell < catalog.effects.columns * catalog.effects.rows);
            assert!(channel.accent[..3].iter().any(|component| *component > 0.5));
            assert_eq!(channel.hostile_notches, channel.id != "colony");
        }
    }

    #[test]
    fn every_recruit_and_hostile_record_resolves_to_authored_unit_and_portrait_art() {
        let catalog = VisualCatalog::load();
        let data = crate::data::GameData::load().expect("embedded game data loads");
        for unit in &data.roster {
            let definition = catalog
                .unit_definition_for(&unit.id, &unit.name)
                .unwrap_or_else(|| panic!("{} ({}) has no authored art", unit.name, unit.id));
            assert!(
                !definition.texture.is_empty(),
                "{} tactical texture",
                unit.id
            );
            assert!(
                !definition.portrait_texture.is_empty(),
                "{} portrait texture",
                unit.id
            );
        }
    }

    #[test]
    fn every_declared_atlas_decodes_and_has_art_in_every_cell() {
        let catalog = VisualCatalog::load();
        let manifest: Vec<serde_json::Value> =
            serde_json::from_str(include_str!("../assets/data/texture_manifest.json"))
                .expect("texture manifest parses");
        let paths = manifest
            .iter()
            .map(|entry| {
                (
                    entry["key"].as_str().expect("texture key").to_owned(),
                    entry["path"].as_str().expect("texture path").to_owned(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let mut grids = BTreeMap::new();
        for unit in &catalog.units {
            grids.insert(
                unit.texture.clone(),
                (catalog.unit_columns, catalog.facing_rows),
            );
            grids.insert(unit.portrait_texture.clone(), (unit.portrait_columns, 1));
        }
        for atlas in [
            &catalog.terrain,
            &catalog.colony,
            &catalog.equipment,
            &catalog.effects,
        ] {
            grids.insert(atlas.texture.clone(), (atlas.columns, atlas.rows));
        }

        for (key, (columns, rows)) in grids {
            let relative = paths
                .get(&key)
                .unwrap_or_else(|| panic!("{key} is absent from texture manifest"));
            let bytes = std::fs::read(relative)
                .unwrap_or_else(|err| panic!("cannot read {relative}: {err}"));
            let image = Image::from_file_with_format(&bytes, None)
                .unwrap_or_else(|err| panic!("cannot decode {relative}: {err}"));
            assert_eq!(image.width as usize % columns, 0, "{relative} width");
            assert_eq!(image.height as usize % rows, 0, "{relative} height");
            assert_cells_have_alpha(&image, columns, rows, relative);
            assert_cells_are_distinct(&image, columns, rows, relative);
        }
    }

    fn assert_cells_have_alpha(image: &Image, columns: usize, rows: usize, label: &str) {
        let width = image.width as usize;
        let cell_width = width / columns;
        let cell_height = image.height as usize / rows;
        for row in 0..rows {
            for column in 0..columns {
                let mut visible = 0usize;
                for y in row * cell_height..(row + 1) * cell_height {
                    for x in column * cell_width..(column + 1) * cell_width {
                        visible += (image.bytes[(y * width + x) * 4 + 3] > 12) as usize;
                    }
                }
                let coverage = visible as f32 / (cell_width * cell_height) as f32;
                assert!(
                    coverage >= 0.03,
                    "{label} cell ({column}, {row}) has only {coverage:.1}% alpha coverage"
                );
            }
        }
    }

    fn assert_cells_are_distinct(image: &Image, columns: usize, rows: usize, label: &str) {
        let width = image.width as usize;
        let cell_width = width / columns;
        let cell_height = image.height as usize / rows;
        let mut checksums = BTreeSet::new();
        for row in 0..rows {
            for column in 0..columns {
                let mut hasher = DefaultHasher::new();
                for y in row * cell_height..(row + 1) * cell_height {
                    let start = (y * width + column * cell_width) * 4;
                    let end = start + cell_width * 4;
                    image.bytes[start..end].hash(&mut hasher);
                }
                assert!(
                    checksums.insert(hasher.finish()),
                    "{label} cell ({column}, {row}) exactly duplicates another atlas cell"
                );
            }
        }
    }
}
