<#
.SYNOPSIS
    Headless screenshot harness for Mirexis.

.DESCRIPTION
    Thin wrapper around the shared macroquad-toolkit capture script. Builds the
    debug exe and drives it through the env-var capture hook
    (MIREXIS_CAPTURE_*) provided by macroquad_toolkit::capture in src/main.rs.

.EXAMPLE
    ./scripts/capture_ui.ps1
    ./scripts/capture_ui.ps1 -Frames 60 -SkipBuild
#>
param(
    [string[]]$Scenes = @("title", "colony", "contact", "contact_gear", "contact_event", "adaptation", "gene_lab", "evolution", "mara_evolution", "adaptation_operation", "glass_nerve", "damage", "power", "construction", "research", "roster", "legacy", "briefing", "pressure", "gameplay", "extraction", "variant", "sporefield", "vault", "black_channel", "living_chorus", "open_circuit", "trace_active", "equipment", "class_target", "breach", "debrief"),
    [int]$Frames = 150,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$shared = Join-Path (Split-Path -Parent $gameDir) "macroquad-toolkit\scripts\capture_ui.ps1"

& $shared -GameDir $gameDir -Prefix "MIREXIS" -Scenes $Scenes -Frames $Frames -OutputDir $OutputDir -SkipBuild:$SkipBuild
