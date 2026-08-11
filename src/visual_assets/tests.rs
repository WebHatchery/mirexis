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
        serde_json::from_str(include_str!("../../assets/data/texture_manifest.json"))
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
        let bytes =
            std::fs::read(relative).unwrap_or_else(|err| panic!("cannot read {relative}: {err}"));
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
