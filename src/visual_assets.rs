//! Manifest-backed sprite, portrait, terrain, and colony art definitions.

use crate::data::Team;
use crate::state::UnitState;
use crate::tactical::{UnitAnimationState, UnitFacing};
use macroquad::prelude::*;
use macroquad_toolkit::assets::{AssetManager, TextureConfig};
use macroquad_toolkit::data_loader::load_embedded_json_labeled;
use serde::Deserialize;
use std::collections::BTreeSet;

pub const DEFINITIONS_JSON: &str =
    macroquad_toolkit::include_json_str!("../assets/data/sprite_definitions.json");

const REQUIRED_CONCEPT_ATLASES: [&str; 7] = [
    "passage_wreckage",
    "flora",
    "emplacements",
    "colony_props",
    "colony_machinery",
    "objectives",
    "operative_actions",
];

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasDefinition {
    #[serde(default)]
    pub id: String,
    pub texture: String,
    pub columns: usize,
    pub rows: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnitSpriteDefinition {
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
pub struct FactionChannelDefinition {
    pub id: String,
    pub accent: [f32; 4],
    pub terrain_cell: usize,
    pub effect_cell: usize,
    pub hostile_notches: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VisualCatalog {
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
    pub fn load() -> Result<Self, String> {
        let catalog: Self = load_embedded_json_labeled("sprite_definitions", DEFINITIONS_JSON)?;
        catalog.validate_registry()?;
        Ok(catalog)
    }

    pub fn validate_registry(&self) -> Result<(), String> {
        if self.unit_columns == 0 || self.facing_rows == 0 {
            return Err("sprite_definitions: unit atlas dimensions must be non-zero".to_owned());
        }
        if self
            .faction_channels
            .iter()
            .all(|channel| channel.id != "colony")
        {
            return Err(
                "sprite_definitions: required colony faction channel is missing".to_owned(),
            );
        }
        validate_unique_ids(
            "faction channel",
            self.faction_channels
                .iter()
                .map(|channel| channel.id.as_str()),
        )?;
        validate_unique_ids("unit", self.units.iter().map(|unit| unit.id.as_str()))?;
        validate_atlas("terrain", &self.terrain)?;
        validate_atlas("colony", &self.colony)?;
        validate_atlas("equipment", &self.equipment)?;
        validate_atlas("effects", &self.effects)?;
        validate_unique_ids(
            "concept atlas",
            self.concepts.iter().map(|atlas| atlas.id.as_str()),
        )?;
        for atlas in &self.concepts {
            validate_atlas("concept", atlas)?;
        }
        for required_id in REQUIRED_CONCEPT_ATLASES {
            if self.concept_atlas(required_id).is_none() {
                return Err(format!(
                    "sprite_definitions: required concept atlas '{}' is missing",
                    required_id
                ));
            }
        }
        for channel in &self.faction_channels {
            if channel.terrain_cell >= self.terrain.columns * self.terrain.rows {
                return Err(format!(
                    "sprite_definitions: faction '{}' terrain cell {} is outside the terrain atlas",
                    channel.id, channel.terrain_cell
                ));
            }
            if channel.effect_cell >= self.effects.columns * self.effects.rows {
                return Err(format!(
                    "sprite_definitions: faction '{}' effect cell {} is outside the effects atlas",
                    channel.id, channel.effect_cell
                ));
            }
        }
        for unit in &self.units {
            if unit.texture.is_empty() || unit.portrait_texture.is_empty() {
                return Err(format!(
                    "sprite_definitions: unit '{}' must declare tactical and portrait textures",
                    unit.id
                ));
            }
            if unit.portrait_columns == 0 || unit.scale <= 0.0 {
                return Err(format!(
                    "sprite_definitions: unit '{}' has invalid portrait columns or scale",
                    unit.id
                ));
            }
            if unit
                .pivot
                .iter()
                .any(|component| !(0.0..=1.0).contains(component))
            {
                return Err(format!(
                    "sprite_definitions: unit '{}' pivot must stay within 0..=1",
                    unit.id
                ));
            }
        }
        Ok(())
    }

    pub fn unit_definition_for(&self, unit_id: &str, name: &str) -> Option<&UnitSpriteDefinition> {
        if let Some(definition) = self
            .units
            .iter()
            .find(|definition| definition.id == unit_id)
        {
            return Some(definition);
        }
        let lowercase_name = name.to_ascii_lowercase();
        if let Some(definition) = self.units.iter().find(|definition| {
            definition
                .name_contains
                .iter()
                .any(|needle| lowercase_name.contains(needle))
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

    pub fn faction_channel(&self, faction: &str) -> &FactionChannelDefinition {
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

    pub fn unit_faction_channel(&self, unit: &UnitState) -> &FactionChannelDefinition {
        if unit.team == Team::Colony {
            self.faction_channel("colony")
        } else {
            self.faction_channel(unit.faction.as_deref().unwrap_or("brood"))
        }
    }

    pub fn faction_accent(&self, unit: &UnitState) -> Color {
        let [r, g, b, a] = self.unit_faction_channel(unit).accent;
        Color::new(r, g, b, a)
    }

    pub fn faction_terrain_cell(&self, faction: &str) -> usize {
        self.faction_channel(faction).terrain_cell
    }

    pub fn faction_effect_cell(&self, faction: &str) -> usize {
        self.faction_channel(faction).effect_cell
    }

    pub fn concept_atlas(&self, id: &str) -> Option<&AtlasDefinition> {
        self.concepts.iter().find(|atlas| atlas.id == id)
    }

    pub fn validate_texture_manifest(&self, manifest: &[TextureConfig]) -> Result<(), String> {
        let manifest_keys = manifest
            .iter()
            .map(|texture| texture.key.as_str())
            .collect::<BTreeSet<_>>();
        let mut required_keys = Vec::new();
        required_keys.extend(
            self.units
                .iter()
                .flat_map(|unit| [&unit.texture, &unit.portrait_texture]),
        );
        required_keys.extend([
            &self.terrain.texture,
            &self.colony.texture,
            &self.equipment.texture,
            &self.effects.texture,
        ]);
        required_keys.extend(self.concepts.iter().map(|atlas| &atlas.texture));
        let missing = required_keys
            .into_iter()
            .filter(|key| !manifest_keys.contains(key.as_str()))
            .collect::<BTreeSet<_>>();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "sprite_definitions: texture manifest is missing {}",
                missing.into_iter().cloned().collect::<Vec<_>>().join(", ")
            ))
        }
    }

    pub fn validate_loaded(&self, assets: &AssetManager) -> Vec<String> {
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

    pub fn diagnostic_summary(&self, assets: &AssetManager) -> String {
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

    pub fn draw_unit_sprite(
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

    // A sprite pose is a pure render request; explicit geometry and tint
    // inputs prevent hidden asset or camera state.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_unit_pose(
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
        let facing_row_index = facing_row(facing).min(rows - 1);
        let animation_column_index = animation_column(state).min(columns - 1);
        let source = Rect::new(
            animation_column_index as f32 * cell.x,
            facing_row_index as f32 * cell.y,
            cell.x,
            cell.y,
        );
        // The source atlases are authored by different passes, so a small
        // amount of scale normalization keeps silhouettes from jumping in
        // size when a roster mixes humans, drones, and larger creatures.
        let height = tile_bounds.w * definition.scale.clamp(1.30, 1.42);
        let width = height * cell.x / cell.y;
        let ground = vec2(
            tile_bounds.x + tile_bounds.w * 0.5,
            tile_bounds.y + tile_bounds.h * 0.72,
        );
        draw_sprite_shadow(ground, width, height);
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

    pub fn draw_portrait(
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

    pub fn draw_atlas_cell(
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

fn validate_unique_ids<'a>(label: &str, ids: impl Iterator<Item = &'a str>) -> Result<(), String> {
    let mut seen = std::collections::BTreeSet::new();
    for id in ids {
        if id.is_empty() || !seen.insert(id) {
            return Err(format!(
                "sprite_definitions: {label} id '{id}' is empty or duplicated"
            ));
        }
    }
    Ok(())
}

fn validate_atlas(label: &str, atlas: &AtlasDefinition) -> Result<(), String> {
    if atlas.texture.is_empty() || atlas.columns == 0 || atlas.rows == 0 {
        return Err(format!(
            "sprite_definitions: {label} atlas '{}' has an empty texture or zero dimensions",
            atlas.id
        ));
    }
    Ok(())
}

pub fn draw_sprite_shadow(ground: Vec2, width: f32, height: f32) {
    draw_ellipse(
        ground.x + width * 0.04,
        ground.y + height * 0.025,
        (width * 0.23).clamp(4.0, 15.0),
        (height * 0.045).clamp(1.5, 4.0),
        0.0,
        Color::new(0.0, 0.01, 0.012, 0.48),
    );
}

pub fn aspect_fit(source: Vec2, destination: Rect) -> Rect {
    let scale = (destination.w / source.x).min(destination.h / source.y);
    let size = source * scale;
    Rect::new(
        destination.x + (destination.w - size.x) * 0.5,
        destination.y + destination.h - size.y,
        size.x,
        size.y,
    )
}

pub fn equipment_index(id: &str) -> Option<usize> {
    if id == "directorate_cipher" {
        return Some(9);
    }
    if id == "mireborn_sense" {
        return Some(5);
    }
    if id == "severed_resonance" {
        return Some(10);
    }
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

pub fn diagnostic_placeholder_image(size: u16) -> Image {
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

pub fn draw_missing_asset(rect: Rect, id: &str) {
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

pub fn facing_row(facing: UnitFacing) -> usize {
    match facing {
        UnitFacing::SouthEast => 0,
        UnitFacing::SouthWest => 1,
        UnitFacing::NorthEast => 2,
        UnitFacing::NorthWest => 3,
    }
}

pub fn animation_column(state: UnitAnimationState) -> usize {
    match state {
        UnitAnimationState::Idle => 0,
        UnitAnimationState::Move => 1,
        UnitAnimationState::AttackAnticipation => 2,
        UnitAnimationState::AttackRelease => 3,
        UnitAnimationState::Hit => 4,
        UnitAnimationState::Incapacitated => 5,
    }
}
