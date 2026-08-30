use std::collections::BTreeSet;

fn capture_manifest() -> Vec<&'static str> {
    let script = include_str!("../../../scripts/capture_ui.ps1");
    let manifest_line = script
        .lines()
        .find(|line| line.contains("[string[]]$Scenes"))
        .expect("capture script declares its scene manifest");
    manifest_line.split('"').skip(1).step_by(2).collect()
}

#[test]
fn capture_script_manifest_is_unique_and_every_scene_has_an_explicit_builder() {
    let source = include_str!("../capture_scenes.rs");
    let scenes = capture_manifest();
    assert_eq!(scenes.len(), 87, "update the audited capture inventory");
    assert_eq!(
        scenes.iter().copied().collect::<BTreeSet<_>>().len(),
        scenes.len(),
        "capture manifest contains duplicate scene names"
    );
    for scene in scenes {
        assert!(
            source.contains(&format!("\"{scene}\" =>")),
            "capture scene '{scene}' falls through to the tactical default"
        );
    }
}

#[test]
fn acceptance_documents_cover_the_exact_capture_manifest() {
    let matrix = include_str!("../../../docs/verification/UI_GRAPHICS_ACCEPTANCE_MATRIX.md");
    let documented = matrix
        .lines()
        .filter_map(|line| {
            let marker = "`ui_";
            let start = line.find(marker)? + marker.len();
            let end = line[start..].find(".png`")? + start;
            Some(&line[start..end])
        })
        .collect::<BTreeSet<_>>();
    let manifest = capture_manifest().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(documented, manifest, "capture matrix and harness drifted");

    let inventory = include_str!("../../../docs/verification/FINAL_CAPTURE_INVENTORY.md");
    let inventoried = inventory
        .lines()
        .filter_map(|line| {
            let marker = "| `ui_";
            let start = line.find(marker)? + marker.len();
            let end = line[start..].find(".png`")? + start;
            Some(&line[start..end])
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        inventoried, manifest,
        "capture inventory and harness drifted"
    );
}
