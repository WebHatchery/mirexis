# Mirexis Final Capture Inventory

Generated: 2026-08-31
Capture root: `docs/verification`
Manifest: `scripts/capture_ui.ps1`

## Integrity summary

- 100 manifest scenes and 100 canonical root PNG files; no scene is stored in a capture subfolder.
- Every capture uses the standard 1280 x 720 logical target.
- The harness overwrites each canonical `ui_<scene>.png` file in place.
- Tactical inspection covers default gameplay, movement routes, class targeting, valid shots, hazards, and the off-center zoom variant.
- Colony inspection covers the default settlement, construction placement, damage, repair, and power loss.
- `capture_audit.json` records automated size, duplicate, diagnostic-color, and visual-detail checks.
- The acceptance-document test fails if the manifest, screen matrix, and this inventory drift apart.

## Spatial acceptance evidence

- `ui_gameplay.png`: default tactical camera with a cropped 40 x 40 battlefield and readable elevation.
- `ui_variant.png`: off-center tactical camera with selection, objective, hazard, unit, and contextual overlays intact.
- `ui_movement_route.png` and `ui_valid_shot.png`: transformed route, target overlays, and the visible ATTACK confirmation remain aligned.
- `ui_class_target.png`: transformed unit targeting and contextual card remain aligned.
- `ui_colony.png`: centered settlement with a visible first-hour coordinator route and undeveloped 20 x 20 frontier.
- `ui_briefing.png`: first-operation deployment guidance focuses DEPLOY SQUAD while preserving the selectable roster rows.
- `ui_first_hour_tactical.png`: guided tactical lesson marks a legal move target without blocking the battlefield inspection.
- `ui_first_hour_ability.png`: guided ability lesson focuses a currently actionable mutation, class action, or field item.
- `ui_first_hour_enemy_phase.png`: enemy-phase guidance pairs the focused END PHASE control with READY, SPENT, and INCAP counts.
- `ui_second_operation_tactical.png`: the persisted transfer lesson keeps the active preparation, ApplyLearning hostile focus, and forecast prompt visible after deployment.
- `ui_first_hour_return.png`: the first-return recovery handoff routes the player to Ilya's highlighted speech marker before preparation.
- `ui_first_hour_dialogue.png`: required first-hour conversations focus the visible CONTINUE control after the target speaker opens.
- `ui_first_hour_promise.png`: the promise beat distinguishes a hardened outer route from a second-operation breach and focuses CONTINUE CAMPAIGN before continuation.
- `ui_first_hour_operations.png`: guided colony handoff keeps the next mission briefing and active preparation visible inside the open Operations drawer.
- `ui_battle_log.png` and `ui_phase_replay.png`: attack history and hostile replay beats use readable display names, name HIT/MISS outcomes with immediate damage consequences, and the battle log distinguishes ability, recovery, status, objective, and cover events with matching counts on each filter.
- `ui_debrief.png`: the operation result keeps rewards and squad consequences visible while the notification stack names each automatic technique learned by a readable colonist name.
- `ui_combat_feedback.png`: hit, miss, critical, damage, recovery, objective, and cover callouts remain anchored to their affected field locations and stack clear of unit effects.
- `ui_construction.png`: placement blueprint and reserved resources remain visible at fixed building scale.

## Files

