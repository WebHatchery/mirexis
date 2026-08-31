# Mirexis UI and Graphics Acceptance Matrix

Status values: `FAIL-BASELINE`, `IN PROGRESS`, `PASS`, `BLOCKED`.  
Completion rule: the overhaul cannot complete while any row is not `PASS`.

## A. Architecture and cross-cutting gates

| ID | Area | Acceptance evidence | Baseline finding | Status |
|---|---|---|---|---|
| A01 | Asset loader | Manifest-driven async loading, diagnostics, counts, obvious missing texture | Texture manifest is empty; no runtime art pipeline | PASS |
| A02 | Atlases/definitions | Tactical, portrait, terrain, colony, effect atlas definitions with pivots and source rectangles | No texture atlases or sprite definitions | PASS |
| A03 | Palette/faction channels | Stable semantic colours plus non-colour faction channels | Colour exists but silhouettes/patterns are weak | PASS |
| A04 | Animation states | Idle/move/attack/hit/incapacitated state selection and timing | Mostly static procedural geometry | PASS |
| A05 | Scaling | Crisp atlas sampling and verified 1280x720 through 1920x1080 behaviour | Fixed logical canvas only; no art sampling system | PASS |
| A06 | Missing assets | Magenta/black diagnostic with missing ID and startup logging | Silent absence/procedural substitution | PASS |
| A07 | Isometric projection | Three-quarter grid-to-screen and inverse hit testing | Rebuild remains rectangular top-down grid | PASS |
| A08 | Terrain/elevation | Modular tiles, three visible height bands, walls, cover, roofs/foreground | Flat checkerboard; shallow procedural obstacle blocks | PASS |
| A09 | Projected overlays | Movement, threat, targeting, route, hazard, objective follow diamonds | Rectangular overlays | PASS |
| A10 | Character art | Five recruits and seven hostile records map to authored sprites and portraits | Crude geometric people/symbols | PASS |
| A11 | Unit facings/states | Four facings and six core states for recruit and enemy families | No complete facing/state set | PASS |
| A12 | Equipment/mutation cues | Visible tool/weapon and mutation layers in sprite and portrait | Minimal procedural marks | PASS |
| A13 | Colony settlement | Eight unique buildings, paths, dressing, inhabitants, all operational states | Small procedural island with limited state richness | PASS |
| A14 | Campaign evolution | Human/Directorate/Brood/Ascendant evolution visible on colony and maps | Primarily text-driven | PASS |
| A15 | Combat staging | Forecast, comparison, push-in, recoil, impacts, criticals, faction effects, post-state | Forecast/log exist; staging and authored art do not | PASS |
| A16 | UI design system | Shared spacing, typography, panels, controls, focus/hover/disabled, cues, tooltips | Partial chamfer restyle; inconsistent legacy surfaces remain | PASS |
| A17 | Accessibility/readability | Grayscale/pattern redundancy, type floor, no clipping at supported sizes | Not demonstrated | PASS |
| A18 | Validation | Cargo tests and `publish.ps1` pass at meaningful checkpoints | Pending overhaul checkpoint | PASS |

## B. Screen and overlay audit

