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
- `ui_combat_feedback.png`: hit, miss, critical, damage, recovery, objective, and cover callouts remain anchored to their affected field locations.
- `ui_construction.png`: placement blueprint and reserved resources remain visible at fixed building scale.

## Files

| # | Scene | File | Bytes | SHA-256 |
|---:|---|---|---:|---|
| 1 | title | `ui_title.png` | 441092 | `53fb8d28a0f09958be6af54c788d75e9950aca4d25005946f2a8197bfd312f2d` |
| 2 | title_controller | `ui_title_controller.png` | 480500 | `bd9f65b06ab08a86b57d18e5b013ea61a58565618ec1e787dea482e281eea04c` |
| 3 | title_hover | `ui_title_hover.png` | 480446 | `6271e46c99c208b88a3e881de2e66d7b7ba6ad5301df2fcdf65a12353cf622a3` |
| 4 | colony | `ui_colony.png` | 879197 | `0d243d778864aeae362d4cfe4bd31bf7626e3f6b623b989ddf776fa708ead902` |
| 5 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739505 | `40555091c98a7a00e51c5536ba43684579e17d55d2ad2aa324d5e6b580515e58` |
| 6 | contact | `ui_contact.png` | 393944 | `49062d6925b6f79d448894a6e2c6d774aab321e28825a92f6d923dce89eb3911` |
| 7 | contact_gear | `ui_contact_gear.png` | 249571 | `f80b5bc361f12ed23eb369f1dce1cfd30628c2830ccedc53b1f8663ecb22bd55` |
| 8 | contact_event | `ui_contact_event.png` | 877143 | `b8b4d4330b7052d90ab716c774cd79dbec5634e435557ec5d0f594f2a3359c80` |
| 9 | adaptation | `ui_adaptation.png` | 354242 | `9967ef31d1799de1f599deb6de1ab61097ec56dd444426dd39a3fd869453b00b` |
| 10 | gene_lab | `ui_gene_lab.png` | 200729 | `147c3324791d4a30c03c71966b6d07c8a3c7bf38dc694e92eec9b65084b8f5fe` |
| 11 | evolution | `ui_evolution.png` | 248326 | `730d5bb149016333ee92b12f1352451b02cc9b2650160753568a1250be4cc3b4` |
| 12 | mara_evolution | `ui_mara_evolution.png` | 200415 | `506ef81cb346b9f65bc232618febfab147e81ea97c56283d8b64771ceacb010d` |
| 13 | ilya_evolution | `ui_ilya_evolution.png` | 198705 | `785700dc76ad89352d8372c34e934d521b2ec8cc1b999abfdf4eaf47fac0c94b` |
| 14 | sol_evolution | `ui_sol_evolution.png` | 193087 | `31673e4b689d0eaba0d8f6e82304325da11a2db75dde0bf383df385db6af586b` |
| 15 | nadi_evolution | `ui_nadi_evolution.png` | 205081 | `faeb61d3de95e72cf0c437f1a0a9c8fad3b41edcc9dc8a571d2b8d373bf03fd4` |
| 16 | escalation | `ui_escalation.png` | 362375 | `fcdda7f3b37215bb8f78dc204856a9373d04669b738103d605d7428c85649835` |
| 17 | escalation_operation | `ui_escalation_operation.png` | 216609 | `d30c5ef4dfe568704ace1acc51e35019fb5d25937276dd09cb38073622ed6bf8` |
| 18 | escalation_response | `ui_escalation_response.png` | 403128 | `5096e93d895f727a37a93a043c6dc71f20136c6abed02e6455c2bc3f088396ba` |
| 19 | mirexis | `ui_mirexis.png` | 408395 | `f686d37aed8547ca291015168cd0ebdad1d423ba07237ec838936bf66b16c0c4` |
| 20 | mirexis_path | `ui_mirexis_path.png` | 363082 | `8685ffa4b9568c5ec4e75900f92503a953bd2a08437cdcbe6ee0336e4d75379d` |
| 21 | redoubt_end | `ui_redoubt_end.png` | 403182 | `9c55901367a8cdd4e9a6d557aedda873af630efa5cfa69a2e9104b589dea5071` |
| 22 | commonwealth_end | `ui_commonwealth_end.png` | 415414 | `048463497bb6e61342cf251c487c6745b7bf3869b98ab4c5d3be76a7f1be5a83` |
| 23 | threshold_end | `ui_threshold_end.png` | 391246 | `01ee8c1caa171865dcba344a858ed3750d578b54ad2cd6723092b42972647589` |
| 24 | finale_debrief | `ui_finale_debrief.png` | 199546 | `d1c6a8ccde3d42d24a524f58ca999b578d531d06342c7c1b5766df5166103b8f` |
| 25 | adaptation_operation | `ui_adaptation_operation.png` | 199754 | `db17fd85732ad17fd22683183d6cff245906e9f34a2d1f1892e030ae190b87ff` |
| 26 | glass_nerve | `ui_glass_nerve.png` | 589736 | `ea871ab4e3cab40c71423f383d19ea3f79609cfef78196abaf90e1b30bc0adb0` |
| 27 | three_knives | `ui_three_knives.png` | 590805 | `ea1e44778e98a2dbd29b2d3225ffadc40d6e0a581552fec6cc03a3712733ab83` |
| 28 | reinforcement_warning | `ui_reinforcement_warning.png` | 591141 | `b36e6fc042d4b12ba6a43e09941b895c1a8cbf3e5cc5f7c0f48ae99ad335bc91` |
| 29 | line_formation | `ui_line_formation.png` | 592027 | `2a290a70ba11433e11b5ffd948f35c427961e0b93b20c05f67f4be85a5788bd7` |
| 30 | thin_shelter | `ui_thin_shelter.png` | 595056 | `767bf83b34e185c9e3510e953ed0b88a2402bb5018e023c17a2d004929d8ebe8` |
| 31 | breakwater | `ui_breakwater.png` | 584882 | `a5dc260d79d5b68663749a0c0231d8da8e36a836f6df06255550fba6d1f15f62` |
| 32 | false_heart | `ui_false_heart.png` | 594878 | `e9946434503d8813a9ff545937ac3a3369aa6810527a921c1a0b0773d04843c1` |
| 33 | live_wire | `ui_live_wire.png` | 596645 | `f3a520abacb853e76c7f93b769c559681040b495010484e4abc220c48bd77844` |
| 34 | last_wall | `ui_last_wall.png` | 579223 | `625c9bcf3a67cf739d8974bb6e124799d68c883e91c0cf23fc9d98609975bfd0` |
| 35 | root_choir | `ui_root_choir.png` | 599112 | `ce8936d54dfdd37f5a1f05b234c268f3547d5b1ee63b73a5d15b0c68d0add454` |
| 36 | door_of_light | `ui_door_of_light.png` | 590918 | `883e64c7fb89c806d6fd657e879a3800cd2df7b0aa4ccee1675a09d81718dbfa` |
| 37 | damage | `ui_damage.png` | 377161 | `f5e2aa68e01e044ea0df5e33971f1767756e7aa51d360171470fa2aa2365e5da` |
| 38 | repair | `ui_repair.png` | 384174 | `c5313941a29ef523c36eb93b551cb8b51dcd2aa686fcf1cb4ad99afc1eb7005a` |
| 39 | power | `ui_power.png` | 375422 | `1512d3c917a2e05fec08534d5460b4796fa3c1625e68bf3fdfca962e95def00b` |
| 40 | construction | `ui_construction.png` | 869841 | `a1a129215402f630eeca8f13f11334cf906da2f21a9e9dd7ae92e4e85f14a255` |
| 41 | research | `ui_research.png` | 366043 | `c2e1dd49ba41a89a2866bbf0ea519d2d7ebc3c92c208c5a503a2472aecdaca2c` |
| 42 | roster | `ui_roster.png` | 248310 | `17b7bf38c85c91a72e4c1b2708ca613b9e90b56a39e7656c5136e45e180f0152` |
| 43 | recruited_roster | `ui_recruited_roster.png` | 286972 | `ee2f662d0c81bcb7278f04a5316091b87fe2868639fdd54534ba1197c41d7a64` |
| 44 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 208872 | `0474a64f46af1c22d44363518b17fe13d93db46bc59c11fdd6e5385dcb32a0ca` |
| 45 | advanced_roster | `ui_advanced_roster.png` | 247069 | `397706441e15f7d69b2e43a856a839ae8149a7f53718d01f3c6b2d45f1c491b7` |
| 46 | relationships | `ui_relationships.png` | 251902 | `c8e8b1ff45d00ea87e1b9dfb6b1fe70af598a383b6a48cac1434731453167df3` |
| 47 | trauma | `ui_trauma.png` | 252210 | `3648cddc67a932214d85ca318e203cf6f0dca3f0bccb51c61a22ffa846445e56` |
| 48 | bonded_briefing | `ui_bonded_briefing.png` | 205833 | `583e7a369da6ac5703d4e365fd3588902bccee118279b841133f280893ca8510` |
| 49 | legacy | `ui_legacy.png` | 250520 | `8d26ff96d52dc5482e713536bb609374c3485dd72663fa42684bee8574b0c8aa` |
| 50 | briefing | `ui_briefing.png` | 210993 | `802a273495c2d1c54bb36456795f95ee2e8c41f7b6e42a9b789b52414f16fef2` |
| 51 | recruited_briefing | `ui_recruited_briefing.png` | 213565 | `115902291d35bb8fa66f937cd5604ab1ed3a25355e8c18325665a4f3ac027267` |
| 52 | threat_briefing | `ui_threat_briefing.png` | 198417 | `d0c74a5269b3f1084260bdfcdfbe135a99ff43353f199c5bd6e3f5e9cee99e8e` |
| 53 | loadout_briefing | `ui_loadout_briefing.png` | 204768 | `8c88ac12741bb5a5393a3007b08aa192cad1759b33a79a73a815226f79de55a3` |
| 54 | pressure | `ui_pressure.png` | 213058 | `0064f4053ba56ef8d6220e74609add729c8b7779a6e2423a9952cdf821bf08e4` |
| 55 | gameplay | `ui_gameplay.png` | 1116849 | `5b0f4b92e251a3b2dbef4a227178330a1a0c8edf16b4014e4058048454b034bb` |
| 56 | first_hour_tactical | `ui_first_hour_tactical.png` | 1187342 | `95b5726b90aa74289382def4a072d0510ae784bc27fa6e3aba470241633858f8` |
| 57 | first_hour_attack | `ui_first_hour_attack.png` | 1104803 | `4c46641fd9714f983eb7db2943f4768c9edf0fee97e174fb0051b4466804d9b2` |
| 58 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1200636 | `5325875368aa81ad0b1273e94ea182771c2e6949fb3955d444038abc911f2b72` |
| 59 | first_hour_objective | `ui_first_hour_objective.png` | 1339572 | `92460f30fd23b5853bf43bd303a7dab7efd473c30372c98da8a540b72af7c6dd` |
| 60 | first_hour_replay | `ui_first_hour_replay.png` | 1098095 | `9ec93d8ae132df386bc4d6ff2ddd0ace1cce0163683a2fabf1e9052d87179dc6` |
| 61 | first_hour_ability | `ui_first_hour_ability.png` | 1186451 | `f8a59f9c1e1eb0614466deffc7c43d5fc5d3d9337654f8ac43a8fcde4edfcf9d` |
| 62 | second_operation_tactical | `ui_second_operation_tactical.png` | 1109695 | `a38e1c228e2d11b3ba075dc59c41f24cf127e68c3d87555816093718fdbfbc8d` |
| 63 | overwatch | `ui_overwatch.png` | 594734 | `0aec17aac4f14e91e889fc544ee3ab4fa0665d8a049b0ce92543869d2502fc29` |
| 64 | brood_ability | `ui_brood_ability.png` | 629299 | `e40545431cb68f7139989524ba8a8fee9e1a85c7740a14c6dfd5123745531553` |
| 65 | directorate_ability | `ui_directorate_ability.png` | 1177444 | `f5c428358d44019d8f150535d24ba92fe1a79c0dbba5e926ba52ef137c9e7a23` |
| 66 | ascendant_ability | `ui_ascendant_ability.png` | 626952 | `cdb6cbf761c903bf78651ac7b56fcf84dd733dd6e9802577291ff6cc1eca8b60` |
| 67 | hazard | `ui_hazard.png` | 1183762 | `322f40b021a9e8c338083268328841f83af37ee08dbf5e100651aa0244be20d3` |
| 68 | intent | `ui_intent.png` | 598712 | `842478e3e51f0ad00c007f0e85e80acbfd29fcc7b14495ca16dae4f68277f430` |
| 69 | action_preview | `ui_action_preview.png` | 599078 | `24f26414df5cdb20668d776400096ff8253d8e95e32e69a31c770d8bc6678488` |
| 70 | movement_route | `ui_movement_route.png` | 577371 | `8d516e3125429e2ac7f75cf20fedbd4b812ef7f3bf7db2799b46e25b274fdb76` |
| 71 | cover_edges | `ui_cover_edges.png` | 592422 | `f66742237ffa0fb6de80679829a57b12a58a1714f98173184b10fb9a44f38c11` |
| 72 | invalid_command | `ui_invalid_command.png` | 621952 | `692c8cf5737031be42a64126dfe508c19b0b1a4e6afbc6707cb2c55394951948` |
| 73 | valid_shot | `ui_valid_shot.png` | 1101894 | `c883fb4168ec56316215807cc23ba452e00c3d7aeeaadfca15766f076df8c9dc` |
| 74 | threat_range | `ui_threat_range.png` | 621802 | `1bb02b56a287ec4be4db0aa3ba91af231a58240f7cb6844ff9266d0f48832c77` |
| 75 | danger_reach | `ui_danger_reach.png` | 612994 | `bea205b685717599c3cc455640e3231da0514b273adfcde3ca241a12a35ac55f` |
| 76 | help | `ui_help.png` | 332701 | `a1e0f3b1cca548d6bf1425a40587e26d6f098fb1fdbf094365ef89f97a65fe4e` |
| 77 | first_hour_guide | `ui_first_hour_guide.png` | 211550 | `4ec8274af4c4b4b2c88ec224ad1a5b6bf4eec8a74e796824c0a271e970c3a20d` |
| 78 | first_hour_return | `ui_first_hour_return.png` | 878773 | `fd0dc04443a8997f75d5df52ccb52be4dd81f50ef99317054f2085f570c9fb38` |
| 79 | first_hour_promise | `ui_first_hour_promise.png` | 881379 | `4b64d7d086f919d7b25a1c0dadfbcb5cb60b67329b3982b6211827ce61dac3fb` |
| 80 | first_hour_operations | `ui_first_hour_operations.png` | 920045 | `e0ce281e92487bdb0b2c5c2a3e603963f26423d427f50b9131e6b6128451f161` |
| 81 | battle_log | `ui_battle_log.png` | 655810 | `3a0d56f8d97647193717f7d0a882db8e0f7111fa2f601700eb7f0d2f744d88c0` |
| 82 | combat_feedback | `ui_combat_feedback.png` | 1193733 | `1f146be467569e73090343ff56310b2a8ea413dba80976c70e5ae30cf3bc8d74` |
| 83 | phase_replay | `ui_phase_replay.png` | 559533 | `c8eace3ed8ec5c0736af49d223accd1021977243b20876553edd0b389c9dcf53` |
| 84 | end_phase_guard | `ui_end_phase_guard.png` | 590620 | `45c9d9c7c49b51d2d90fbb6ed503a17f4f629c86b1b1818dbbcde9e6601802f5` |
| 85 | readiness_markers | `ui_readiness_markers.png` | 593135 | `90486f3d5aeba58d4cec38751bfd775d4ec29e5a74b8c91230b57c887fe0cf48` |
| 86 | vitality_markers | `ui_vitality_markers.png` | 593589 | `730eac87effecdfcc2bffb9eb1a12a548955744c49474ea8ef207ff335570d4b` |
| 87 | extraction | `ui_extraction.png` | 727213 | `0afcf722a4e0814e3b8778de1758bf03945b06f545aa846fa29112690062b61e` |
| 88 | variant | `ui_variant.png` | 705792 | `2b2462e3b0f39dcea2a8e17e988af3bc55d813f5ec9e56c36a7af02d85aaa6e9` |
| 89 | sporefield | `ui_sporefield.png` | 594401 | `21f9c6d36b47719e0efeadfa9d5b262fc898f294177768a73f48d76810896c64` |
| 90 | vault | `ui_vault.png` | 591250 | `fd063591d11ba7a91cd902e1e7fe9bdda82fb77a71a100d201695da77e2ac9d4` |
| 91 | black_channel | `ui_black_channel.png` | 597678 | `2737a53b3b0f6b226e46603849cd8f4e7f6ab8e1707042a7ff66096129f994b5` |
| 92 | living_chorus | `ui_living_chorus.png` | 609095 | `f51f8624d2256f78be20c83a8c43fe4638d0be018f7f862ad1ddccf75eaca304` |
| 93 | open_circuit | `ui_open_circuit.png` | 601930 | `af4238e5c217832a146ec788ca5b83a7b621d11fe0da4c041122cbf3132f5619` |
| 94 | trace_active | `ui_trace_active.png` | 587206 | `874acdc593f1b132d46f71707decac3148270554a2d98bd68b688a7ba72330e6` |
| 95 | equipment | `ui_equipment.png` | 538359 | `1b0a8b56125e3ff610ae88c8e3bc8ffc433cb1af6a99b83a38bd9c3de8ff4a0b` |
| 96 | weapon_profile | `ui_weapon_profile.png` | 590465 | `c5e6dd50e15942449ebc53432628c432974601dbb16d25b049f6f953bd9002d2` |
| 97 | class_target | `ui_class_target.png` | 1035222 | `5b4745bdcdfd79e7e4545409a00556bdb85e17c6caf879b98781ddc869aff5c4` |
| 98 | breach | `ui_breach.png` | 686023 | `c7342bce74a9e6ec5fc09d70b0502404ed33d9fb15ee63b6a9e8f29958850e37` |
| 99 | debrief | `ui_debrief.png` | 195872 | `988f2b868d5c9774f9f84bd48b12cf251cfeadddb94c2f001b39aa5877b5f724` |
| 100 | trauma_debrief | `ui_trauma_debrief.png` | 201036 | `7a21ccb5bbc5c8df237205bd887f48e18a447f6b2a9826d3b612031fae2d7398` |
