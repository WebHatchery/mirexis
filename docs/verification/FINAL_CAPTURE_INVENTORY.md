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
| 1 | title | `ui_title.png` | 441382 | `73809eb66e1b98b672d153d0f58389cc94a03a40bc1d5e594bb350d94c5b241c` |
| 2 | title_controller | `ui_title_controller.png` | 443843 | `157f42abc431917d32605da100c0f246b401f18c2df1aff34bbb40fb0d540848` |
| 3 | title_hover | `ui_title_hover.png` | 441360 | `bb170f8e7c94033f52d5cbaa5c5eaa3d402fe66bae06a846d08d8ed813c6cc60` |
| 4 | colony | `ui_colony.png` | 879218 | `9e93d0119cc887772195e190bcb7410e584d403bc57ea48bcf7928466ac8da86` |
| 5 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739505 | `40555091c98a7a00e51c5536ba43684579e17d55d2ad2aa324d5e6b580515e58` |
| 6 | contact | `ui_contact.png` | 931500 | `8e3374a5350de4f2dd30fa4da96ba86dbd40a565e4bfc20a6fe34ba5502cef82` |
| 7 | contact_gear | `ui_contact_gear.png` | 250838 | `96ff80f0234bb178c9bd189ac69e5348ff7c031831f2b03da9b57ef94586fe80` |
| 8 | contact_event | `ui_contact_event.png` | 918683 | `2623f5943e7dc1d67c6f42dbd4141a608776202bf763fbf4cedadcc5dfc44563` |
| 9 | adaptation | `ui_adaptation.png` | 893395 | `99fce637f750306c6f9e8766197600258f4db54eb2f8b4309f73e045a64375dc` |
| 10 | gene_lab | `ui_gene_lab.png` | 182294 | `b94511d1faf0221cc7ade953d8c540d334b8727cf93ed2fffef487e5620c97a8` |
| 11 | evolution | `ui_evolution.png` | 245447 | `4d916398db63377bbb09d2d4e4547cf61cdd08ac1125ead933e865f1570e6149` |
| 12 | mara_evolution | `ui_mara_evolution.png` | 180575 | `8ebc2be07af7aacf84ac103907b39e61f21128e68df48a44645373fca7027c4b` |
| 13 | ilya_evolution | `ui_ilya_evolution.png` | 180388 | `ceccb6545ac9d1e119176f35f7217b8272ab58093ded156eb679af6186dae421` |
| 14 | sol_evolution | `ui_sol_evolution.png` | 175088 | `d1dd3440570c56e2281d4474ab222694f0c3f68aa1f35a649551b185126aa90b` |
| 15 | nadi_evolution | `ui_nadi_evolution.png` | 185705 | `344a9ce195d3da9ceb1423fe495b7270c01e29107a15ce09286c5256daa9cc28` |
| 16 | escalation | `ui_escalation.png` | 896113 | `46e9c771bcf42083a082a043c6e077318543f52640dce353e0cccf7f8af2fbfc` |
| 17 | escalation_operation | `ui_escalation_operation.png` | 205051 | `bab6032fbff48143bd01938c245598cbeda0d8b3ad8ac78382da813fbd63f233` |
| 18 | escalation_response | `ui_escalation_response.png` | 938046 | `267ee7a25400a9e2996dad3e05ae524817b8f7ad74afd2181402c3f9f0105ef1` |
| 19 | mirexis | `ui_mirexis.png` | 938337 | `e0656b51996c811c58657f386ba5bbf2754bf8a8784b3186cd1548e97d7136f8` |
| 20 | mirexis_path | `ui_mirexis_path.png` | 898392 | `b59a9070f87d656a3556965190319ad5ac7b4c65cc247165cf9485b97d5bd14d` |
| 21 | redoubt_end | `ui_redoubt_end.png` | 917887 | `f8ff2bdc6c3130cdc739acd59f957c2e4656cbef1f9375d94b6822d3738acbe7` |
| 22 | commonwealth_end | `ui_commonwealth_end.png` | 927019 | `b061517fe963fe0b1b6971698152b520f68b03b1782125593e837d232d7748d5` |
| 23 | threshold_end | `ui_threshold_end.png` | 927961 | `749e592b00d71444658be7a1c2861a7831853e2aa288c99f320457b1159a7253` |
| 24 | finale_debrief | `ui_finale_debrief.png` | 197645 | `b402a19906425a1f4b7c7afa28a02e8fd5fe46f4cd03c8a6cd085332e4306f6c` |
| 25 | adaptation_operation | `ui_adaptation_operation.png` | 190607 | `db93aeee7990707263b0b1a36c0436ca1bce0084fa7ce67d764a4bb3affab67c` |
| 26 | glass_nerve | `ui_glass_nerve.png` | 1187814 | `9d1237152445f0cc9433e0a2e1ca558d7c1cd5f815575fd49575ea4d50edc9b1` |
| 27 | three_knives | `ui_three_knives.png` | 1191863 | `d4b656797af1fb89a0e80f795866b3d32b1a952bff127946464323d0e967009a` |
| 28 | reinforcement_warning | `ui_reinforcement_warning.png` | 1192193 | `af90b832d2fcc587e1a3ed86f2550def9357a7af9fcdb545576b40bc8fb223d9` |
| 29 | line_formation | `ui_line_formation.png` | 1187529 | `976ae4fefb7093691a3b854edf250d0b96fdfbddf3aa4cf9be67cfb58abe14b7` |
| 30 | thin_shelter | `ui_thin_shelter.png` | 1189415 | `d6879f1cc3bc0bc7aaa633b1a22c2e63b25d9dc8e5b3823ea7a0463a3dd02350` |
| 31 | breakwater | `ui_breakwater.png` | 1181710 | `1298cc4531016e5d13ca8ae29eafd001431470dfc96b3762724bece808e22e3f` |
| 32 | false_heart | `ui_false_heart.png` | 1188976 | `9423fdaf4005a13d1ffc5ecb2a1956ea1cbc1821d1f7feb6793b64120237e5e1` |
| 33 | live_wire | `ui_live_wire.png` | 1193343 | `4ef61115d96a79d6df190efdcbc36af0aa9c463a2a02aa0e56f48818031a621e` |
| 34 | last_wall | `ui_last_wall.png` | 1183310 | `a02cb564bb86d3d1e9c8deb07cc93ee33d17d5dc11da82c62d3e7e522f81f5eb` |
| 35 | root_choir | `ui_root_choir.png` | 1193167 | `bdc84444a51661d6cd1a5c378d4e6e2de1ecdf7b1ebb5992fac4e513458f917f` |
| 36 | door_of_light | `ui_door_of_light.png` | 1185270 | `d89a45621e3847a000ed52c554eb0141517e80b003b4829deaf8c8d94e7fe0eb` |
| 37 | damage | `ui_damage.png` | 883826 | `94ed94e960ec290e45c83c49f57ec00a689427468e3314ac92161158dec5e629` |
| 38 | repair | `ui_repair.png` | 890744 | `3379ace9b3a6f1fd51d72a411aaebf849942e2798060b5f1cffcab3f355ec662` |
| 39 | power | `ui_power.png` | 882494 | `42e7e36f3a667e891401c4c98f1e859844af755cf4c4d342f81efcc3e3297d60` |
| 40 | construction | `ui_construction.png` | 884151 | `a7691feb536b793a15dada7a36328a6ed866b66c6d4593b9c9ee19a7c0c56b49` |
| 41 | research | `ui_research.png` | 904690 | `c321201331e5cd7bc9180c3a9decacbb2081c8437c7d99540c907d89dea5793c` |
| 42 | roster | `ui_roster.png` | 249521 | `60ecc00c83814d78b3c2aa4c0a73c8242527e4188606ff4ee7dc7a5b0e36f77b` |
| 43 | recruited_roster | `ui_recruited_roster.png` | 282364 | `9f8b014cb1575471a2052b951d112b46415148b9eec48d6c0b4b4048373105c9` |
| 44 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210723 | `c352cda78b5478ae793797d30a8b41579ec89fc260c1089a736aca268d3224c9` |
| 45 | advanced_roster | `ui_advanced_roster.png` | 249362 | `6138b07d6fe803c149b498788fb0178a080c55147a32bedd83be0f64c44c6cc5` |
| 46 | relationships | `ui_relationships.png` | 251568 | `166b5e04d7ef6a1144f0b2fbeaca006c8ad6e6ae9bf090f10cd5e4b0941a324c` |
| 47 | trauma | `ui_trauma.png` | 253538 | `2bc841408b024c7ca7929ff17a525e3f72674df2ffcbb0cfb389fe060ee14c2f` |
| 48 | bonded_briefing | `ui_bonded_briefing.png` | 200409 | `e2e60e54985a88d632552aceb9e6a60a8e53bec754bfbe3fdc2eda1d5940a79d` |
| 49 | legacy | `ui_legacy.png` | 250217 | `5b595f3d554dbe19b9696748859c76941a9a4341f91c9a54cc6faf3a98eaa19a` |
| 50 | briefing | `ui_briefing.png` | 209618 | `ee6e4ad7661b9ebf8ffad7d00850fc90d326c85937b848b0644f011251a25d38` |
| 51 | recruited_briefing | `ui_recruited_briefing.png` | 212065 | `fd90b489d14d0f6a7007a6a5732853e8a10e5ad73ed4ff305607c5fb5130c878` |
| 52 | threat_briefing | `ui_threat_briefing.png` | 190482 | `654df154d7305564aa61728a5483fd586b979d4bec98d39cab234629ff81967d` |
| 53 | loadout_briefing | `ui_loadout_briefing.png` | 199389 | `7fc638da89595d7f192b659a5d74379137f014e062d9b263e8de36fc937cf4e1` |
| 54 | pressure | `ui_pressure.png` | 204667 | `696a06f6aacab488625de1ef8825ebc8897b9835fd74d1a0edb4db5fd5e69682` |
| 55 | gameplay | `ui_gameplay.png` | 1186921 | `ebd80e3f20bf301f3bed2b0c96556502fe2c603cc87a26ad02a792f9953b3494` |
| 56 | first_hour_tactical | `ui_first_hour_tactical.png` | 1192617 | `7360a4fddc7410af463f47c74ee1d2a714abe06c5bc8231382c2d7c5ef4da885` |
| 57 | first_hour_attack | `ui_first_hour_attack.png` | 1109226 | `207d686ade73c7204889d1d1e3ace7f8535486d14192f089c407c414acd20962` |
| 58 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1201779 | `85a95624beac45d8bceb415bd953d130da8babfdd058c05379ff1951487fdcf0` |
| 59 | first_hour_objective | `ui_first_hour_objective.png` | 1344084 | `e34bce8ab5f59b6fe56e1f7f6f414f4a7d8b5e8fb7a6bb57b2a080f7910cc797` |
| 60 | first_hour_replay | `ui_first_hour_replay.png` | 1102879 | `45b7cf6706ec2f5fa6ed5ba24be71ee3d66d711e1dd83227c27a5392ef90480a` |
| 61 | first_hour_ability | `ui_first_hour_ability.png` | 1192008 | `77da45276ed6db9674ca3cd0e4716c13a1c408798806377f1e76433b5caa0fbf` |
| 62 | second_operation_tactical | `ui_second_operation_tactical.png` | 1114111 | `b2944b8ba4bd16cb200f2948a13b5406b2c4eea8ba641661227a9caab164321b` |
| 63 | overwatch | `ui_overwatch.png` | 1189986 | `f5bbdf9a6ef060a1e94ee17755fdb3b7f31b06501dcfe7fe1b811edc1a677d18` |
| 64 | brood_ability | `ui_brood_ability.png` | 1245401 | `18caa05ebb93ff3905d6ab49dfbd0cb58dab215e7cc8e4204bdae91aa9c5bdac` |
| 65 | directorate_ability | `ui_directorate_ability.png` | 1245614 | `0af0fcca5243f67dc5f61cb8e6883167fd0b511596ff603781d1354d4dd5df7b` |
| 66 | ascendant_ability | `ui_ascendant_ability.png` | 1245865 | `71f2883d822c902ee983fc16a4c79efa13db2e956d4c7d64a7d3c07790f5b33e` |
| 67 | hazard | `ui_hazard.png` | 1252032 | `87c7c98fd63ce29e284aaeda7dd2d11cdcc8192fab1b09682c32d96c146553be` |
| 68 | intent | `ui_intent.png` | 1148796 | `7e1384a892035fa1d4dd36fbef9e917312db6edd9d205ed7c73cadd46d65b071` |
| 69 | action_preview | `ui_action_preview.png` | 1111728 | `bf8f7e6cde2acb15b1749b9bedb98dd98d97162ae4fc6f0f9f4aaca519048bc8` |
| 70 | movement_route | `ui_movement_route.png` | 1148299 | `980aa385322708e7fda669ff1ef3f3758b0efad0b2dd188ebe74fcef5ea67ba3` |
| 71 | cover_edges | `ui_cover_edges.png` | 1187965 | `1095560099cffef8c76fa4e78b889691e1685aed78d2719d8c56fff11e09b7dd` |
| 72 | invalid_command | `ui_invalid_command.png` | 1216485 | `1f26b9f3fa21bdd4e9a4f1b7bc755226f5ebd6f0f757003ec6f91ed8ea77de23` |
| 73 | valid_shot | `ui_valid_shot.png` | 1107216 | `e4c166f6b8af8bfbc356f2de7b626191029e54e9878b714d9e63b19c110b0eee` |
| 74 | threat_range | `ui_threat_range.png` | 1219610 | `fb387642fc0bdf6516dd40d941a40b6944edf5b1c8236c8f1fad21c0c2209d6c` |
| 75 | danger_reach | `ui_danger_reach.png` | 1215862 | `716914840273c4151793324e69b7327ddff7755b73f7e2cdcbe8d5612904d8ab` |
| 76 | help | `ui_help.png` | 359975 | `60c29f59b02ed80596d3610e3a747d87d04c3592402137f14130afe0cd39ce2c` |
| 77 | first_hour_guide | `ui_first_hour_guide.png` | 215325 | `d9b590da6f79bc56432eb6b17a6930a8cb0be6274f0986efb9948e04a1295538` |
| 78 | first_hour_return | `ui_first_hour_return.png` | 878773 | `fd0dc04443a8997f75d5df52ccb52be4dd81f50ef99317054f2085f570c9fb38` |
| 79 | first_hour_promise | `ui_first_hour_promise.png` | 881405 | `44fd622d618faf77ff7d1e85fda005e596bb9a65978d0b4a48a3d0df535261bf` |
| 80 | first_hour_operations | `ui_first_hour_operations.png` | 919262 | `ea67c002f84511cbdaf867ae443a9a017ca07861c3cc943d6febde7eac03f6dc` |
| 81 | battle_log | `ui_battle_log.png` | 662286 | `b3d4e6c8b8c9a9a7dc785161175c4066a9fd9062f4b11e0a9237ae0bbe4d7e63` |
| 82 | combat_feedback | `ui_combat_feedback.png` | 1187398 | `0ac4a186b73eeb365329bbdeb38f13c0e4ae305e8007471ec590b413d90d7320` |
| 83 | phase_replay | `ui_phase_replay.png` | 1097082 | `e69bb209709587051abab1fd29fa63520ef286b0ff76e5fa0e9f75f4d9592220` |
| 84 | end_phase_guard | `ui_end_phase_guard.png` | 1187099 | `218188d7d2aa08a1a5af98a272f214ee2baeb87ef98252bc57e12eb0594827ee` |
| 85 | readiness_markers | `ui_readiness_markers.png` | 1191112 | `fd1e864274cb381a0e172e0fda6b4e1f483888648a16b5dcab01cb69facc519b` |
| 86 | vitality_markers | `ui_vitality_markers.png` | 1191544 | `fed7fe86c5034f9bc8fefa616014a5133afcfe2c4879a982e91748fe7bfa62f7` |
| 87 | extraction | `ui_extraction.png` | 1440504 | `1702aa39ac7e72e8752c4e5f00d76110e8059ef20df91c864f34509767afa854` |
| 88 | variant | `ui_variant.png` | 1239265 | `7d6912b4702bab681ccaae7a2e90d0729e990af16edaec95cc654eeb89438118` |
| 89 | sporefield | `ui_sporefield.png` | 1192057 | `acf612c1bd78b46d191feb8c16333b47d517c4465185b06d0af3fb4b19bf9a89` |
| 90 | vault | `ui_vault.png` | 1187496 | `ae9bc2c748f39b2ea4a6edaa2e641192a8677712a5dac7473fe1839220683b6b` |
| 91 | black_channel | `ui_black_channel.png` | 1195753 | `57d4efb52856a77ff2d02fa07285597e8b7a1ca7403fb3aa7a2caff5bf1810ac` |
| 92 | living_chorus | `ui_living_chorus.png` | 1200230 | `f9a47d6d758e600d9c78b28d0188b91a07dc8091d7b9dd206ff427e55a012b6a` |
| 93 | open_circuit | `ui_open_circuit.png` | 1199173 | `d8b5d6be79fd06350a87f9960eef670f87b62bbb966ec4cde7b6fe30956a5514` |
| 94 | trace_active | `ui_trace_active.png` | 1163008 | `e913c2502b30986b34892cf115cb38066a7c6a5df0ca0567163e9e626cf3bb6b` |
| 95 | equipment | `ui_equipment.png` | 1020266 | `445a044d2800ab16e7da08149e918cce2c24e21e226357dbfd71a7e12fa9896d` |
| 96 | weapon_profile | `ui_weapon_profile.png` | 1187022 | `9f4106f3b9418341811f57597433c107a3c371307032f40d36e7349d8aacefc1` |
| 97 | class_target | `ui_class_target.png` | 1039465 | `920317c684c2aba54f3743fd103ac7562b1e408777a30f7dbd66ac5557f16885` |
| 98 | breach | `ui_breach.png` | 1363247 | `005e22b959272afa7af9ef7f4840d72ad7bb83d72baf92ae45449d0d4661c1e6` |
| 99 | debrief | `ui_debrief.png` | 218341 | `14fed24df655c66292009d0d7bd70f5d628f32490a41aafc3794998d06e22a7a` |
| 100 | trauma_debrief | `ui_trauma_debrief.png` | 201693 | `81df01610872790016c035b259de1fa58eb7326e443aef0d4712442b0ed9d382` |
