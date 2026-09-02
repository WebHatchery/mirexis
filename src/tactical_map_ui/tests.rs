use super::*;
use crate::first_hour::{FirstHourProgress, FirstHourStage, TacticalLesson};
use crate::grid_ui::{STRUCTURE_ART_PIVOT, STRUCTURE_ART_SCALE};
use crate::state::GameSession;

#[test]
fn cancel_targeting_control_is_visible_without_covering_the_target_portrait() {
    let card = super::targeting_card::card_bounds(tactical_panel());
    let cancel = super::targeting_card::cancel_bounds(card);
    let target_portrait = Rect::new(card.right() - 72.0, card.y + 8.0, 64.0, 64.0);
    assert!(cancel.x >= card.x);
    assert!(cancel.right() <= card.right());
    assert!(cancel.y >= card.y);
    assert!(cancel.bottom() <= card.bottom());
    assert!(cancel.right() <= target_portrait.x);
}

#[test]
fn guided_attack_selects_before_it_commits_the_shot() {
    let data = crate::data::GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    let attacker = session.selected_unit().unwrap().position;
    let hostile = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == crate::data::Team::Hostile)
        .unwrap();
    hostile.position = TilePos::new(attacker.x + 4, attacker.y);
    let hostile_id = hostile.id.clone();
    let hostile_tile = hostile.position;
    let progress = FirstHourProgress {
        stage: FirstHourStage::FirstOperation,
        lesson: TacticalLesson::Attack,
        ..FirstHourProgress::default()
    };

    assert_eq!(
        normal_tile_action(&session, &progress, hostile_tile),
        UiAction::SelectTile(hostile_tile)
    );
    session.select_tile(hostile_tile);
    assert_eq!(
        normal_tile_action(&session, &progress, hostile_tile),
        UiAction::AttackSelected(hostile_id)
    );
}

#[test]
fn unguided_attack_keeps_the_direct_hostile_intent() {
    let data = crate::data::GameData::load().unwrap();
    let mut session = GameSession::new(&data.config, &data.mission, &data.roster);
    let attacker = session.selected_unit().unwrap().position;
    let hostile = session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.team == crate::data::Team::Hostile)
        .unwrap();
    hostile.position = TilePos::new(attacker.x + 4, attacker.y);
    let hostile_id = hostile.id.clone();
    let hostile_tile = hostile.position;

    assert_eq!(
        normal_tile_action(
            &session,
            &FirstHourProgress {
                guidance_enabled: false,
                ..FirstHourProgress::default()
            },
            hostile_tile,
        ),
        UiAction::AttackSelected(hostile_id)
    );
}

#[test]
fn invalid_map_target_does_not_cancel_targeting() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = crate::campaign::CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let session = GameSession::new(&data.config, &data.mission, &roster);
    let empty_tile = session
        .tactical
        .fog
        .iter_with_pos()
        .map(|(tile, _)| tile)
        .find(|tile| {
            !session
                .tactical
                .units
                .iter()
                .any(|unit| unit.position == *tile)
        })
        .unwrap();

    assert_eq!(
        targeting_action(
            &session,
            TargetingView::Equipment {
                unit_id: "ilya_reed",
                equipment_id: "field_medkit",
            },
            empty_tile,
        ),
        None
    );
}

#[test]
fn valid_map_target_keeps_the_equipment_intent() {
    let data = crate::data::GameData::load().unwrap();
    let campaign = crate::campaign::CampaignState::new(&data);
    let roster = campaign.deployment_roster(&data, &data.mission);
    let mut session = GameSession::new(&data.config, &data.mission, &roster);
    let kira_position = session.unit("kira_voss").unwrap().position;
    session
        .tactical
        .units
        .iter_mut()
        .find(|unit| unit.id == "brood_stalker_a")
        .unwrap()
        .position = TilePos::new(kira_position.x + 1, kira_position.y);
    let target_tile = session.unit("brood_stalker_a").unwrap().position;

    assert_eq!(
        targeting_action(
            &session,
            TargetingView::Equipment {
                unit_id: "kira_voss",
                equipment_id: "survey_harness",
            },
            target_tile,
        ),
        Some(UiAction::UseEquipmentOn("brood_stalker_a".to_owned()))
    );
}

#[test]
fn attack_confirmation_card_stays_inside_the_tactical_panel() {
    let panel = tactical_panel();
    let card = crate::action_preview_ui::attack_card_bounds(panel);
    assert!(card.x >= panel.x);
    assert!(card.right() <= panel.right());
    assert!(card.y >= panel.y);
    assert!(card.bottom() <= panel.bottom());
}