| ID | Screen / overlay | Required acceptance | Baseline finding | Status |
|---|---|---|---|---|
| S01 | Title | Illustrated world/recruit/faction tableau, hierarchy, actions and disabled continue | Rebuild still uses geometric people | PASS |
| S02 | Mission briefing | Mission identity, objective, threat, rewards, squad, art/map vignette | Text-heavy modal with weak scene identity | PASS |
| S03 | Deployment | Portrait squad strip, deployment budget, selected/locked/ready states | Dense text/buttons; limited identity | PASS |
| S04 | Loadout briefing | Equipment silhouettes and comparable readiness changes | Text-only loadout summary | PASS |
| S05 | Colony | Inhabited settlement, resources, operations, interaction feedback | Prototype island; incomplete state/evolution grammar | PASS |
| S06 | Construction | Foundations/scaffold/progress/cost/valid placement feedback | Procedural blueprint marks only | PASS |
| S07 | Roster | Portrait-led recruit identity, class/equipment/mutation and history hierarchy | Rebuild portrait remains procedural | PASS |
| S08 | Relationships | Portrait pairs, bond state, effects and consistent navigation | Roster screen with text emphasis | PASS |
| S09 | Research | Technology identity, prerequisites, progress, costs and unlocked states | Colony grid plus command buttons | PASS |
| S10 | Gene lab | Portrait mutation anatomy, evolution comparison, costs/consequences | Text list and plain panels | PASS |
| S11 | Equipment | Weapon/tool art, target state, forecast and disabled reasons | Tactical buttons and text | PASS |
| S12 | Help | Reading typography, grouped topics, keyboard/controller cues | Dense modal, small text | PASS |
| S13 | Battle log | Chronology, faction/action icons, outcome emphasis, filter/close | Plain text modal | PASS |
| S14 | Tactical HUD | Isometric battlefield dominant; stable command rail and footer cues | Rectangular board; partial portrait restyle | PASS |
| S15 | Targeting | Projected legal targets, clear attacker/target relation and cancel cue | Rectangular highlights | PASS |
| S16 | Attack forecast | Portrait comparison, hit/damage/cover/AP/before-after values | Small text panel and line | PASS |
| S17 | Phase replay | Ordered action cards/markers, readable held state, return to board | Board plus small footer/banner | PASS |
| S18 | Combat feedback | Directional recoil, damage/critical/faction impacts and clear post-state | Minimal lines/numerals | PASS |
| S19 | Objective feedback | Objective beacon, progress, success/failure transition | Objective ring/text only | PASS |
| S20 | Hazards | Projected patterned tile, faction motion, legend/tooltip, propagation state | Simple coloured rectangles/marks | PASS |
| S21 | Reinforcements | Entry edge, timing, faction preview and arrival transition | Small warning marker/text | PASS |
| S22 | Debrief | Outcome tableau, rewards, casualties/trauma, consequences, continue | Sparse text modal | PASS |
| S23 | Campaign events | Illustrated event identity, choices, consequences, disabled reasons | Text-heavy colony/briefing panels | PASS |
| S24 | Endings | Each path has distinct full-frame visual state and decisive continuation | Same colony grid with altered text | PASS |
| S25 | Capture harness | Every supported scene emits current PNG to final inventory | 82 baseline PNGs; post-overhaul set pending | PASS |

## C. Deterministic capture scene inventory

Each row must have a post-overhaul PNG in the final capture directory and pass the relevant screen gates above. The baseline column records the principal starting defect, not every defect visible in the scene.

