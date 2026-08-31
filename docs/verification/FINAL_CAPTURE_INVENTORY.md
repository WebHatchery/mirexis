# Mirexis Final Capture Inventory

Generated: 2026-08-31
Capture root: `docs/verification`
Manifest: `scripts/capture_ui.ps1`

## Integrity summary

- 103 manifest scenes and 103 canonical root PNG files; no scene is stored in a capture subfolder.
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
- `ui_memorial.png`: the Operations register keeps persistent scars, recovery records, and character legacies visible with an explicit return path.
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
| 5 | settings | `ui_settings.png` | 229997 | `8990330f8632af3567983018a6ca54c28bc1dd7840378ca47937c152c210ba12` |
| 6 | field_notes | `ui_field_notes.png` | 124583 | `eb1d71533f5d05bcd445b5b4d3f0c3f4c4c3688697fdf182b277c88d47729094` |
| 7 | memorial | `ui_memorial.png` | 132240 | `8fe6d38384a476e6503f9eeddfec3fa190098f18f1ef783c52542f7cc7a1c0ca` |
| 8 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739504 | `83ea540b8e0b77dd164f3ea0e89722e494a77379d3533893e63822edac0d24a2` |
| 9 | contact | `ui_contact.png` | 935791 | `651a9f1b47ef9f31fd7c8518d7928d2e1caa21f5574920465f20ccc8916980f6` |
| 10 | contact_gear | `ui_contact_gear.png` | 250838 | `a1f2bd616f4a8fa0d1e2f861cc74fb77a883f26238ff2e5e9e98702695bb5537` |
| 11 | contact_event | `ui_contact_event.png` | 923460 | `6f768b1ff91ea01534d662b306a45cd39e6cff9fbc5dbef0cff8f237734cbe05` |
| 12 | adaptation | `ui_adaptation.png` | 898160 | `8b490fa90fc8894db1652d20a82ca27f335c83bc807f581afa8257febec5934a` |
| 13 | gene_lab | `ui_gene_lab.png` | 182294 | `89c7bcf0ec5e3b0fc50ce6f64e83a61a22f0d60e0eed67f8434a853bcd7c62a5` |
| 14 | evolution | `ui_evolution.png` | 245447 | `99c0f37dc3cdd108c8df708e894aa62950ed26bd490c2de7553202b8fe5bdb6f` |
| 15 | mara_evolution | `ui_mara_evolution.png` | 180575 | `40af42ef65ccc15dd0ad21bf2b770ac97e17212d40c4d0f70ddffd5cf2426b6f` |
| 16 | ilya_evolution | `ui_ilya_evolution.png` | 180387 | `7f68f468214e5f1c81555f9d2dce1475c4b656cd19a10dc656a75411f5d06a9a` |
| 17 | sol_evolution | `ui_sol_evolution.png` | 175088 | `c9c0fbf2d1fb24ab2cfec64aa41d00661a40bb53307eb7f7c413a93e8d765438` |
| 18 | nadi_evolution | `ui_nadi_evolution.png` | 185705 | `1b342f768cf9a2250b870ee64136b6b49109c0e01bbeb7fd095eb2d6dbf3f20f` |
| 19 | escalation | `ui_escalation.png` | 900870 | `78387ba2dab079c9d923dbf5b2b2995f9116aea2b677f19c948506aeaeb7775b` |
| 20 | escalation_operation | `ui_escalation_operation.png` | 205069 | `e7cae3dff966be9a9325490ad8f9ad3b9b82dd6f9b82291cbb38bdf182d02320` |
| 21 | escalation_response | `ui_escalation_response.png` | 942844 | `f6c5f8b6b9c8bf234e6a70d994c45b4c046f2b115f93a1fdef009a903b9da1aa` |
| 22 | mirexis | `ui_mirexis.png` | 943095 | `7c68fef3b5906cb0b4cbc11c0ff2193011f2e2d52fe8d8a5c3969bc7b80d9d76` |
| 23 | mirexis_path | `ui_mirexis_path.png` | 903150 | `19fbadab4297fc715824907e30d6457f45be78365f99b769b3781f203ffeab36` |
| 24 | redoubt_end | `ui_redoubt_end.png` | 922655 | `1d06b4658533cdeb73744a2efb9c2ae55c6ecf56dacaef5877b09d5222c67d8e` |
| 25 | commonwealth_end | `ui_commonwealth_end.png` | 931772 | `72ecc58765ab4a51ebd8a6352b23b43364054df134b2ee04d6febbaaabfef40d` |
| 26 | threshold_end | `ui_threshold_end.png` | 932719 | `245a7370c0b4b8d40e2461d288a366cb07da55c46873eccb278f0e21f47bb67a` |
| 27 | finale_debrief | `ui_finale_debrief.png` | 197645 | `b402a19906425a1f4b7c7afa28a02e8fd5fe46f4cd03c8a6cd085332e4306f6c` |
| 28 | adaptation_operation | `ui_adaptation_operation.png` | 190594 | `eeae0bc5c89771e52b4ce091c4ae39ebabb17eaa819e76047b8d2f2e48a4fba2` |
| 29 | glass_nerve | `ui_glass_nerve.png` | 1187814 | `9d1237152445f0cc9433e0a2e1ca558d7c1cd5f815575fd49575ea4d50edc9b1` |
| 30 | three_knives | `ui_three_knives.png` | 1191863 | `d4b656797af1fb89a0e80f795866b3d32b1a952bff127946464323d0e967009a` |
| 31 | reinforcement_warning | `ui_reinforcement_warning.png` | 1192193 | `af90b832d2fcc587e1a3ed86f2550def9357a7af9fcdb545576b40bc8fb223d9` |
| 32 | line_formation | `ui_line_formation.png` | 1187529 | `976ae4fefb7093691a3b854edf250d0b96fdfbddf3aa4cf9be67cfb58abe14b7` |
| 33 | thin_shelter | `ui_thin_shelter.png` | 1189415 | `d6879f1cc3bc0bc7aaa633b1a22c2e63b25d9dc8e5b3823ea7a0463a3dd02350` |
| 34 | breakwater | `ui_breakwater.png` | 1181710 | `1298cc4531016e5d13ca8ae29eafd001431470dfc96b3762724bece808e22e3f` |
| 35 | false_heart | `ui_false_heart.png` | 1188976 | `4c9423769adb46a98e69a7908e023ca041998614a547d452cf90106b128a2820` |
| 36 | live_wire | `ui_live_wire.png` | 1193343 | `4ef61115d96a79d6df190efdcbc36af0aa9c463a2a02aa0e56f48818031a621e` |
| 37 | last_wall | `ui_last_wall.png` | 1183310 | `a02cb564bb86d3d1e9c8deb07cc93ee33d17d5dc11da82c62d3e7e522f81f5eb` |
| 38 | root_choir | `ui_root_choir.png` | 1193167 | `bdc84444a51661d6cd1a5c378d4e6e2de1ecdf7b1ebb5992fac4e513458f917f` |
| 39 | door_of_light | `ui_door_of_light.png` | 1185270 | `d89a45621e3847a000ed52c554eb0141517e80b003b4829deaf8c8d94e7fe0eb` |
| 40 | damage | `ui_damage.png` | 883826 | `94ed94e960ec290e45c83c49f57ec00a689427468e3314ac92161158dec5e629` |
| 41 | repair | `ui_repair.png` | 890744 | `3379ace9b3a6f1fd51d72a411aaebf849942e2798060b5f1cffcab3f355ec662` |
| 42 | power | `ui_power.png` | 882494 | `42e7e36f3a667e891401c4c98f1e859844af755cf4c4d342f81efcc3e3297d60` |
| 43 | construction | `ui_construction.png` | 884151 | `a7691feb536b793a15dada7a36328a6ed866b66c6d4593b9c9ee19a7c0c56b49` |
| 44 | research | `ui_research.png` | 908981 | `2752b9a66123b1b709ef8f44fab21e549224251cb3d534723f13d2f8c5dff61c` |
| 45 | roster | `ui_roster.png` | 249521 | `475705299d05a802d1e248b24dd60beaa323a9dd981aa8567c7048bfb6b3cbe9` |
| 46 | recruited_roster | `ui_recruited_roster.png` | 282364 | `7231aabd84c71abac0edc00e3dd6d5180e2a8e64c8bd3eeddbf4c71336962baf` |
| 47 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210723 | `8a0334f4113c4cc0844a8a9aed59400a1db433a19bfc88481b6d36123aee1a5e` |
| 48 | advanced_roster | `ui_advanced_roster.png` | 249362 | `632032aeedec1911b7f6635899fdb15099074b0b61a70e2a4fa561f3d83e1b5f` |
| 49 | relationships | `ui_relationships.png` | 251569 | `1b5026d51f71eab73cb0d1572c530f9ca75c3dd8c55ce4d22080c5d933c532fe` |
| 50 | trauma | `ui_trauma.png` | 253538 | `6ef4d645eaaa782694705bc68a632e40ecc218795a72cd99988ee30133b8b8c4` |
| 51 | bonded_briefing | `ui_bonded_briefing.png` | 200427 | `1201768ed09061c735806ac2d1a80c6c2c3eaafc5fa5a37666778bab616d80f6` |
| 52 | legacy | `ui_legacy.png` | 250217 | `534ce4d45b0f63b57321596d283e43ab243aee8d9c119c32505469a854c4f743` |
| 53 | briefing | `ui_briefing.png` | 209604 | `5444301da0d6ddf1c648979b4d3061e22798ff1809e24d0c42d5788f68a52762` |
| 54 | recruited_briefing | `ui_recruited_briefing.png` | 212060 | `ddaea8e7f0692c22e2038a8fbd78361275d52371b8b7ac85c7caf476552c4bd5` |
| 55 | threat_briefing | `ui_threat_briefing.png` | 190469 | `0545951d7452dac0bf44c45b6fde44c185569c6a967ac75bf7bbe54fef78c5e9` |
| 56 | loadout_briefing | `ui_loadout_briefing.png` | 199376 | `0281abeec21acba0588dc322a5fed3a2fb190d489277ff6e629e3d8a32d25d76` |
| 57 | pressure | `ui_pressure.png` | 204653 | `685acdbdefa6c2f5af75923fc17af8e1b54f9c3890d6e6db65654423bd62c00e` |
| 58 | gameplay | `ui_gameplay.png` | 1186921 | `ebd80e3f20bf301f3bed2b0c96556502fe2c603cc87a26ad02a792f9953b3494` |
| 59 | first_hour_tactical | `ui_first_hour_tactical.png` | 1192617 | `7360a4fddc7410af463f47c74ee1d2a714abe06c5bc8231382c2d7c5ef4da885` |
| 60 | first_hour_attack | `ui_first_hour_attack.png` | 1111820 | `8c6c8cd08dbee98fe940ec208e704592e38b473c0b61cda7ff2f2be2bb0c4d5e` |
| 61 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1201779 | `85a95624beac45d8bceb415bd953d130da8babfdd058c05379ff1951487fdcf0` |
| 62 | first_hour_objective | `ui_first_hour_objective.png` | 1344092 | `bba10d9b3646aa7696568a221d0265214cf8a60b77afcba933092690cfe61c10` |
| 63 | first_hour_replay | `ui_first_hour_replay.png` | 1102879 | `1ce1002376913cdb51be634d09117b5147ec3699d23fe040045664cf542a040b` |
| 64 | first_hour_ability | `ui_first_hour_ability.png` | 1192008 | `77da45276ed6db9674ca3cd0e4716c13a1c408798806377f1e76433b5caa0fbf` |
| 65 | second_operation_tactical | `ui_second_operation_tactical.png` | 1116704 | `37885b78e1fb6e7cb13e6128d4715d635d0743245857db361a09193b51fcc273` |
| 66 | overwatch | `ui_overwatch.png` | 1189986 | `f5bbdf9a6ef060a1e94ee17755fdb3b7f31b06501dcfe7fe1b811edc1a677d18` |
| 67 | brood_ability | `ui_brood_ability.png` | 1245401 | `18caa05ebb93ff3905d6ab49dfbd0cb58dab215e7cc8e4204bdae91aa9c5bdac` |
| 68 | directorate_ability | `ui_directorate_ability.png` | 1245614 | `0af0fcca5243f67dc5f61cb8e6883167fd0b511596ff603781d1354d4dd5df7b` |
| 69 | ascendant_ability | `ui_ascendant_ability.png` | 1245865 | `71f2883d822c902ee983fc16a4c79efa13db2e956d4c7d64a7d3c07790f5b33e` |
| 70 | hazard | `ui_hazard.png` | 1252032 | `87c7c98fd63ce29e284aaeda7dd2d11cdcc8192fab1b09682c32d96c146553be` |
| 71 | intent | `ui_intent.png` | 1153093 | `0cb2bf766d7555bbcab0c8bc3f1ad7d71a2043d2cd0faad0e72af3deee6e7d4f` |
| 72 | action_preview | `ui_action_preview.png` | 1114328 | `6b5e7c1a71a3bbe84ce619f80478d5e0831053e9cdf5d7fd047e9a80ecc30e7e` |
| 73 | movement_route | `ui_movement_route.png` | 1148299 | `980aa385322708e7fda669ff1ef3f3758b0efad0b2dd188ebe74fcef5ea67ba3` |
| 74 | cover_edges | `ui_cover_edges.png` | 1187965 | `1095560099cffef8c76fa4e78b889691e1685aed78d2719d8c56fff11e09b7dd` |
| 75 | invalid_command | `ui_invalid_command.png` | 1219078 | `d5d6a2aa2ba232433b87425bca63df9bf4187c52c10def57ab45233c80ca6f30` |
| 76 | valid_shot | `ui_valid_shot.png` | 1109809 | `cff0d209c890d2ee300561f9bd51d3bfc71ef2a355b876c4f90d88a74040ca31` |
| 77 | threat_range | `ui_threat_range.png` | 1222067 | `84146877f437ec3282140cd766a24473e0120b987ab575072a8fa8fe87588705` |
| 78 | danger_reach | `ui_danger_reach.png` | 1218320 | `5a385d35d90b3afd41b813fb4332072dbcd6451e484f93aaa426f6d52c20b8fc` |
| 79 | help | `ui_help.png` | 360276 | `631b265bd344d34ee5e4594e0c58781d36a3ca0c6a24201b67d3e2efa21fe703` |
| 80 | first_hour_guide | `ui_first_hour_guide.png` | 215325 | `d9b590da6f79bc56432eb6b17a6930a8cb0be6274f0986efb9948e04a1295538` |
| 81 | first_hour_return | `ui_first_hour_return.png` | 878773 | `fd0dc04443a8997f75d5df52ccb52be4dd81f50ef99317054f2085f570c9fb38` |
| 82 | first_hour_promise | `ui_first_hour_promise.png` | 881405 | `44fd622d618faf77ff7d1e85fda005e596bb9a65978d0b4a48a3d0df535261bf` |
| 83 | first_hour_operations | `ui_first_hour_operations.png` | 923592 | `9ad0577dc4f0f783f39c272fb502471cc8430ab55a3ad856f45bd327f89bc744` |
| 84 | battle_log | `ui_battle_log.png` | 662286 | `59f4692fbfd11c1dba6110b7292831add5ae8944845aa3bb57aa632a39e819be` |
| 85 | combat_feedback | `ui_combat_feedback.png` | 1187398 | `0ac4a186b73eeb365329bbdeb38f13c0e4ae305e8007471ec590b413d90d7320` |
| 86 | phase_replay | `ui_phase_replay.png` | 1097082 | `b64f794d0df68e4422310c3ec8a985f2d93d18c6272bcc887e9bb9f012ca4f35` |
| 87 | end_phase_guard | `ui_end_phase_guard.png` | 1187099 | `218188d7d2aa08a1a5af98a272f214ee2baeb87ef98252bc57e12eb0594827ee` |
| 88 | readiness_markers | `ui_readiness_markers.png` | 1191113 | `ee379d5b5d4d329e5c4b35d1025aec3fdee72cdd935fa425f45d04e05e24379e` |
| 89 | vitality_markers | `ui_vitality_markers.png` | 1191545 | `07212c2eb73a7e7e5cc3057b97b4872a7fd2e7b7e64435a8b8409dfb11c43891` |
| 90 | extraction | `ui_extraction.png` | 1440504 | `1702aa39ac7e72e8752c4e5f00d76110e8059ef20df91c864f34509767afa854` |
| 91 | variant | `ui_variant.png` | 1239265 | `aeb1fca2177e6c2976c10e7e8f4c3f366409a7cc2dda764a47389fe9a3b04277` |
| 92 | sporefield | `ui_sporefield.png` | 1192057 | `acf612c1bd78b46d191feb8c16333b47d517c4465185b06d0af3fb4b19bf9a89` |
| 93 | vault | `ui_vault.png` | 1187496 | `ae9bc2c748f39b2ea4a6edaa2e641192a8677712a5dac7473fe1839220683b6b` |
| 94 | black_channel | `ui_black_channel.png` | 1195753 | `57d4efb52856a77ff2d02fa07285597e8b7a1ca7403fb3aa7a2caff5bf1810ac` |
| 95 | living_chorus | `ui_living_chorus.png` | 1200231 | `ac3078b5285b2a14e2a09a611f0a02f6a812568b0fe8255edebd3bd8b6364149` |
| 96 | open_circuit | `ui_open_circuit.png` | 1199173 | `d8b5d6be79fd06350a87f9960eef670f87b62bbb966ec4cde7b6fe30956a5514` |
| 97 | trace_active | `ui_trace_active.png` | 1163008 | `e913c2502b30986b34892cf115cb38066a7c6a5df0ca0567163e9e626cf3bb6b` |
| 98 | equipment | `ui_equipment.png` | 1022311 | `b606fe018168bc24b84b1934cd767c49e5c767b4d457e36548e1970f6f04d449` |
| 99 | weapon_profile | `ui_weapon_profile.png` | 1187022 | `9f4106f3b9418341811f57597433c107a3c371307032f40d36e7349d8aacefc1` |
| 100 | class_target | `ui_class_target.png` | 1039465 | `920317c684c2aba54f3743fd103ac7562b1e408777a30f7dbd66ac5557f16885` |
| 101 | breach | `ui_breach.png` | 1363247 | `5a9274f453e087861b4c42648d8dff348e633ae2e5d59a89d7f09ccaf9da6199` |
| 102 | debrief | `ui_debrief.png` | 218341 | `ff180ca5fb1f345f768e6922f29de5b5d570afbb7d7856c8b870d2ffc9b4498c` |
| 103 | trauma_debrief | `ui_trauma_debrief.png` | 201693 | `3f6029712d04ea8c530fa6a1ebc29d57bd91ae9c0620bb777c53d80801c02f08` |
