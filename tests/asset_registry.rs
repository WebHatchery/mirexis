use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct AssetRegistry {
    version: u32,
    assets: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TextureManifestEntry {
    path: String,
}

#[test]
fn registry_matches_external_texture_manifest_assets() {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let registry: AssetRegistry = macroquad_toolkit::data_loader::load_json_file_sync(
        project_root.join("asset_registry.json"),
    )
    .expect("asset registry exists and is valid JSON");
    assert_eq!(registry.version, 1, "asset registry version is supported");

    let manifest: Vec<TextureManifestEntry> = macroquad_toolkit::data_loader::load_json_file_sync(
        project_root.join("assets/data/texture_manifest.json"),
    )
    .expect("texture manifest exists and is valid JSON");

    let registered = registry.assets.iter().cloned().collect::<BTreeSet<_>>();
    let runtime = manifest
        .into_iter()
        .map(|entry| entry.path)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        registered, runtime,
        "registry and runtime texture paths differ"
    );

    for path in registered {
        let file = project_root.join(&path);
        assert!(
            file.is_file(),
            "registered runtime asset is missing: {path}"
        );
    }
}