#[test]
fn tactical_clamp_exposes_complete_boundary_art() {
    let viewport = Rect::new(18.0, 106.0, 884.0, 568.0);
    let cases = [
        (vec2(-10_000.0, 468.0), TilePos::new(0, 39), 0),
        (vec2(10_000.0, 468.0), TilePos::new(39, 0), 1),
        (vec2(0.0, -10_000.0), TilePos::new(0, 0), 2),
        (vec2(0.0, 10_000.0), TilePos::new(39, 39), 3),
    ];
    for zoom in [0.65, 1.0, 1.85] {
        for (requested_center, position, edge) in cases {
            let mut camera = WorldCamera::tactical_start(TilePos::new(4, 19));
            camera.zoom = zoom;
            camera.center = requested_center;
            camera.clamp_isometric_with_insets(
                40,
                40,
                crate::grid_ui::TACTICAL_HALF_WIDTH,
                crate::grid_ui::TACTICAL_HALF_HEIGHT,
                viewport,
                camera_art_insets(zoom),
            );
            let view = GridView::with_camera(40, 40, viewport, &camera);
            let bounds = match edge {
                0 | 1 => view.art_bounds(position, CANOPY_ART_SCALE, CANOPY_ART_PIVOT),
                2 => {
                    let tile = view.tile_rect(position);
                    let height = tile.w * 1.5;
                    let width = height * (2.0 / 3.0);
                    let ground = vec2(tile.center().x, tile.y + tile.h * 0.72);
                    Rect::new(
                        ground.x - width * 0.5,
                        ground.y - height * 0.82,
                        width,
                        height,
                    )
                }
                _ => view.art_bounds(position, TERRAIN_ART_SCALE, TERRAIN_ART_PIVOT),
            };
            match edge {
                0 => assert!(bounds.x >= viewport.x - 0.01),
                1 => assert!(bounds.right() <= viewport.right() + 0.01),
                2 => assert!(bounds.y >= viewport.y - 0.01),
                _ => assert!(bounds.bottom() <= viewport.bottom() + 0.01),
            }
        }
    }
}

#[test]
fn tactical_camera_insets_cover_every_declared_static_sprite() {
    let insets = camera_art_insets(1.0);
    let tile_width = crate::grid_ui::TACTICAL_HALF_WIDTH * 2.0;
    let ground_y =
        -crate::grid_ui::TACTICAL_HALF_HEIGHT + crate::grid_ui::TACTICAL_HALF_HEIGHT * 2.0 * 0.72;
    for definition in crate::visual_assets::VisualCatalog::load().units {
        let height = tile_width * definition.scale;
        let width = height * (2.0 / 3.0);
        let left = width * definition.pivot[0] - crate::grid_ui::TACTICAL_HALF_WIDTH;
        let right = width * (1.0 - definition.pivot[0]) - crate::grid_ui::TACTICAL_HALF_WIDTH;
        let top = height * definition.pivot[1] - (ground_y + crate::grid_ui::TACTICAL_HALF_HEIGHT);
        let bottom =
            ground_y + height * (1.0 - definition.pivot[1]) - crate::grid_ui::TACTICAL_HALF_HEIGHT;
        assert!(left <= insets.left, "{} exceeds left inset", definition.id);
        assert!(
            right <= insets.right,
            "{} exceeds right inset",
            definition.id
        );
        assert!(top <= insets.top, "{} exceeds top inset", definition.id);
        assert!(
            bottom <= insets.bottom,
            "{} exceeds bottom inset",
            definition.id
        );
    }
    for (scale, pivot) in [
        (TERRAIN_ART_SCALE, TERRAIN_ART_PIVOT),
        (STRUCTURE_ART_SCALE, STRUCTURE_ART_PIVOT),
        (CANOPY_ART_SCALE, CANOPY_ART_PIVOT),
    ] {
        let width = tile_width * scale;
        let height = width * crate::grid_ui::TERRAIN_ART_ASPECT;
        assert!(width * pivot[0] - tile_width * 0.5 <= insets.left);
        assert!(width * (1.0 - pivot[0]) - tile_width * 0.5 <= insets.right);
        assert!(height * pivot[1] - crate::grid_ui::TACTICAL_HALF_HEIGHT <= insets.top);
        assert!(height * (1.0 - pivot[1]) - crate::grid_ui::TACTICAL_HALF_HEIGHT <= insets.bottom);
    }
}
