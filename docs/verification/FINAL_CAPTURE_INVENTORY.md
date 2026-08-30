# Mirexis Final Capture Inventory

Generated: 2026-08-31
Capture root: `docs/verification`
Manifest: `scripts/capture_ui.ps1`

## Integrity summary

- 86 manifest scenes and 86 canonical root PNG files; no scene is stored in a capture subfolder.
- Every capture uses the standard 1280 x 720 logical target.
- The harness overwrites each canonical `ui_<scene>.png` file in place.
- Tactical inspection covers default gameplay, movement routes, class targeting, valid shots, hazards, and the off-center zoom variant.
- Colony inspection covers the default settlement, construction placement, damage, repair, and power loss.
- `capture_audit.json` records automated size, duplicate, diagnostic-color, and visual-detail checks.
- The acceptance-document test fails if the manifest, screen matrix, and this inventory drift apart.

## Spatial acceptance evidence

- `ui_gameplay.png`: default tactical camera with a cropped 40 x 40 battlefield and readable elevation.
- `ui_variant.png`: off-center tactical camera with selection, objective, hazard, unit, and contextual overlays intact.
- `ui_movement_route.png` and `ui_valid_shot.png`: transformed route and target overlays remain aligned.
- `ui_class_target.png`: transformed unit targeting and contextual card remain aligned.
- `ui_colony.png`: centered settlement with a visible first-hour coordinator route and undeveloped 20 x 20 frontier.
- `ui_construction.png`: placement blueprint and reserved resources remain visible at fixed building scale.

## Files

