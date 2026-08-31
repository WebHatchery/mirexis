<#
.SYNOPSIS
    Audits Mirexis's canonical UI captures and refreshes their integrity ledger.

.DESCRIPTION
    Reads the scene names from capture_ui.ps1, verifies every root-level PNG,
    checks basic visual-health signals, then rewrites capture_audit.json and
    FINAL_CAPTURE_INVENTORY.md in manifest order.
#>
param(
    [string]$CaptureRoot = "docs\verification"
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$captureScript = Join-Path $PSScriptRoot "capture_ui.ps1"
$captureDir = Join-Path $gameDir $CaptureRoot

$source = Get-Content $captureScript -Raw
$sceneBlock = [regex]::Match(
    $source,
    '\[string\[\]\]\$Scenes\s*=\s*@\((?<scenes>.*?)\),\s*\r?\n\s*\[int\]\$Frames',
    [System.Text.RegularExpressions.RegexOptions]::Singleline
)
if (-not $sceneBlock.Success) {
    throw "Could not read the scene manifest from $captureScript"
}
$scenes = [regex]::Matches($sceneBlock.Groups['scenes'].Value, '"(?<scene>[^"]+)"') |
    ForEach-Object { $_.Groups['scene'].Value }

Add-Type -AssemblyName System.Drawing
$expectedNames = @($scenes | ForEach-Object { "ui_$_.png" })
$actualFiles = @(Get-ChildItem $captureDir -Filter "ui_*.png" -File)
$actualNames = @($actualFiles.Name)
$missing = @($expectedNames | Where-Object { $_ -notin $actualNames })
$extra = @($actualNames | Where-Object { $_ -notin $expectedNames })
$records = [System.Collections.Generic.List[object]]::new()
$issues = [System.Collections.Generic.List[string]]::new()

foreach ($scene in $scenes) {
    $name = "ui_$scene.png"
    $path = Join-Path $captureDir $name
    if (-not (Test-Path $path)) {
        continue
    }

    $bitmap = [System.Drawing.Bitmap]::new($path)
    try {
        $unique = [System.Collections.Generic.HashSet[int]]::new()
        $magenta = 0
        $lumaMin = 255.0
        $lumaMax = 0.0
        $lumaTotal = 0.0
        $samples = 0
        for ($y = 0; $y -lt $bitmap.Height; $y += 8) {
            for ($x = 0; $x -lt $bitmap.Width; $x += 8) {
                $pixel = $bitmap.GetPixel($x, $y)
                [void]$unique.Add(($pixel.R -shl 16) -bor ($pixel.G -shl 8) -bor $pixel.B)
                if ($pixel.R -eq 255 -and $pixel.G -eq 0 -and $pixel.B -eq 255) {
                    $magenta++
                }
                $luma = 0.2126 * $pixel.R + 0.7152 * $pixel.G + 0.0722 * $pixel.B
                $lumaMin = [Math]::Min($lumaMin, $luma)
                $lumaMax = [Math]::Max($lumaMax, $luma)
                $lumaTotal += $luma
                $samples++
            }
        }

        $wrongSize = $bitmap.Width -ne 1280 -or $bitmap.Height -ne 720
        $diagnostic = $magenta -gt 32
        $lowDetail = $unique.Count -lt 64 -or ($lumaMax - $lumaMin) -lt 20 -or
            ($samples -gt 0 -and ($lumaTotal / $samples) -lt 5)
        if ($wrongSize) { $issues.Add("${name}: expected 1280x720, found $($bitmap.Width)x$($bitmap.Height)") }
        if ($diagnostic) { $issues.Add("${name}: contains $magenta sampled diagnostic-magenta pixels") }
        if ($lowDetail) { $issues.Add("${name}: blank/dark/low-detail heuristic failed") }

        $file = Get-Item $path
        $records.Add([pscustomobject]@{
            Scene = $scene
            Name = $name
            Bytes = $file.Length
            Sha256 = (Get-FileHash $path -Algorithm SHA256).Hash.ToLowerInvariant()
            WrongSize = $wrongSize
            Diagnostic = $diagnostic
            LowDetail = $lowDetail
        })
    }
    finally {
        $bitmap.Dispose()
    }
}

foreach ($name in $missing) { $issues.Add("missing: $name") }
foreach ($name in $extra) { $issues.Add("unexpected: $name") }
$duplicateGroups = @($records | Group-Object Sha256 | Where-Object Count -gt 1)
foreach ($group in $duplicateGroups) {
    $issues.Add("exact duplicate: $($group.Group.Name -join ', ')")
}

$audit = [ordered]@{
    generated = (Get-Date -Format "yyyy-MM-dd")
    capture_root = "docs/verification"
    target_dimensions = @(1280, 720)
    manifest_scenes = $scenes.Count
    files_audited = $records.Count
    unique_sha256 = @($records.Sha256 | Sort-Object -Unique).Count
    missing_files = $missing.Count
    extra_files = $extra.Count
    wrong_dimensions = @($records | Where-Object WrongSize).Count
    exact_duplicates = $duplicateGroups.Count
    magenta_diagnostic_frames = @($records | Where-Object Diagnostic).Count
    blank_or_low_detail_frames = @($records | Where-Object LowDetail).Count
    issues = @($issues)
}
$audit | ConvertTo-Json -Depth 4 | Set-Content (Join-Path $captureDir "capture_audit.json") -Encoding utf8

$lines = [System.Collections.Generic.List[string]]::new()
$lines.Add("# Mirexis Final Capture Inventory")
$lines.Add("")
$lines.Add("Generated: $(Get-Date -Format 'yyyy-MM-dd')")
$lines.Add('Capture root: `docs/verification`')
$lines.Add('Manifest: `scripts/capture_ui.ps1`')
$lines.Add("")
$lines.Add("## Integrity summary")
$lines.Add("")
$lines.Add("- $($scenes.Count) manifest scenes and $($records.Count) canonical root PNG files; no scene is stored in a capture subfolder.")
$lines.Add("- Every capture uses the standard 1280 x 720 logical target.")
$lines.Add("- The harness overwrites each canonical ``ui_<scene>.png`` file in place.")
$lines.Add("- Tactical inspection covers default gameplay, movement routes, class targeting, valid shots, hazards, and the off-center zoom variant.")
$lines.Add("- Colony inspection covers the default settlement, construction placement, damage, repair, and power loss.")
$lines.Add("- ``capture_audit.json`` records automated size, duplicate, diagnostic-color, and visual-detail checks.")
$lines.Add("- The acceptance-document test fails if the manifest, screen matrix, and this inventory drift apart.")
$lines.Add("")
$lines.Add("## Spatial acceptance evidence")
$lines.Add("")
$lines.Add("- ``ui_gameplay.png``: default tactical camera with a cropped 40 x 40 battlefield and readable elevation.")
$lines.Add("- ``ui_variant.png``: off-center tactical camera with selection, objective, hazard, unit, and contextual overlays intact.")
$lines.Add("- ``ui_movement_route.png`` and ``ui_valid_shot.png``: transformed route, target overlays, and the visible ATTACK confirmation remain aligned.")
$lines.Add("- ``ui_class_target.png``: transformed unit targeting and contextual card remain aligned.")
$lines.Add("- ``ui_colony.png``: centered settlement with a visible first-hour coordinator route and undeveloped 20 x 20 frontier.")
$lines.Add("- ``ui_briefing.png``: first-operation deployment guidance focuses DEPLOY SQUAD while preserving the selectable roster rows.")
$lines.Add("- ``ui_first_hour_tactical.png``: guided tactical lesson marks a legal move target without blocking the battlefield inspection.")
$lines.Add("- ``ui_first_hour_ability.png``: guided ability lesson focuses a currently actionable mutation, class action, or field item.")
$lines.Add("- ``ui_first_hour_enemy_phase.png``: enemy-phase guidance pairs the focused END PHASE control with READY, SPENT, and INCAP counts.")
$lines.Add("- ``ui_second_operation_tactical.png``: the persisted transfer lesson keeps the active preparation, ApplyLearning hostile focus, and forecast prompt visible after deployment.")
$lines.Add("- ``ui_first_hour_return.png``: the first-return recovery handoff routes the player to Ilya's highlighted speech marker before preparation.")
$lines.Add("- ``ui_first_hour_dialogue.png``: required first-hour conversations focus the visible CONTINUE control after the target speaker opens.")
$lines.Add("- ``ui_first_hour_promise.png``: the promise beat distinguishes a hardened outer route from a second-operation breach and focuses CONTINUE CAMPAIGN before continuation.")
$lines.Add("- ``ui_first_hour_operations.png``: guided colony handoff keeps the next mission briefing and active preparation visible inside the open Operations drawer.")
$lines.Add("- ``ui_combat_feedback.png``: hit, miss, critical, damage, and recovery callouts remain anchored to the affected combatants.")
$lines.Add("- ``ui_construction.png``: placement blueprint and reserved resources remain visible at fixed building scale.")
$lines.Add("")
$lines.Add("## Files")
$lines.Add("")
$lines.Add("| # | Scene | File | Bytes | SHA-256 |")
$lines.Add("|---:|---|---|---:|---|")
for ($index = 0; $index -lt $records.Count; $index++) {
    $record = $records[$index]
    $lines.Add("| $($index + 1) | $($record.Scene) | ``$($record.Name)`` | $($record.Bytes) | ``$($record.Sha256)`` |")
}
$lines | Set-Content (Join-Path $captureDir "FINAL_CAPTURE_INVENTORY.md") -Encoding utf8

if ($issues.Count -gt 0) {
    throw "Capture audit failed with $($issues.Count) issue(s); see capture_audit.json"
}
Write-Host "Capture audit passed: $($records.Count) unique 1280x720 scenes."
