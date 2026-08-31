# Mirexis Final Capture Inventory

Generated: 2026-08-31
Capture root: `docs/verification`
Manifest: `scripts/capture_ui.ps1`

## Integrity summary

- 104 manifest scenes and 104 canonical root PNG files; no scene is stored in a capture subfolder.
- Every capture uses the standard 1280 x 720 logical target.
- The harness overwrites each canonical `ui_<scene>.png` file in place.
- Tactical inspection covers default gameplay, movement routes, class targeting, valid shots, hazards, hostile intent, readable active effects, and the off-center zoom variant.
- Colony inspection covers the default settlement, construction placement, damage, repair, and power loss.
- `capture_audit.json` records automated size, duplicate, diagnostic-color, and visual-detail checks.
- The acceptance-document test fails if the manifest, screen matrix, and this inventory drift apart.

## Spatial acceptance evidence

- `ui_gameplay.png`: default tactical camera with a cropped 40 x 40 battlefield and readable elevation.
- `ui_variant.png`: off-center tactical camera with selection, objective, hazard, unit, and contextual overlays intact.
- `ui_movement_route.png` and `ui_valid_shot.png`: transformed route, target overlays, and the visible ATTACK confirmation remain aligned.
- `ui_class_target.png`: transformed unit targeting and contextual card remain aligned.
- `ui_intent.png`: inspected hostile intent names active status effects and the remaining phases beside the forecast.
- `ui_colony.png`: centered settlement with a visible first-hour coordinator route and undeveloped 20 x 20 frontier.
- `ui_settings.png`: persistent audio and readability controls show the current volume level, mute state, reduced-motion state, and return path.
- `ui_field_notes.png`: the Operations archive keeps chronological speaker, title, transcript, navigation, and return context visible.
- `ui_memorial.png`: the Operations register keeps persistent scars, recovery records, lost objectives, and character legacies visible with an explicit return path.
- `ui_memorial_page_two.png`: the paginated Operations register keeps older long-campaign entries reachable with an explicit PREVIOUS path.
- `ui_contact_event.png`: the active Contact aftermath event opens in the Operations drawer with its participants, carrier choices, and consequence summary visible.
- `ui_contact.png`, `ui_research.png`, `ui_adaptation.png`, `ui_escalation_response.png`, and `ui_mirexis_path.png`: progression references open the Operations drawer at the decision surface they represent, keeping dossier, doctrine, and path context visible.
- `ui_briefing.png`: first-operation deployment guidance focuses DEPLOY SQUAD while preserving the selectable roster rows.
- `ui_first_hour_tactical.png`: guided tactical lesson marks a legal move target without blocking the battlefield inspection.
- `ui_first_hour_ability.png`: guided ability lesson focuses a currently actionable mutation, class action, or field item.
- `ui_first_hour_enemy_phase.png`: enemy-phase guidance pairs the focused END PHASE control with READY, SPENT, and INCAP counts.
- `ui_second_operation_tactical.png`: the persisted transfer lesson keeps the active preparation, ApplyLearning hostile focus, and forecast prompt visible after deployment.
- `ui_first_hour_return.png`: the first-return recovery handoff routes the player to Ilya's highlighted speech marker before preparation.
- `ui_first_hour_dialogue.png`: required first-hour conversations focus the visible CONTINUE control after the target speaker opens.
- `ui_first_hour_promise.png`: the promise beat distinguishes a hardened outer route from a second-operation breach and focuses CONTINUE CAMPAIGN before continuation.
- `ui_first_hour_operations.png`: guided colony handoff keeps the next mission briefing distinct from the framed event decision card, with both carrier choices and the colony plan visible inside the open Operations drawer.
- `ui_adaptation_operation.png` and `ui_escalation_operation.png`: threat-intelligence rows stay inside the right briefing column, with repeated hostile roles collapsed and long labels fitted before the deployment controls.
- `ui_roster.png`, `ui_recruited_roster.png`, and `ui_advanced_roster.png`: the complete workshop equipment catalog stays inside the training panel and above the footer instruction line.
- `ui_battle_log.png` and `ui_phase_replay.png`: attack history and hostile replay beats use readable display names, name HIT/MISS outcomes with immediate damage consequences, and the battle log distinguishes ability, recovery, status, objective, and cover events with matching counts on each filter.
- `ui_debrief.png`: the operation result keeps rewards and squad consequences visible while the notification stack names each automatic technique learned by a readable colonist name.
- `ui_finale_debrief.png`: the completed-campaign title, bounded legacy summary, operation result, and reward card remain separated and readable.
- `ui_redoubt_end.png`, `ui_commonwealth_end.png`, and `ui_threshold_end.png`: the identity ending card and bounded colony legacy register remain inside the open Operations drawer without the obsolete colony plan line colliding with the register.
- `ui_gene_lab.png` and `ui_recruited_gene_lab.png`: the mutation anatomy scan label remains visible above the separated irreversible evolution cards.
- `ui_combat_feedback.png`: hit, miss, critical, damage, recovery, objective, and cover callouts remain anchored to their affected field locations in separate readable plates above unit effects.
- `ui_construction.png`: placement blueprint and reserved resources remain visible at fixed building scale.

