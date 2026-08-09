//! Gene Lab mutation research and evolution presentation.

use crate::campaign::CampaignState;
use crate::data::GameData;
use crate::ui::{UiAction, LOGICAL_HEIGHT, LOGICAL_WIDTH};
use crate::ui_widgets::button;
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text_ex, VirtualUi};

pub(crate) fn draw_gene_lab(
    campaign: &CampaignState,
    data: &GameData,
    ui: &VirtualUi,
) -> Vec<UiAction> {
    let mouse = ui.mouse_position();
    let mut actions = Vec::new();
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.025, 0.04, 0.055, 1.0),
    );
    draw_header(campaign);
    draw_character_list(campaign, data, mouse, &mut actions);
    draw_evolution_panel(campaign, data, mouse, &mut actions);
    actions
}

fn draw_header(campaign: &CampaignState) {
    draw_surface(
        Rect::new(18.0, 16.0, LOGICAL_WIDTH - 36.0, 66.0),
        &SurfaceStyle::new(Color::new(0.045, 0.075, 0.085, 0.98))
            .with_border(1.0, Color::new(0.22, 0.62, 0.56, 0.8))
            .with_left_accent(5.0, Color::new(0.62, 0.86, 0.42, 1.0)),
    );
    draw_ui_text_ex(
        "GENE LAB",
        42.0,
        57.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        "MUTATION RESEARCH // STABILIZATION // EVOLUTION",
        230.0,
        54.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "BIOMASS {}  //  POWER {}/{}",
            campaign.colony.resources.biomass,
            campaign.colony.power_supply(),
            campaign.colony.power_demand()
        ),
        990.0,
        54.0,
        TextStyle::new(15.0, dark::TEXT_DIM).params(),
    );
}

fn draw_character_list(
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let panel = Rect::new(18.0, 96.0, 300.0, 580.0);
    draw_surface_with_title(
        panel,
        Some("COLONIST MUTATIONS"),
        &SurfaceStyle::new(Color::new(0.04, 0.06, 0.07, 0.98))
            .with_border(1.0, Color::new(0.19, 0.40, 0.40, 0.9))
            .with_header(42.0, Color::new(0.06, 0.10, 0.11, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    for (index, character) in campaign.roster.iter().enumerate() {
        let mutation = data
            .mutations
            .iter()
            .find(|mutation| mutation.id == character.mutation_id);
        let mutation_name =
            mutation.map_or(character.mutation_id.as_str(), |entry| entry.name.as_str());
        let status = if character.mutation_evolution_id.is_empty() {
            if mutation.is_some_and(|entry| entry.evolutions.is_empty()) {
                "STABLE"
            } else {
                "READY"
            }
        } else {
            "EVOLVED"
        };
        if button(
            Rect::new(36.0, 154.0 + index as f32 * 88.0, 264.0, 72.0),
            &format!(
                "{}{}\n{} // {}",
                if character.id == campaign.selected_character_id {
                    "> "
                } else {
                    ""
                },
                character.name,
                mutation_name,
                status
            ),
            true,
            mouse,
        ) {
            actions.push(UiAction::SelectColonist(character.id.clone()));
        }
    }
    draw_text_block(
        "Evolution is irreversible. Every gift carries a complication into later deployments.",
        40.0,
        574.0,
        252.0,
        62.0,
        14.0,
        4.0,
        dark::TEXT_DIM,
    );
}

fn draw_evolution_panel(
    campaign: &CampaignState,
    data: &GameData,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let panel = Rect::new(336.0, 96.0, 926.0, 580.0);
    draw_surface_with_title(
        panel,
        Some("EVOLUTION CHAMBER"),
        &SurfaceStyle::new(Color::new(0.055, 0.07, 0.08, 0.98))
            .with_border(1.0, Color::new(0.25, 0.38, 0.40, 0.8))
            .with_header(42.0, Color::new(0.08, 0.105, 0.115, 1.0)),
        TextStyle::new(17.0, dark::TEXT),
    );
    let Some(character) = campaign.selected_character() else {
        return;
    };
    let Some(mutation) = data
        .mutations
        .iter()
        .find(|mutation| mutation.id == character.mutation_id)
    else {
        return;
    };
    draw_ui_text_ex(
        &character.name,
        366.0,
        180.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!("BASE EXPRESSION // {}", mutation.name.to_uppercase()),
        366.0,
        214.0,
        TextStyle::new(15.0, dark::ACCENT).params(),
    );

    if let Some(evolution) = mutation
        .evolutions
        .iter()
        .find(|evolution| evolution.id == character.mutation_evolution_id)
    {
        draw_ui_text_ex(
            &format!("EVOLVED // {}", evolution.name.to_uppercase()),
            366.0,
            278.0,
            TextStyle::new(24.0, dark::POSITIVE).params(),
        );
        draw_text_block(
            &evolution.description,
            366.0,
            304.0,
            820.0,
            70.0,
            18.0,
            5.0,
            dark::TEXT,
        );
        draw_text_block(
            "The chamber has stabilized this expression. It cannot be replaced.",
            366.0,
            410.0,
            820.0,
            60.0,
            16.0,
            4.0,
            dark::TEXT_DIM,
        );
    } else if mutation.evolutions.is_empty() {
        draw_text_block(
            "No stable evolution path has been researched for this mutation.",
            366.0,
            278.0,
            820.0,
            80.0,
            19.0,
            5.0,
            dark::TEXT_DIM,
        );
    } else {
        draw_ui_text_ex(
            "CHOOSE ONE IRREVERSIBLE EXPRESSION",
            366.0,
            268.0,
            TextStyle::new(16.0, dark::WARNING).params(),
        );
        for (index, evolution) in mutation.evolutions.iter().enumerate() {
            let y = 290.0 + index as f32 * 128.0;
            draw_surface(
                Rect::new(366.0, y, 840.0, 108.0),
                &SurfaceStyle::new(Color::new(0.06, 0.11, 0.11, 1.0))
                    .with_border(1.0, Color::new(0.22, 0.48, 0.42, 0.9)),
            );
            draw_ui_text_ex(
                &evolution.name.to_uppercase(),
                386.0,
                y + 32.0,
                TextStyle::new(21.0, dark::TEXT_BRIGHT).params(),
            );
            draw_text_block(
                &evolution.description,
                386.0,
                y + 44.0,
                500.0,
                48.0,
                15.0,
                3.0,
                dark::TEXT_DIM,
            );
            if button(
                Rect::new(930.0, y + 30.0, 250.0, 48.0),
                &format!("EVOLVE // {} BIOMASS", evolution.biomass_cost),
                campaign.colony.resources.biomass >= evolution.biomass_cost,
                mouse,
            ) {
                actions.push(UiAction::ChooseMutationEvolution(
                    character.id.clone(),
                    evolution.id.clone(),
                ));
            }
        }
    }

    if button(
        Rect::new(1000.0, 622.0, 220.0, 40.0),
        "RETURN TO COLONY",
        true,
        mouse,
    ) {
        actions.push(UiAction::ReturnToColony);
    }
}