| # | Scene | File | Bytes | SHA-256 |
|---:|---|---|---:|---|
| 1 | title | `ui_title.png` | 441092 | `53fb8d28a0f09958be6af54c788d75e9950aca4d25005946f2a8197bfd312f2d` |
| 2 | title_controller | `ui_title_controller.png` | 480500 | `bd9f65b06ab08a86b57d18e5b013ea61a58565618ec1e787dea482e281eea04c` |
| 3 | title_hover | `ui_title_hover.png` | 480446 | `6271e46c99c208b88a3e881de2e66d7b7ba6ad5301df2fcdf65a12353cf622a3` |
| 4 | colony | `ui_colony.png` | 879197 | `0d243d778864aeae362d4cfe4bd31bf7626e3f6b623b989ddf776fa708ead902` |
| 5 | contact | `ui_contact.png` | 393944 | `49062d6925b6f79d448894a6e2c6d774aab321e28825a92f6d923dce89eb3911` |
| 6 | contact_gear | `ui_contact_gear.png` | 249571 | `f80b5bc361f12ed23eb369f1dce1cfd30628c2830ccedc53b1f8663ecb22bd55` |
| 7 | contact_event | `ui_contact_event.png` | 877143 | `b8b4d4330b7052d90ab716c774cd79dbec5634e435557ec5d0f594f2a3359c80` |
| 8 | adaptation | `ui_adaptation.png` | 354242 | `9967ef31d1799de1f599deb6de1ab61097ec56dd444426dd39a3fd869453b00b` |
| 9 | gene_lab | `ui_gene_lab.png` | 200729 | `147c3324791d4a30c03c71966b6d07c8a3c7bf38dc694e92eec9b65084b8f5fe` |
| 10 | evolution | `ui_evolution.png` | 248326 | `730d5bb149016333ee92b12f1352451b02cc9b2650160753568a1250be4cc3b4` |
| 11 | mara_evolution | `ui_mara_evolution.png` | 200415 | `506ef81cb346b9f65bc232618febfab147e81ea97c56283d8b64771ceacb010d` |
| 12 | ilya_evolution | `ui_ilya_evolution.png` | 198705 | `785700dc76ad89352d8372c34e934d521b2ec8cc1b999abfdf4eaf47fac0c94b` |
| 13 | sol_evolution | `ui_sol_evolution.png` | 193087 | `31673e4b689d0eaba0d8f6e82304325da11a2db75dde0bf383df385db6af586b` |
| 14 | nadi_evolution | `ui_nadi_evolution.png` | 205081 | `faeb61d3de95e72cf0c437f1a0a9c8fad3b41edcc9dc8a571d2b8d373bf03fd4` |
| 15 | escalation | `ui_escalation.png` | 362375 | `fcdda7f3b37215bb8f78dc204856a9373d04669b738103d605d7428c85649835` |
| 16 | escalation_operation | `ui_escalation_operation.png` | 216609 | `d30c5ef4dfe568704ace1acc51e35019fb5d25937276dd09cb38073622ed6bf8` |
| 17 | escalation_response | `ui_escalation_response.png` | 403128 | `5096e93d895f727a37a93a043c6dc71f20136c6abed02e6455c2bc3f088396ba` |
| 18 | mirexis | `ui_mirexis.png` | 408395 | `f686d37aed8547ca291015168cd0ebdad1d423ba07237ec838936bf66b16c0c4` |
| 19 | mirexis_path | `ui_mirexis_path.png` | 363082 | `8685ffa4b9568c5ec4e75900f92503a953bd2a08437cdcbe6ee0336e4d75379d` |
| 20 | redoubt_end | `ui_redoubt_end.png` | 403182 | `9c55901367a8cdd4e9a6d557aedda873af630efa5cfa69a2e9104b589dea5071` |
| 21 | commonwealth_end | `ui_commonwealth_end.png` | 415414 | `048463497bb6e61342cf251c487c6745b7bf3869b98ab4c5d3be76a7f1be5a83` |
| 22 | threshold_end | `ui_threshold_end.png` | 391246 | `01ee8c1caa171865dcba344a858ed3750d578b54ad2cd6723092b42972647589` |
| 23 | finale_debrief | `ui_finale_debrief.png` | 199546 | `d1c6a8ccde3d42d24a524f58ca999b578d531d06342c7c1b5766df5166103b8f` |
| 24 | adaptation_operation | `ui_adaptation_operation.png` | 199754 | `db17fd85732ad17fd22683183d6cff245906e9f34a2d1f1892e030ae190b87ff` |
| 25 | glass_nerve | `ui_glass_nerve.png` | 589736 | `ea871ab4e3cab40c71423f383d19ea3f79609cfef78196abaf90e1b30bc0adb0` |
| 26 | three_knives | `ui_three_knives.png` | 590805 | `ea1e44778e98a2dbd29b2d3225ffadc40d6e0a581552fec6cc03a3712733ab83` |
| 27 | reinforcement_warning | `ui_reinforcement_warning.png` | 591141 | `b36e6fc042d4b12ba6a43e09941b895c1a8cbf3e5cc5f7c0f48ae99ad335bc91` |
| 28 | line_formation | `ui_line_formation.png` | 592027 | `2a290a70ba11433e11b5ffd948f35c427961e0b93b20c05f67f4be85a5788bd7` |
| 29 | thin_shelter | `ui_thin_shelter.png` | 595056 | `767bf83b34e185c9e3510e953ed0b88a2402bb5018e023c17a2d004929d8ebe8` |
| 30 | breakwater | `ui_breakwater.png` | 584882 | `a5dc260d79d5b68663749a0c0231d8da8e36a836f6df06255550fba6d1f15f62` |
| 31 | false_heart | `ui_false_heart.png` | 594878 | `e9946434503d8813a9ff545937ac3a3369aa6810527a921c1a0b0773d04843c1` |
| 32 | live_wire | `ui_live_wire.png` | 596645 | `f3a520abacb853e76c7f93b769c559681040b495010484e4abc220c48bd77844` |
| 33 | last_wall | `ui_last_wall.png` | 579223 | `625c9bcf3a67cf739d8974bb6e124799d68c883e91c0cf23fc9d98609975bfd0` |
| 34 | root_choir | `ui_root_choir.png` | 599112 | `ce8936d54dfdd37f5a1f05b234c268f3547d5b1ee63b73a5d15b0c68d0add454` |
| 35 | door_of_light | `ui_door_of_light.png` | 590918 | `883e64c7fb89c806d6fd657e879a3800cd2df7b0aa4ccee1675a09d81718dbfa` |
| 36 | damage | `ui_damage.png` | 377161 | `f5e2aa68e01e044ea0df5e33971f1767756e7aa51d360171470fa2aa2365e5da` |
| 37 | repair | `ui_repair.png` | 384174 | `c5313941a29ef523c36eb93b551cb8b51dcd2aa686fcf1cb4ad99afc1eb7005a` |
| 38 | power | `ui_power.png` | 375422 | `1512d3c917a2e05fec08534d5460b4796fa3c1625e68bf3fdfca962e95def00b` |
| 39 | construction | `ui_construction.png` | 869841 | `a1a129215402f630eeca8f13f11334cf906da2f21a9e9dd7ae92e4e85f14a255` |
| 40 | research | `ui_research.png` | 366043 | `c2e1dd49ba41a89a2866bbf0ea519d2d7ebc3c92c208c5a503a2472aecdaca2c` |
| 41 | roster | `ui_roster.png` | 248310 | `17b7bf38c85c91a72e4c1b2708ca613b9e90b56a39e7656c5136e45e180f0152` |
| 42 | advanced_roster | `ui_advanced_roster.png` | 247069 | `397706441e15f7d69b2e43a856a839ae8149a7f53718d01f3c6b2d45f1c491b7` |
| 43 | relationships | `ui_relationships.png` | 251902 | `c8e8b1ff45d00ea87e1b9dfb6b1fe70af598a383b6a48cac1434731453167df3` |
| 44 | trauma | `ui_trauma.png` | 252210 | `3648cddc67a932214d85ca318e203cf6f0dca3f0bccb51c61a22ffa846445e56` |
| 45 | bonded_briefing | `ui_bonded_briefing.png` | 205833 | `583e7a369da6ac5703d4e365fd3588902bccee118279b841133f280893ca8510` |
| 46 | legacy | `ui_legacy.png` | 250520 | `8d26ff96d52dc5482e713536bb609374c3485dd72663fa42684bee8574b0c8aa` |
| 47 | briefing | `ui_briefing.png` | 213472 | `4b90be39d980dd5a6a420e4e80635756a0a05b3f609e250b8498d224fbaae4c6` |
| 48 | threat_briefing | `ui_threat_briefing.png` | 198417 | `d0c74a5269b3f1084260bdfcdfbe135a99ff43353f199c5bd6e3f5e9cee99e8e` |
| 49 | loadout_briefing | `ui_loadout_briefing.png` | 204768 | `8c88ac12741bb5a5393a3007b08aa192cad1759b33a79a73a815226f79de55a3` |
| 50 | pressure | `ui_pressure.png` | 213058 | `0064f4053ba56ef8d6220e74609add729c8b7779a6e2423a9952cdf821bf08e4` |
| 51 | gameplay | `ui_gameplay.png` | 1116849 | `5b0f4b92e251a3b2dbef4a227178330a1a0c8edf16b4014e4058048454b034bb` |
| 52 | overwatch | `ui_overwatch.png` | 594734 | `0aec17aac4f14e91e889fc544ee3ab4fa0665d8a049b0ce92543869d2502fc29` |
| 53 | brood_ability | `ui_brood_ability.png` | 629299 | `e40545431cb68f7139989524ba8a8fee9e1a85c7740a14c6dfd5123745531553` |
| 54 | directorate_ability | `ui_directorate_ability.png` | 1177444 | `f5c428358d44019d8f150535d24ba92fe1a79c0dbba5e926ba52ef137c9e7a23` |
| 55 | ascendant_ability | `ui_ascendant_ability.png` | 626952 | `cdb6cbf761c903bf78651ac7b56fcf84dd733dd6e9802577291ff6cc1eca8b60` |
| 56 | hazard | `ui_hazard.png` | 1183762 | `322f40b021a9e8c338083268328841f83af37ee08dbf5e100651aa0244be20d3` |
| 57 | intent | `ui_intent.png` | 598712 | `842478e3e51f0ad00c007f0e85e80acbfd29fcc7b14495ca16dae4f68277f430` |
| 58 | action_preview | `ui_action_preview.png` | 599078 | `24f26414df5cdb20668d776400096ff8253d8e95e32e69a31c770d8bc6678488` |
| 59 | movement_route | `ui_movement_route.png` | 577371 | `8d516e3125429e2ac7f75cf20fedbd4b812ef7f3bf7db2799b46e25b274fdb76` |
| 60 | cover_edges | `ui_cover_edges.png` | 592422 | `f66742237ffa0fb6de80679829a57b12a58a1714f98173184b10fb9a44f38c11` |
| 61 | invalid_command | `ui_invalid_command.png` | 621952 | `692c8cf5737031be42a64126dfe508c19b0b1a4e6afbc6707cb2c55394951948` |
| 62 | valid_shot | `ui_valid_shot.png` | 599005 | `365861e22fde44b4b77ce65b33775b40b8a44fa7b7a55e5c6c04da347614bead` |
| 63 | threat_range | `ui_threat_range.png` | 621802 | `1bb02b56a287ec4be4db0aa3ba91af231a58240f7cb6844ff9266d0f48832c77` |
| 64 | danger_reach | `ui_danger_reach.png` | 612994 | `bea205b685717599c3cc455640e3231da0514b273adfcde3ca241a12a35ac55f` |
| 65 | help | `ui_help.png` | 332701 | `a1e0f3b1cca548d6bf1425a40587e26d6f098fb1fdbf094365ef89f97a65fe4e` |
| 66 | first_hour_guide | `ui_first_hour_guide.png` | 211550 | `4ec8274af4c4b4b2c88ec224ad1a5b6bf4eec8a74e796824c0a271e970c3a20d` |
| 67 | battle_log | `ui_battle_log.png` | 372152 | `f70c358f9d6ad10037fab7f49bf77daa03d4a7dc21d706b58e6240fc399d152d` |
| 68 | combat_feedback | `ui_combat_feedback.png` | 598840 | `b6999d774ec6003e1f667751964ae182931db601974e214b10b08b765bc6c840` |
| 69 | phase_replay | `ui_phase_replay.png` | 559533 | `c8eace3ed8ec5c0736af49d223accd1021977243b20876553edd0b389c9dcf53` |
| 70 | end_phase_guard | `ui_end_phase_guard.png` | 590620 | `45c9d9c7c49b51d2d90fbb6ed503a17f4f629c86b1b1818dbbcde9e6601802f5` |
| 71 | readiness_markers | `ui_readiness_markers.png` | 593135 | `90486f3d5aeba58d4cec38751bfd775d4ec29e5a74b8c91230b57c887fe0cf48` |
| 72 | vitality_markers | `ui_vitality_markers.png` | 593589 | `730eac87effecdfcc2bffb9eb1a12a548955744c49474ea8ef207ff335570d4b` |
| 73 | extraction | `ui_extraction.png` | 727213 | `0afcf722a4e0814e3b8778de1758bf03945b06f545aa846fa29112690062b61e` |
| 74 | variant | `ui_variant.png` | 705792 | `2b2462e3b0f39dcea2a8e17e988af3bc55d813f5ec9e56c36a7af02d85aaa6e9` |
| 75 | sporefield | `ui_sporefield.png` | 594401 | `21f9c6d36b47719e0efeadfa9d5b262fc898f294177768a73f48d76810896c64` |
| 76 | vault | `ui_vault.png` | 591250 | `fd063591d11ba7a91cd902e1e7fe9bdda82fb77a71a100d201695da77e2ac9d4` |
| 77 | black_channel | `ui_black_channel.png` | 597678 | `2737a53b3b0f6b226e46603849cd8f4e7f6ab8e1707042a7ff66096129f994b5` |
| 78 | living_chorus | `ui_living_chorus.png` | 609095 | `f51f8624d2256f78be20c83a8c43fe4638d0be018f7f862ad1ddccf75eaca304` |
| 79 | open_circuit | `ui_open_circuit.png` | 601930 | `af4238e5c217832a146ec788ca5b83a7b621d11fe0da4c041122cbf3132f5619` |
| 80 | trace_active | `ui_trace_active.png` | 587206 | `874acdc593f1b132d46f71707decac3148270554a2d98bd68b688a7ba72330e6` |
| 81 | equipment | `ui_equipment.png` | 538359 | `1b0a8b56125e3ff610ae88c8e3bc8ffc433cb1af6a99b83a38bd9c3de8ff4a0b` |
| 82 | weapon_profile | `ui_weapon_profile.png` | 590465 | `c5e6dd50e15942449ebc53432628c432974601dbb16d25b049f6f953bd9002d2` |
| 83 | class_target | `ui_class_target.png` | 545325 | `3c2c5ffa5d23f38e9232227e6487d012ccba5312c8d70f46b79ccd961c3b5fe7` |
| 84 | breach | `ui_breach.png` | 686023 | `c7342bce74a9e6ec5fc09d70b0502404ed33d9fb15ee63b6a9e8f29958850e37` |
| 85 | debrief | `ui_debrief.png` | 191580 | `d6ee367247a56dec453b3de76914f3f15ecb6673bb7987583fdcba8687b84f4a` |
| 86 | trauma_debrief | `ui_trauma_debrief.png` | 203134 | `8f79c52fc2bcc5fd150d3fb074f674849e25bb84ddfb1cf33d30c5756585751c` |