## Files

| # | Scene | File | Bytes | SHA-256 |
|---:|---|---|---:|---|
| 1 | title | `ui_title.png` | 441801 | `4c908f4fdb4dea26e9c941a87ef7b737c46a3650087fd8cc5d9af36ec6f1c365` |
| 2 | title_controller | `ui_title_controller.png` | 444305 | `957ba6dab4aaa7bcb8a630b4194985011bd00e5a410e16d6809b52acc7ed4759` |
| 3 | title_hover | `ui_title_hover.png` | 441779 | `050d069f8d224264f99809eb52c28c769e3e1fa4d6908ad526f4be3bf8dc7732` |
| 4 | colony | `ui_colony.png` | 879220 | `bb155e224532d9cd3447521ef322ceb54cffcb176dafe192958708ebe5e4f73e` |
| 5 | settings | `ui_settings.png` | 230013 | `e4a388bbbbdcb66174520c948b15117b2b05cfbf9d7d7562e2791dc106c7f995` |
| 6 | field_notes | `ui_field_notes.png` | 124896 | `bd713a2f387913e9b389ad43d9cf62e0e6f448566e35300699b3eff04f78c902` |
| 7 | memorial | `ui_memorial.png` | 168374 | `c068c1855b5d9ef013e48a3f9bccddb57ae6434ac102bc804dc0f6d3fe85f1d3` |
| 8 | memorial_page_two | `ui_memorial_page_two.png` | 121891 | `07e59fcee69c04131f00cd887e607e55b016c83b9c79c402bbf176a6ff9b3edf` |
| 9 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739635 | `df03c9c785f1d4a60b23552db7dfb66b433b92e39588e10d8c8cf7e5ca83f7cb` |
| 10 | contact | `ui_contact.png` | 936222 | `b0fcb2f71ccaf578cda03379d0082a219a8aa2ee573ffcdc621fda89f764be47` |
| 11 | contact_gear | `ui_contact_gear.png` | 250763 | `d9ddb04705bd5c8df1f18a4ea3d9a65fd996d2ad855a5f3aba18901bb2b00eaf` |
| 12 | contact_event | `ui_contact_event.png` | 923715 | `2687cae73c04d964793c31f485f7499ce25b05aecc13fa110c4e76e165bd6f5c` |
| 13 | adaptation | `ui_adaptation.png` | 898335 | `78665c468dcc0fe982c6adbfe2bbdc3438c89b4ac6056d8a2a8167e7e06d1929` |
| 14 | gene_lab | `ui_gene_lab.png` | 182141 | `059d67e8b00486c4627f9a999121b46ea2612ff824fe9a9d23446d7457483249` |
| 15 | evolution | `ui_evolution.png` | 245335 | `2ff441956d97947ab27f94d8fde4b209402009a94b8facd2c60d24cee3a1edd4` |
| 16 | mara_evolution | `ui_mara_evolution.png` | 180425 | `36aeac0e75ed9e988f7fd2b65dadff0d74efbf66b0ad347f2068f4516c580c1b` |
| 17 | ilya_evolution | `ui_ilya_evolution.png` | 180215 | `c7e600f48fabf0363275cd6a41eaa9af3ee22a42c1a10339eb97d6d8d58dab9d` |
| 18 | sol_evolution | `ui_sol_evolution.png` | 174924 | `d62ee19981a701bb8be3ccea1a7c08f55304fa193e8e14ad22977563030d0eb1` |
| 19 | nadi_evolution | `ui_nadi_evolution.png` | 185539 | `e4985cef48b0ee9eeb3a2a7a6b9419dee5dbe797b60b7912fa10c6fa75aa5eaa` |
| 20 | escalation | `ui_escalation.png` | 901052 | `1c0ddae9e1b2bb57bdc66bc8adc4b41936227d0d9077c84ef64c249882aa6524` |
| 21 | escalation_operation | `ui_escalation_operation.png` | 205398 | `0ff67a6e2b8c60a8d5129f6ec0fb767cc9c2a77fd6dc37b3c4f6d3c66e9b7ce4` |
| 22 | escalation_response | `ui_escalation_response.png` | 943058 | `25c8f44de9dcb6bcfd648da7e675c2fd54e486668adecd860897460e4336803d` |
| 23 | mirexis | `ui_mirexis.png` | 943045 | `3aa82d3fd38c5e40188bae8e9791a519f2e6e0a553cd75bc8b8b412486dfb89f` |
| 24 | mirexis_path | `ui_mirexis_path.png` | 903198 | `fef113a9411906cb290d5608cc35adee361126a971f2209ff83a23fa7e0b534d` |
| 25 | redoubt_end | `ui_redoubt_end.png` | 922881 | `fa8adf74a9c8e8f9f3d35f5caa732bd30a245f4d1c83fcf412308e10d4fe63f3` |
| 26 | commonwealth_end | `ui_commonwealth_end.png` | 931971 | `b74bbfa7582f762198225803ec6bb59918deb2a429ecf34615ed56f69c9d9546` |
| 27 | threshold_end | `ui_threshold_end.png` | 932927 | `51f99a7a34da92405141429d9b113aec36bdc43497954e47756f2e114efe4f49` |
| 28 | finale_debrief | `ui_finale_debrief.png` | 198007 | `a7a733be483ff1a8797b0f0fac129ab9b1ebe5d4f41b1d654215f3da23016ab0` |
| 29 | adaptation_operation | `ui_adaptation_operation.png` | 190784 | `84a31c8caf192d50726d2c451f1811f2ed6523e2f6c9665a449a526fc25fc2eb` |
| 30 | glass_nerve | `ui_glass_nerve.png` | 1187705 | `7af047be43252e381f2188f42da67ba09a39305acd268801175d7af3b7a8e68a` |
| 31 | three_knives | `ui_three_knives.png` | 1191754 | `ae6f4e41b811282a67242d3c888cd6ca80bdeb4fe845d0b018627ac62fbef17d` |
| 32 | reinforcement_warning | `ui_reinforcement_warning.png` | 1192082 | `22dc649b925d82d536236f91cad0aabbe3f7705e0ad30e395795f5cc3a6a6f14` |
| 33 | line_formation | `ui_line_formation.png` | 1187419 | `05a63ccaa64a5edaab5a7c4c24c4b2785a49b41cfd87b98da5a1a05f4d7e67e0` |
| 34 | thin_shelter | `ui_thin_shelter.png` | 1189319 | `4b2725822aa0c329a44af67d4bf13f8f63a81c79a0aaa858dcb1a550ced14c60` |
| 35 | breakwater | `ui_breakwater.png` | 1181591 | `018be2e0129eafa3072769f8e3879958e956a854d2283c3838dd6f1d4d2ed197` |
| 36 | false_heart | `ui_false_heart.png` | 1188868 | `7d4e5c851f644efc278259e3a155c73f3ce86f3cecd425a7f7f86f814e2de73e` |
| 37 | live_wire | `ui_live_wire.png` | 1193234 | `095aa89e54708bd4b92fee1af0d2db72d9a4c720b01a6c084ded75338b67f59f` |
| 38 | last_wall | `ui_last_wall.png` | 1183184 | `ed4e7a633b564b0255fe936410eb98d5161a0f6b3f4342a2addbb07a9e95811c` |
| 39 | root_choir | `ui_root_choir.png` | 1193020 | `40530c7c5feeeb73d420de49ce34fc7a7a26147d3a8b41e0d5caa7571760937e` |
| 40 | door_of_light | `ui_door_of_light.png` | 1185127 | `8754b2add1a0325487a97e0c40f3cfbef4a9cd36a50f7c18ee45d34ae93722ae` |
| 41 | damage | `ui_damage.png` | 883820 | `0b81be6b81576177dbf5a7cc9cb0e04093ccd1dd5c9237aad13f8ed238a5a562` |
| 42 | repair | `ui_repair.png` | 890712 | `52eae065ce2f91a6b5b611ed3bb7167d82b897068e23cbb745b497b9bd644d26` |
| 43 | power | `ui_power.png` | 882488 | `77c5c1595a2eb6445b6cd49513d22cb941d721a4b7e8adec780bdb913289670d` |
| 44 | construction | `ui_construction.png` | 884153 | `2a0575b1ad6d8d129c8577b65b6517833a00cb92039244c9b0b6acd0dd77942b` |
| 45 | research | `ui_research.png` | 909179 | `02abb8de3d70d790db4060bfcc8fa7464dbe326c4ad74072168063a3f82cbc4b` |
| 46 | roster | `ui_roster.png` | 249450 | `60c54c7acb77a1cdbf7c71bc8eb04ad04e9d05810626a1d2e48aa9cac20bfcbe` |
| 47 | recruited_roster | `ui_recruited_roster.png` | 282004 | `7f03338e19432c91492d6cddfa9e83ee556aa5d4ce57a10b19eed5f4a5e20f1d` |
| 48 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210284 | `65f791e048475e8693abaf0338305191bbc432143187db0e286dfd250216b3e8` |
| 49 | advanced_roster | `ui_advanced_roster.png` | 249299 | `5ee8e54011f979bef59112dd19de792538b82649ecbdc329dd6e71338c2e7b34` |
| 50 | relationships | `ui_relationships.png` | 251499 | `c0972eecf870ce0d6740cb112611e10b2737a82024221277c003dedd827b54e7` |
| 51 | trauma | `ui_trauma.png` | 253466 | `6b7454df1f559317bceaaf0e30001201adcac8a24986752e413719ed10f5eeb4` |
| 52 | bonded_briefing | `ui_bonded_briefing.png` | 200736 | `7efdda5203d72a976dc69d094e6d52ab072f4af4716be7f9af24c795f35c7a19` |
| 53 | legacy | `ui_legacy.png` | 250160 | `237a365b3a727662f554f28debd85cfb6eaa6cdc8b2d3a00433e6dda3a9413e4` |
| 54 | briefing | `ui_briefing.png` | 209843 | `4a12ae0824824850ec6b72e929527f0a5c2b30db439c8b44b7fda52ccbe10562` |
| 55 | recruited_briefing | `ui_recruited_briefing.png` | 211910 | `65c49338104f4e5822ae047eceb61d6bbf56371960e7934bc71e09d5eca178b8` |
| 56 | threat_briefing | `ui_threat_briefing.png` | 190772 | `5eaa1dc05b7b3ab358a5bd99c021dd95317b480a5d647e43a7f11ef021866091` |
| 57 | loadout_briefing | `ui_loadout_briefing.png` | 199670 | `b32a3d06f893bb83d941dd0dce28da1930c1060e03b7c49e2194992c23d98cff` |
| 58 | pressure | `ui_pressure.png` | 205013 | `139498efbec4181c92d3756302b12f5914e7ad8d285249b9055b9a72cded905f` |
| 59 | gameplay | `ui_gameplay.png` | 1186812 | `9b1cbe3d28edd2938a0f8e68e6551f2cc27251e2ea591a3af6f4fc0d023bf0c8` |
| 60 | first_hour_tactical | `ui_first_hour_tactical.png` | 1192606 | `8f1b4ee7bea7debedffb7f866a57dd1d02c8c16775d4f0e67cd7e7e94ae459dc` |
| 61 | first_hour_attack | `ui_first_hour_attack.png` | 1111712 | `8bf8ee5022e1dffb4d6488f7e3314224d4af70bc4adacb718835f6f89f78f24c` |
| 62 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1201644 | `0fc6836a9c674d1ad0428905887840a0bdde538d226d6edba51c7bcfb48670aa` |
| 63 | first_hour_objective | `ui_first_hour_objective.png` | 1344070 | `b8c399c1d98f3ef0630651c57d4e006ba91d9c615829682ffccf34577d72aa3e` |
| 64 | first_hour_replay | `ui_first_hour_replay.png` | 1102875 | `cec9895e7067b80c4287b8c0d44d3d66d13fff4e5b3da097817a52e2098301e8` |
| 65 | first_hour_ability | `ui_first_hour_ability.png` | 1192006 | `b5c8736a0fdeb1a0bbf193dd8fa71931a63dd989f7e105aed5f45d70329a04f7` |
| 66 | second_operation_tactical | `ui_second_operation_tactical.png` | 1116602 | `27a467f2e0aba38c83b54741551847d6b80812921df6fcd03bdc94c756375a76` |
| 67 | overwatch | `ui_overwatch.png` | 1189873 | `1c2b562ffd6232c2af1d5334870d132b5b22ed9ba8ff92db2557fada2d7ac479` |
| 68 | brood_ability | `ui_brood_ability.png` | 1245396 | `c35a6a6756b9296ec5d92e183f98bcc2b1faa8aaf9139468fdeed88654da2931` |
| 69 | directorate_ability | `ui_directorate_ability.png` | 1245496 | `caace2d53ac305b8ec0dc9fbd249ca4efe5151df99a239f611fa283a9481da30` |
| 70 | ascendant_ability | `ui_ascendant_ability.png` | 1245768 | `cd16737fc410b9b08546fa3b62b565a6b62a5ff14563b196bc1e6b422a6a8495` |
| 71 | hazard | `ui_hazard.png` | 1251911 | `6b31b87e6dc1ed168e29228becc9dab0ebd99dd348a0e68bdc323b8f0cc0955e` |
| 72 | intent | `ui_intent.png` | 1152911 | `9f104f1b95d53d65e98ad1cf2cd5b37cc7659970a206254ddea96f6b299d5dcc` |
| 73 | action_preview | `ui_action_preview.png` | 1114234 | `c43ffbaf9c562829b3b81e4db76df757d7b57604682f22f6c29721e5d7de365f` |
| 74 | movement_route | `ui_movement_route.png` | 1148196 | `a8c859ddd20d5c5366916fb20d5e025a72834d6948f9be2e0c478c604ecd803b` |
| 75 | cover_edges | `ui_cover_edges.png` | 1187857 | `e133de7f2e3cd953948b10e7c555d86ac5daa58bde5deb0fa727c3b5703cd93e` |
| 76 | invalid_command | `ui_invalid_command.png` | 1218920 | `d7a8a51d29af0eb197e2942630024f10b6acfc9e4dd75d2790968ffff9a2d742` |
| 77 | valid_shot | `ui_valid_shot.png` | 1109645 | `39f244dd49b9da43bf57dd726a6ba043b16a07db7960592ae38333944f9fdf59` |
| 78 | threat_range | `ui_threat_range.png` | 1221897 | `0740835a054337bddbca7af8b08efe5041d81ba73c2b2c171c76f5018ee52679` |
| 79 | danger_reach | `ui_danger_reach.png` | 1218147 | `56b9ddc37cb0c2266b966ae4531158e290a521f69b8ed8ecf625ee18f09fc3b2` |
| 80 | help | `ui_help.png` | 359978 | `3e30fdfa7ca875e8395104db0b590ab152933975a6b21ed03155f51ef5d47527` |
| 81 | first_hour_guide | `ui_first_hour_guide.png` | 215033 | `cedcface632da2d35a30aaa8d0c212a18d4d07639d0c6bb6dfa2270752f594f1` |
| 82 | first_hour_return | `ui_first_hour_return.png` | 878753 | `0f0d13e44b01da3bc62a1a5da05e9b49990ff8bf060f4773cdcc32c95746eec6` |
| 83 | first_hour_promise | `ui_first_hour_promise.png` | 881402 | `19353ca9ae7a2a7488875944d375f1a9bdeec50e04ebac60aefb788a51aa5d31` |
| 84 | first_hour_operations | `ui_first_hour_operations.png` | 923812 | `512451c3a57bc2fc3c4ab36894e423a903243addb4c753693fcb056c4ac4eb1c` |
| 85 | battle_log | `ui_battle_log.png` | 662166 | `23399c0bb6ed08bf447875decc75b8de74b760170137e8967ed10188946dc488` |
| 86 | combat_feedback | `ui_combat_feedback.png` | 1187287 | `71ca21df0cb4536cb6fd3f084601bdc86149db1d752e5d0d9241bcfbd50167da` |
| 87 | phase_replay | `ui_phase_replay.png` | 1096970 | `76242fe615a9e3b9618a099dfa121ea9ed73d8829a4666ff4e7060715079ea70` |
| 88 | end_phase_guard | `ui_end_phase_guard.png` | 1186990 | `b21805a1a31877e3af9a1402be24552a821ca17ffc6a7806db17f80a586f08a9` |
| 89 | readiness_markers | `ui_readiness_markers.png` | 1191013 | `c29d3fa05d13583ee71e4d4701cd8fa9bd8c0561dc3af449de64634b6cda7db2` |
| 90 | vitality_markers | `ui_vitality_markers.png` | 1191437 | `99698a2e5fd97a5a82fbfc6cb16f3b07e8f23ba8dde34f60951a7f3f6d8bb0a3` |
| 91 | extraction | `ui_extraction.png` | 1440374 | `d18d818f26d857ea4de3bd152f88c23c8fd17b21550dabacf4396416e51d9a2d` |
| 92 | variant | `ui_variant.png` | 1239152 | `3d72868a84d48e1b2def33a1b0d36b7e64a34fd5b54a12b4d454311697b4e5d0` |
| 93 | sporefield | `ui_sporefield.png` | 1191934 | `f4ee8884ce368df04d613a6680e0ec614005f507258b955fcca4cef73788164e` |
| 94 | vault | `ui_vault.png` | 1187389 | `5c5972f745be14bf1294d3cc1b9177791abafa06f8a876e765a77735df1ead11` |
| 95 | black_channel | `ui_black_channel.png` | 1195631 | `6089dca21f5de0364463d13c9526237a7a7962b5830d3f9fb54ac6022e72cafd` |
| 96 | living_chorus | `ui_living_chorus.png` | 1200199 | `fc2749449b236b81d249287eea6aa7022982abf156543abfdfbd28b6dfaf9330` |
| 97 | open_circuit | `ui_open_circuit.png` | 1199040 | `583f59628b88d3a1efe85527449c66f099ce9aa9b923c4a481352434e36f2d36` |
| 98 | trace_active | `ui_trace_active.png` | 1162893 | `21cbc66c29a7c2682e0e808b3368205cbb19582339ce3c00c6ea19dbf13373f4` |
| 99 | equipment | `ui_equipment.png` | 1022218 | `fc4f08a4486324e87c6975ad57929e4670b1f9f68e36cea5d2be2ea09ff0d92e` |
| 100 | weapon_profile | `ui_weapon_profile.png` | 1186911 | `529739b5ecf83b35a649d03330f1f43f3f2d3a475962437f52ad8016a11f438f` |
| 101 | class_target | `ui_class_target.png` | 1039360 | `0baee8c030f96f76e61db5e2586d9ce42b17c5183bf8fa54af691eba174b241c` |
| 102 | breach | `ui_breach.png` | 1363151 | `84aca4dcd785a558f98c983f9bc2dd4439d036babb3fa5a228a5b7ad1a64587c` |
| 103 | debrief | `ui_debrief.png` | 218691 | `eff866c5baeada5255e2346338ec9bda67d5340457624497a2f2e3995efe0600` |
| 104 | trauma_debrief | `ui_trauma_debrief.png` | 202128 | `5c36876217325d535f5d866ee9ce65a9f047dd455c7250ea84a6c2eb40e57477` |
