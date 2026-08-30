<#
.SYNOPSIS
    Single-window background screenshot harness for Mirexis.

.DESCRIPTION
    Thin wrapper around the shared macroquad-toolkit capture script. Builds the
    debug exe once and drives every scene through the manifest capture hook
    (MIREXIS_CAPTURE_*) provided by macroquad_toolkit::capture in src/main.rs.

.EXAMPLE
    ./scripts/capture_ui.ps1
    ./scripts/capture_ui.ps1 -Frames 60 -SkipBuild
#>
param(
    [string[]]$Scenes = @("title", "title_controller", "title_hover", "colony", "contact", "contact_gear", "contact_event", "adaptation", "gene_lab", "evolution", "mara_evolution", "ilya_evolution", "sol_evolution", "nadi_evolution", "escalation", "escalation_operation", "escalation_response", "mirexis", "mirexis_path", "redoubt_end", "commonwealth_end", "threshold_end", "finale_debrief", "adaptation_operation", "glass_nerve", "three_knives", "reinforcement_warning", "line_formation", "thin_shelter", "breakwater", "false_heart", "live_wire", "last_wall", "root_choir", "door_of_light", "damage", "repair", "power", "construction", "research", "roster", "advanced_roster", "relationships", "trauma", "bonded_briefing", "legacy", "briefing", "threat_briefing", "loadout_briefing", "pressure", "gameplay", "first_hour_tactical", "overwatch", "brood_ability", "directorate_ability", "ascendant_ability", "hazard", "intent", "action_preview", "movement_route", "cover_edges", "invalid_command", "valid_shot", "threat_range", "danger_reach", "help", "first_hour_guide", "battle_log", "combat_feedback", "phase_replay", "end_phase_guard", "readiness_markers", "vitality_markers", "extraction", "variant", "sporefield", "vault", "black_channel", "living_chorus", "open_circuit", "trace_active", "equipment", "weapon_profile", "class_target", "breach", "debrief", "trauma_debrief"),
    [int]$Frames = 150,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$shared = Join-Path (Split-Path -Parent $gameDir) "macroquad-toolkit\scripts\capture_ui.ps1"

& $shared -GameDir $gameDir -Prefix "MIREXIS" -Scenes $Scenes -Frames $Frames -OutputDir $OutputDir -SkipBuild:$SkipBuild