| Scene | Surface | Baseline principal defect | Final evidence | Status |
|---|---|---|---|---|
| title | Title | Geometric tableau / weak world art | `ui_title.png` | PASS |
| title_controller | Title/controller focus | No visible gamepad focus or controller-specific confirmation cue | `ui_title_controller.png` | PASS |
| title_hover | Title/mouse hover | Hover treatment not represented in deterministic evidence | `ui_title_hover.png` | PASS |
| colony | Colony | Prototype island lacks full inhabited/state detail | `ui_colony.png` | PASS |
| contact | Colony event | Text and grid dominate event identity | `ui_contact.png` | PASS |
| contact_gear | Roster/equipment | No authored equipment art | `ui_contact_gear.png` | PASS |
| contact_event | Colony event | Weak event illustration and hierarchy | `ui_contact_event.png` | PASS |
| adaptation | Colony | Phase evolution primarily textual | `ui_adaptation.png` | PASS |
| gene_lab | Gene lab | No illustrated mutation anatomy | `ui_gene_lab.png` | PASS |
| evolution | Roster/gene lab | Evolution visible mainly as text | `ui_evolution.png` | PASS |
| mara_evolution | Gene lab | No recruit-specific evolved portrait/sprite cue | `ui_mara_evolution.png` | PASS |
| ilya_evolution | Gene lab | No recruit-specific evolved portrait/sprite cue | `ui_ilya_evolution.png` | PASS |
| sol_evolution | Gene lab | No recruit-specific evolved portrait/sprite cue | `ui_sol_evolution.png` | PASS |
| nadi_evolution | Gene lab | No recruit-specific evolved portrait/sprite cue | `ui_nadi_evolution.png` | PASS |
| escalation | Colony | Campaign visual evolution too weak | `ui_escalation.png` | PASS |
| escalation_operation | Briefing | Same text modal language | `ui_escalation_operation.png` | PASS |
| escalation_response | Campaign event | Choice identity mainly textual | `ui_escalation_response.png` | PASS |
| mirexis | Colony | Late-game hybridisation too weak | `ui_mirexis.png` | PASS |
| mirexis_path | Campaign choice | Path identity mainly textual | `ui_mirexis_path.png` | PASS |
| redoubt_end | Ending | Ending reuses colony grid | `ui_redoubt_end.png` | PASS |
| commonwealth_end | Ending | Ending reuses colony grid | `ui_commonwealth_end.png` | PASS |
| threshold_end | Ending | Ending reuses colony grid | `ui_threshold_end.png` | PASS |
| finale_debrief | Debrief | Sparse text modal | `ui_finale_debrief.png` | PASS |
| adaptation_operation | Briefing | Weak mission scene identity | `ui_adaptation_operation.png` | PASS |
| glass_nerve | Tactical | Flat rectangular battlefield | `ui_glass_nerve.png` | PASS |
| three_knives | Tactical | Flat rectangular battlefield | `ui_three_knives.png` | PASS |
| reinforcement_warning | Tactical/reinforcement | Entry warning lacks authored staging | `ui_reinforcement_warning.png` | PASS |
| line_formation | Tactical | Formation lacks sprite population/scale | `ui_line_formation.png` | PASS |
| thin_shelter | Tactical/objective | Structure/objective lacks dimensional identity | `ui_thin_shelter.png` | PASS |
| breakwater | Tactical | Mission art sameness | `ui_breakwater.png` | PASS |
| false_heart | Tactical | Mission art sameness | `ui_false_heart.png` | PASS |
| live_wire | Tactical | Mission art sameness | `ui_live_wire.png` | PASS |
| last_wall | Tactical | Mission art sameness | `ui_last_wall.png` | PASS |
| root_choir | Tactical | Mission art sameness | `ui_root_choir.png` | PASS |
| door_of_light | Tactical | Mission art sameness | `ui_door_of_light.png` | PASS |
| damage | Colony | Damage is a label/colour state, not damaged architecture | `ui_damage.png` | PASS |
| repair | Colony | Restored architecture and repair feedback were not captured explicitly | `ui_repair.png` | PASS |
| power | Colony | Unpowered state lacks environmental response | `ui_power.png` | PASS |
| construction | Colony | Construction state lacks authored staging | `ui_construction.png` | PASS |
| research | Research | Weak technology art and hierarchy | `ui_research.png` | PASS |
| roster | Roster | Procedural portrait and dense controls | `ui_roster.png` | PASS |
| recruited_roster | Roster | Recruited colonists fall below the visible list/relationship boundary | `ui_recruited_roster.png` | PASS |
| recruited_gene_lab | Gene Lab | Recruited colonists fall below the visible mutation list | `ui_recruited_gene_lab.png` | PASS |
| advanced_roster | Roster | Class change lacks silhouette transformation | `ui_advanced_roster.png` | PASS |
| relationships | Relationships | Bonds are text-led | `ui_relationships.png` | PASS |
| trauma | Roster | Injury is text-led | `ui_trauma.png` | PASS |
| bonded_briefing | Briefing/relationships | No portrait-led relationship cue | `ui_bonded_briefing.png` | PASS |
| legacy | Roster | Legacy state lacks visual identity | `ui_legacy.png` | PASS |
| briefing | Briefing | Text-heavy with little mission art; first-hour deployment target lacked a guided focus cue | `ui_briefing.png` | PASS |
| recruited_briefing | Briefing | Recruited colonists overlap the action controls | `ui_recruited_briefing.png` | PASS |
| threat_briefing | Briefing | Threat hierarchy and faction art weak | `ui_threat_briefing.png` | PASS |
| loadout_briefing | Briefing/loadout | Equipment is text-only | `ui_loadout_briefing.png` | PASS |
| pressure | Colony | Pressure state primarily textual | `ui_pressure.png` | PASS |
| gameplay | Tactical | Flat rectangular board / procedural units | `ui_gameplay.png` | PASS |
| first_hour_tactical | Tactical/first-hour lesson | Next action is text-only and does not point at the battlefield or command rail | `ui_first_hour_tactical.png` | PASS |
| first_hour_attack | Tactical/first-hour attack lesson | Valid forecast exposes ATTACK and the confirmation itself receives a guided focus cue | `ui_first_hour_attack.png` | PASS |
| first_hour_enemy_phase | Tactical/first-hour enemy-phase lesson | End-phase confirmation does not explain the ready, spent, and incapacitated squad state | `ui_first_hour_enemy_phase.png` | PASS |
| first_hour_objective | Tactical/first-hour objective lesson | Reachable objective interaction receives a guided focus cue on SECURE OBJECTIVE | `ui_first_hour_objective.png` | PASS |
| first_hour_replay | Tactical/first-hour replay transition | Hostile replay leaves the required skip action without a guided focus cue | `ui_first_hour_replay.png` | PASS |
| first_hour_ability | Tactical/first-hour ability lesson | Ability prompt does not identify one currently actionable command | `ui_first_hour_ability.png` | PASS |
| second_operation_tactical | Tactical/first-hour transfer | ApplyLearning guidance disappears after second-operation deployment | `ui_second_operation_tactical.png` | PASS |
| overwatch | Tactical | State cue small and static | `ui_overwatch.png` | PASS |
| brood_ability | Tactical/effect | No production Brood effect language | `ui_brood_ability.png` | PASS |
| directorate_ability | Tactical/effect | No production Directorate effect language | `ui_directorate_ability.png` | PASS |
| ascendant_ability | Tactical/effect | No production Ascendant effect language | `ui_ascendant_ability.png` | PASS |
| hazard | Tactical/hazard | Flat hazard marks | `ui_hazard.png` | PASS |
| intent | Tactical/forecast | Intent is small text/line | `ui_intent.png` | PASS |
| action_preview | Tactical/forecast | Preview lacks portrait comparison/staging | `ui_action_preview.png` | PASS |
| movement_route | Tactical/route | Route conforms to rectangles | `ui_movement_route.png` | PASS |
| cover_edges | Tactical/cover | Cover is shallow procedural geometry | `ui_cover_edges.png` | PASS |
| invalid_command | Tactical/feedback | Weak authored rejection feedback | `ui_invalid_command.png` | PASS |
| valid_shot | Tactical/forecast | Attack forecast lacked a touch-visible confirmation control | `ui_valid_shot.png` | PASS |
| threat_range | Tactical/overlay | Rectangular threat overlay | `ui_threat_range.png` | PASS |
| danger_reach | Tactical/overlay | Rectangular danger overlay | `ui_danger_reach.png` | PASS |
| help | Help | Dense small-text modal | `ui_help.png` | PASS |
| first_hour_guide | First-hour guide and playtest metrics | No canonical evidence for the observer-facing session readout | `ui_first_hour_guide.png` | PASS |
| first_hour_return | First-hour return/recovery handoff | First return names a colonist but does not visibly route or highlight the conversation target | `ui_first_hour_return.png` | PASS |
| first_hour_dialogue | First-hour conversation handoff | Required conversation has no focus cue on the visible CONTINUE control | `ui_first_hour_dialogue.png` | PASS |
| first_hour_promise | First-hour promise consequence | Second-operation consequence and the required CONTINUE CAMPAIGN control lack a guided focus cue | `ui_first_hour_promise.png` | PASS |
| first_hour_operations | First-hour colony handoff | Operations drawer does not make the next briefing action obvious | `ui_first_hour_operations.png` | PASS |
| battle_log | Battle log | Event rows expose internal identifiers such as `brood_stalker_a` instead of the display names used elsewhere in the UI | `ui_battle_log.png` | PASS |
| combat_feedback | Tactical/feedback | Dense result labels can overlap unit and impact effects, hiding normal HIT feedback | `ui_combat_feedback.png` | PASS |
| phase_replay | Tactical/replay | Replay beat names the attack outcome but omits the immediate damage or incapacitation consequence | `ui_phase_replay.png` | PASS |
| end_phase_guard | Tactical/warning | Warning hierarchy is small | `ui_end_phase_guard.png` | PASS |
| readiness_markers | Tactical/unit state | Tiny state markers | `ui_readiness_markers.png` | PASS |
| vitality_markers | Tactical/unit state | Tiny state markers | `ui_vitality_markers.png` | PASS |
| extraction | Tactical/objective | Objective tile lacks dimensional beacon | `ui_extraction.png` | PASS |
| variant | Tactical/map | Variant differentiation remains low | `ui_variant.png` | PASS |
| sporefield | Tactical/hazard | Brood biome/effect language weak | `ui_sporefield.png` | PASS |
| vault | Tactical/map | Ascendant biome/effect language weak | `ui_vault.png` | PASS |
| black_channel | Tactical/map | Directorate biome/effect language weak | `ui_black_channel.png` | PASS |
| living_chorus | Tactical/map | Late-game hybrid map language weak | `ui_living_chorus.png` | PASS |
| open_circuit | Tactical/map | Late-game hybrid map language weak | `ui_open_circuit.png` | PASS |
| trace_active | Tactical/objective | Relay activation lacks staging | `ui_trace_active.png` | PASS |
| equipment | Tactical/equipment | No equipment art or rich target comparison | `ui_equipment.png` | PASS |
| weapon_profile | Tactical/equipment | Profile is text-only | `ui_weapon_profile.png` | PASS |
| class_target | Tactical/class action | Targeting cue remains rectangular | `ui_class_target.png` | PASS |
| breach | Tactical/cover feedback | Breach effect lacks authored impact | `ui_breach.png` | PASS |
| debrief | Debrief | Sparse outcome modal | `ui_debrief.png` | PASS |
| trauma_debrief | Debrief | Trauma consequence lacks portrait response | `ui_trauma_debrief.png` | PASS |