| # | Scene | File | Bytes | SHA-256 |
|---:|---|---|---:|---|
| 1 | title | `ui_title.png` | 441463 | `1e46ada6fd89ca718fc6756f2f4236095e8179817a9f91814e0d44715016dddd` |
| 2 | title_controller | `ui_title_controller.png` | 443843 | `157f42abc431917d32605da100c0f246b401f18c2df1aff34bbb40fb0d540848` |
| 3 | title_hover | `ui_title_hover.png` | 443805 | `af49ed5bd1aa997dbef06619485e401da09091f0b05292ef6c7cf44a951820fb` |
| 4 | colony | `ui_colony.png` | 879197 | `0d243d778864aeae362d4cfe4bd31bf7626e3f6b623b989ddf776fa708ead902` |
| 5 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739486 | `fc88587d80bfe9e55edcc3c015b272327a86843d487b91ca218e1c15fa314a07` |
| 6 | contact | `ui_contact.png` | 743344 | `c28d59287159d3a9210737843b140621e03f675b2e3bc7f2eb1b7bfde0062861` |
| 7 | contact_gear | `ui_contact_gear.png` | 255491 | `395a64c68d82570af60f4b1ed56002a3915f344760f19129db878de8562710fd` |
| 8 | contact_event | `ui_contact_event.png` | 743312 | `c1531517e3afb28d3037a1ca677b1bae20b0650d3fa803073327a09a0911e1ef` |
| 9 | adaptation | `ui_adaptation.png` | 749657 | `92e0c1d6e6796c8d98654ad4486c0a6bddcb302ebd678c25f457625521f87ce2` |
| 10 | gene_lab | `ui_gene_lab.png` | 180981 | `15333937ce5781c46d30a7a02ec01bf306743cb811a6b9d165809fbe37f3b5ef` |
| 11 | evolution | `ui_evolution.png` | 249737 | `0fd21e300fd38d9b1b12d0aafbf657bbb6775809b2c2f203dd6986c4af56ce86` |
| 12 | mara_evolution | `ui_mara_evolution.png` | 180425 | `36aeac0e75ed9e988f7fd2b65dadff0d74efbf66b0ad347f2068f4516c580c1b` |
| 13 | ilya_evolution | `ui_ilya_evolution.png` | 180215 | `c7e600f48fabf0363275cd6a41eaa9af3ee22a42c1a10339eb97d6d8d58dab9d` |
| 14 | sol_evolution | `ui_sol_evolution.png` | 174924 | `d62ee19981a701bb8be3ccea1a7c08f55304fa193e8e14ad22977563030d0eb1` |
| 15 | nadi_evolution | `ui_nadi_evolution.png` | 185539 | `e4985cef48b0ee9eeb3a2a7a6b9419dee5dbe797b60b7912fa10c6fa75aa5eaa` |
| 16 | escalation | `ui_escalation.png` | 753059 | `83db26465f8cb484169b00cbbbaa455b33fed5c8c0b978a66a507e23e163ea16` |
| 17 | escalation_operation | `ui_escalation_operation.png` | 211944 | `fed67d33003a761ac0f07acbe292b954fc43da05303ed8c0e71151107a4f1975` |
| 18 | escalation_response | `ui_escalation_response.png` | 753248 | `26204ba62db6594004b35ea5c705858ec0a0c3e9c3d309f82f629d05a9229390` |
| 19 | mirexis | `ui_mirexis.png` | 747673 | `d95085a5825fbe8aa0a778a0f5411e680bbe6cd30da0ca454d949c6bc62e20ce` |
| 20 | mirexis_path | `ui_mirexis_path.png` | 749059 | `9d5db1f423ca9def95af43a48edb6d4115fb4a3df8064ddc00d4962fcf5c0be0` |
| 21 | redoubt_end | `ui_redoubt_end.png` | 751153 | `fa6639dd82f8dfbaae6f92bb7e9bff60e3015107cbdcb3828f89ac82a11f20fa` |
| 22 | commonwealth_end | `ui_commonwealth_end.png` | 760299 | `0cd412eab4b6f6e623ce2b8e53f6bc4119822f00d0e82756023f89ac557d6d9f` |
| 23 | threshold_end | `ui_threshold_end.png` | 754324 | `097bb2b7fd1c3d8dbd20477ca9451cd287d870b6f8d91c094458e292f9e33482` |
| 24 | finale_debrief | `ui_finale_debrief.png` | 200847 | `2c4324536623d475887d817c684db49d63c8b1d51d6b835acbcfd40fbe39da30` |
| 25 | adaptation_operation | `ui_adaptation_operation.png` | 195292 | `348b40fd4802a2cc36249f82152ed1e1247d287382da1f7f4171b722b481e1c4` |
| 26 | glass_nerve | `ui_glass_nerve.png` | 1186698 | `7cbd0f2aa6cf98a7012688d2b7ef1e8eebbb902ab2fbb4a9c1b077e003fdf7f4` |
| 27 | three_knives | `ui_three_knives.png` | 1190743 | `375b3774167c9225a1ccc7ca87c10a59dc2cb89374d14fe588a6724c773eb79b` |
| 28 | reinforcement_warning | `ui_reinforcement_warning.png` | 1191071 | `dead5ea967344a27e820b1219b84f4cc989cfc9f66b49f2c428624b57a702fef` |
| 29 | line_formation | `ui_line_formation.png` | 1186413 | `a1732bcf0253c5f73d72163dbdb156ae66bb086d86e8db7b80a381e1254deaa0` |
| 30 | thin_shelter | `ui_thin_shelter.png` | 1188308 | `09be2029de375cea03e5114148d2c8268ea213a751bcdd7f27db55c1af600731` |
| 31 | breakwater | `ui_breakwater.png` | 1180581 | `09e0e00e0b14ef4db71ff00d3ce4ee2050b5bbe9e62a44dd683baacea2303002` |
| 32 | false_heart | `ui_false_heart.png` | 1187858 | `8194ca142e662e566524c7f7701b30be6773af5e344a691e82bededf5ff69acb` |
| 33 | live_wire | `ui_live_wire.png` | 1192221 | `431197863a98d66d75dbf5ad190fb486e0bb07748e624a1bb04e0846dbe1719d` |
| 34 | last_wall | `ui_last_wall.png` | 1182177 | `b6c3f5ae3aafca876e69259d0235decc37c90f6f333237f852ecc1e7d599f7cf` |
| 35 | root_choir | `ui_root_choir.png` | 1192010 | `ed6e7181ee711dec596c87609facb27748af0969d07d9a8ab9dc202cfd72e8f4` |
| 36 | door_of_light | `ui_door_of_light.png` | 1184121 | `df1efc5d9581972c8c8fb15f723e78e6cc78a006550fe61c91b713224f6ce3e6` |
| 37 | damage | `ui_damage.png` | 743865 | `e7a7700cbecb25f5d16015b5393906871f0964f70885f60355a78c42ffc3c45c` |
| 38 | repair | `ui_repair.png` | 750702 | `a130d2dfb43be92bf9adeaf75ed16de58ae658ff30005d3125fb3c1ba1671e42` |
| 39 | power | `ui_power.png` | 742191 | `3f482b1e3fd9645afc66cefa4423f2d12201e8616ea0f7b91b3d0a515bfa442e` |
| 40 | construction | `ui_construction.png` | 743993 | `43385b8287f5f5556d764f8a2d7cc6ee663126df533f1681c2c65a22927bfe95` |
| 41 | research | `ui_research.png` | 743676 | `1030070408c992cda5730de80770be9388759bc3a169db6a93a85fe978440fca` |
| 42 | roster | `ui_roster.png` | 254323 | `4fb53223201f266b393f3885bda55c96b927d1f1dff6353c8bbf7156a3d90d63` |
| 43 | recruited_roster | `ui_recruited_roster.png` | 286857 | `4e93e68ebb8b3854815efd873acbc4aa8044802bdfd7f6b9bc5e27a8a428035a` |
| 44 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 208809 | `f570a715dda664dd70ef8bd6bfb1b6e0acc052ba1544ac7dc2a0a49de6628255` |
| 45 | advanced_roster | `ui_advanced_roster.png` | 254254 | `171c16d5152ee73fdac96fd80a0ff741fee48d93e141d58986a0e6062f453c8a` |
| 46 | relationships | `ui_relationships.png` | 255780 | `de1be4679f94b310eb668cedd08508d88320d62437d275077541678ae5518da4` |
| 47 | trauma | `ui_trauma.png` | 258340 | `7989dfe6959f94738819c8f05ce8568dd0fe1173688aa527bf81e875e07126ca` |
| 48 | bonded_briefing | `ui_bonded_briefing.png` | 201843 | `6e6adb63620d7bca4a0f420de49200cee50e9f7af93c180bd1b963077ba0d67b` |
| 49 | legacy | `ui_legacy.png` | 254860 | `5672778f1a49fc56a0caeb7e458885188415ab8b52970a958a8f0bce3aef620f` |
| 50 | briefing | `ui_briefing.png` | 210885 | `ed3e997f1d725af6f69507b0afb421f54f7512bdeb7e8a3df1992a7e994c3014` |
| 51 | recruited_briefing | `ui_recruited_briefing.png` | 213307 | `ef223729033d2a38c857761cdf67a1e2b03ee984922b3118875094680f07ee0d` |
| 52 | threat_briefing | `ui_threat_briefing.png` | 193721 | `abd60020d0b888bb861bc3773f980961dc4c4f56366c98eed5e23b7a878dc8fd` |
| 53 | loadout_briefing | `ui_loadout_briefing.png` | 200779 | `0b91fc82bec04925db6647fbb72e798f3333c19fbaa01d9e7047c4076b990330` |
| 54 | pressure | `ui_pressure.png` | 208230 | `c151b1a5c9f3772c86716fe98363cc058729595da6e86cae256bd3e3d451e553` |
| 55 | gameplay | `ui_gameplay.png` | 1185805 | `5336c4ae746e0d98b02e7fe65d8dfa20c7ed515cb21877cbd76393178114c468` |
| 56 | first_hour_tactical | `ui_first_hour_tactical.png` | 1191602 | `191064225612cbcedf7653ce19ae210c94e560d584916626a90080a586c32545` |
| 57 | first_hour_attack | `ui_first_hour_attack.png` | 1108169 | `6d8d5e6b8cb8f87b790fb18990384f684d2e94c2a981ef880b8e91d334319d88` |
| 58 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1200636 | `5325875368aa81ad0b1273e94ea182771c2e6949fb3955d444038abc911f2b72` |
| 59 | first_hour_objective | `ui_first_hour_objective.png` | 1343068 | `ebd5b005bc29536f3f9ae5fb833b2f8eccd9892e5d30700109917203c6daec33` |
| 60 | first_hour_replay | `ui_first_hour_replay.png` | 1101868 | `404966a8fe9405aab2f6b346d41aae290d2db5660cdee9a98ca14010cd422efb` |
| 61 | first_hour_ability | `ui_first_hour_ability.png` | 1190999 | `ea7f71368c47bfd7c5df98e6b4dc08cf16096c83ec2b95582ba2392c524d2035` |
| 62 | second_operation_tactical | `ui_second_operation_tactical.png` | 1113060 | `a9518d3808c601bf260f22b84c9fed5dd2cc367a31a091d9496e7102bfa3d348` |
| 63 | overwatch | `ui_overwatch.png` | 1188872 | `90c37859332461958c1f8aace42cda4987b9bc999bbb8b69ed6b1f48f09331a5` |
| 64 | brood_ability | `ui_brood_ability.png` | 1245299 | `4419114d414a9eb72caf6d5493f2dbd28e9993e85a64f91757c288d8e37a6468` |
| 65 | directorate_ability | `ui_directorate_ability.png` | 1245539 | `4be83733c1fa0e66f3f44be7e3929c6272b9d62a9846674f1af269e3cc742224` |
| 66 | ascendant_ability | `ui_ascendant_ability.png` | 1245496 | `dda4c0f08c96801030fcc42eec2bf2df70ef3f75f67516a9ca06f04cd7c739a0` |
| 67 | hazard | `ui_hazard.png` | 1250902 | `3ac62126620b14f0b4b3ef5ffe86203706ad4bfec31e4616c94ce6ba193c4461` |
| 68 | intent | `ui_intent.png` | 1147631 | `75b295104392f82822e6349063a1ae14726cbac43739488bb7000d11bb858fc9` |
| 69 | action_preview | `ui_action_preview.png` | 1110645 | `2f897563fd81fbbaae20457f47d7e43fbda0fd318a4e51b42e6184716edca175` |
| 70 | movement_route | `ui_movement_route.png` | 1147183 | `e8b37cc31ef02c227ba93e739258c190b764c3a92cc6c9c6c37124bfc8f17360` |
| 71 | cover_edges | `ui_cover_edges.png` | 1186850 | `ca06a39459afc8faf517c615980d93a23cbdd56d0d17829d18892e2e51bd0214` |
| 72 | invalid_command | `ui_invalid_command.png` | 1215344 | `17fbf8b79191c287691ada17c0febf4881251e08ea7fcdf813efd4b7105a93fc` |
| 73 | valid_shot | `ui_valid_shot.png` | 1106082 | `92a8a840c46df7ec8fc9f6d5ef91ede546c2238d4203a3945362e4e592582719` |
| 74 | threat_range | `ui_threat_range.png` | 1218476 | `12738a922d760dea862fac4c271fe039863bd65af261f9ab0e811266312392b4` |
| 75 | danger_reach | `ui_danger_reach.png` | 1214726 | `fd7927ec4fca419dc534355c058e3b6db6fd673400ef5c4ec924f20e5aba2408` |
| 76 | help | `ui_help.png` | 357168 | `3744d9a294a9c182b8d177017b12f88f35c49de4c5bfdccf9e51c77f8af3c538` |
| 77 | first_hour_guide | `ui_first_hour_guide.png` | 215086 | `3914697b5a359239e6bd485506ce955e2904c3ce9cb8c278af3e552dd11f410b` |
| 78 | first_hour_return | `ui_first_hour_return.png` | 878751 | `04d9aebe69327cfb7d7a379112dcbdb73ca8c7a8533bb4886103802e8690ccec` |
| 79 | first_hour_promise | `ui_first_hour_promise.png` | 881399 | `dbe4c79162bed43496539774c1bbe7c3a4c6661fa8a94a1ecabcd2cb77d51a39` |
| 80 | first_hour_operations | `ui_first_hour_operations.png` | 920559 | `620b0cf88a42a372e6b15a7c56875f25c2ebf1d1080583b313c05792cd3e1939` |
| 81 | battle_log | `ui_battle_log.png` | 660880 | `8c9162a4f4444324d1ca404e573a316dd46cab6a27d6c7be94812cb6cd123a55` |
| 82 | combat_feedback | `ui_combat_feedback.png` | 1194741 | `ec9de51d34b93e580d6b249f135cf2ddf45cfc2cc01ee6515c80217a7476099a` |
| 83 | phase_replay | `ui_phase_replay.png` | 1095988 | `9269f8ddcd6f27b8816e400744d0ec9e5c63065de5163754d9abde4f94fc7c02` |
| 84 | end_phase_guard | `ui_end_phase_guard.png` | 1186008 | `b89ae39b33d8a2e8564aa9009faa73d71266cdd483dd77ec4929be9eebbc3415` |
| 85 | readiness_markers | `ui_readiness_markers.png` | 1190105 | `6c39d4d0a02194518e1331ef13eb9aa3e435b005c257893b5b57c4d35e9b0454` |
| 86 | vitality_markers | `ui_vitality_markers.png` | 1190529 | `b07345c7b0f04c8ed5a13cec294234f09eca6c9b61b3febeedbb3cc28b321319` |
| 87 | extraction | `ui_extraction.png` | 1439404 | `c00664d700cbd85d2bd392e617bebc9e6d4b06e9915de91ab317ebd8706430ac` |
| 88 | variant | `ui_variant.png` | 1238156 | `5fcaf6f868d14721ad403065a86a524e878648d505e497782bfae149e4901234` |
| 89 | sporefield | `ui_sporefield.png` | 1190948 | `f5e06dfe5872ba52787905bf770b5426f36438bfe293654e542eac61b94925fa` |
| 90 | vault | `ui_vault.png` | 1186412 | `01cebf8c8dc28369a0a411e541e73e51596f041350d6bba79c82024054f82153` |
| 91 | black_channel | `ui_black_channel.png` | 1194613 | `37ebaaa02e5839e4484ca00d2bcfd6d8b7a256c2554362cf76ca5c59324e4483` |
| 92 | living_chorus | `ui_living_chorus.png` | 1199186 | `0f4e30f09c59ab4da604ab3d190ab41768d8ef0e18183f315bc6b0e98e6e2a34` |
| 93 | open_circuit | `ui_open_circuit.png` | 1198023 | `363c4b50dcf643f363d2ef89c8d86dfd6c8bb277bd66979719296bd79f4bad81` |
| 94 | trace_active | `ui_trace_active.png` | 1161977 | `f7bb51a29aa5d23b692fd23e345fc34a2251e8be9347ab14b6e3bb9c271b066d` |
| 95 | equipment | `ui_equipment.png` | 1019155 | `de00553063684c3b4c347eb1ae7351c4d78cdbe578eeb5fef7757ab7c8e19715` |
| 96 | weapon_profile | `ui_weapon_profile.png` | 1185932 | `9c52b564adc03420a975b0a7d69980a67d8b63232739101579539c0ee4fcfe37` |
| 97 | class_target | `ui_class_target.png` | 1038372 | `21e3e870dbfd440883f3170cd85a266719f15cacc135b0674bf492d1db9e3497` |
| 98 | breach | `ui_breach.png` | 1368427 | `0e28167e3c220279695dba370f16d375b0b8e282afc3b035f93db730d86b4926` |
| 99 | debrief | `ui_debrief.png` | 218633 | `2c2fe4a77d1b81415119bb575ca59a8a58da18fbbd79e9764419ca3bd04ae1dd` |
| 100 | trauma_debrief | `ui_trauma_debrief.png` | 201829 | `9535965ff998d2e4b194e0465773a17470d8a4e8c0dcce061506ce5ee93e78eb` |