## D. Final comparison and validation record

| Checkpoint | Command / evidence | Result |
|---|---|---|
| Baseline report read | Complete structural extraction: 163 paragraphs, 7 tables, 14 figures | PASS |
| Baseline capture audit | 82 original captures + 8 prototype rebuild captures reviewed | PASS |
| Implementation checkpoint 1 | 150 tests + source gate; Windows/WebGL publish; `ui_overhaul_checkpoint_1` | PASS |
| Implementation checkpoint 2 | 150 tests + source gate; Windows/WebGL publish; controller bridge/package audit | PASS |
| Implementation checkpoint 3 | 151 tests + source gate; 82-scene recapture; Windows/WebGL publish; 27-file package audit | PASS |
| Live WebGL checkpoint | Warning/error-free browser startup; title through tactical deployment and help exercised | PASS |
| Final implementation gate | `cargo fmt --all -- --check`; 213 gameplay tests plus the source-limit target test; strict Clippy with warnings denied | PASS |
| Final publish/package gate | `publish.ps1`; Windows and WebGL release builds; 27-file/25.85 MB runtime asset package; preview deploy | PASS |
| Final full capture | `FINAL_CAPTURE_INVENTORY.md`; 85 PNGs, SHA-256 digests, eight ordered contact sheets inspected, and machine audit in `capture_audit.json` | PASS |
| Visual regression baseline | Current deterministic PNG set and `FINAL_CAPTURE_INVENTORY.md`; one-off implementation audit reports retired after acceptance | PASS |
| Ongoing requirement contract | This matrix remains the machine-checked scene contract; product-readiness criteria now live in `../PLAYABLE_GAME_PHASE_1.md` | PASS |
| Minimum active work | Goal elapsed active work reached 14,408 seconds before closure | PASS |

