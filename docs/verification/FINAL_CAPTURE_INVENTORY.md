# Mirexis Final Capture Inventory

Generated: 2026-08-12  
Capture root: `docs/verification`  
Manifest: `scripts/capture_ui.ps1`

## Integrity summary

- 85 manifest scenes and 85 canonical root PNG files; no scene is stored in a capture subfolder.
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
- `ui_colony.png`: centered settlement with a visible undeveloped 20 x 20 frontier.
- `ui_construction.png`: placement blueprint and reserved resources remain visible at fixed building scale.

## Files

| # | Scene | File | Bytes | SHA-256 |
|---:|---|---|---:|---|
| 1 | title | `ui_title.png` | 477663 | `bcad0271757d23acd3c9dabca4a29c0ae1a7aa6afd243fcb664a6971c14a5cff` |
| 2 | title_controller | `ui_title_controller.png` | 480500 | `bd9f65b06ab08a86b57d18e5b013ea61a58565618ec1e787dea482e281eea04c` |
| 3 | title_hover | `ui_title_hover.png` | 480446 | `6271e46c99c208b88a3e881de2e66d7b7ba6ad5301df2fcdf65a12353cf622a3` |
| 4 | colony | `ui_colony.png` | 373650 | `92c1e023ebcac30e22b1a4bb2539d4bfe9e20b73fb56f180a76bc39dd5c481a4` |
| 5 | contact | `ui_contact.png` | 391555 | `1cd51c1b8fcc99fb990f996381f9db9221e5b997ba31e7038153c20d312bc8fc` |
| 6 | contact_gear | `ui_contact_gear.png` | 248955 | `49f22966710ee8ff53ee84feacf85e751ee236310245cfe9ad0a94225d062b24` |
| 7 | contact_event | `ui_contact_event.png` | 376401 | `f0931ac5afb3c51715c2719066c02bb01c17397678efddf3c825173726a29227` |
| 8 | adaptation | `ui_adaptation.png` | 352263 | `61e91c3364520712d340b5fc6be62c5e302ffc6df1ca0bd3c041a1394f96b7e1` |
| 9 | gene_lab | `ui_gene_lab.png` | 202388 | `9db3e7c2a77b87613829919a19777467de15b509330081d06f27154b3f8fa980` |
| 10 | evolution | `ui_evolution.png` | 247724 | `e70bb1a675be6a67989cdb0cc3499922fee7e72d1963fbaf800f95be520da63d` |
| 11 | mara_evolution | `ui_mara_evolution.png` | 202093 | `b40d05fdfb8ae5ca55f5fd5853c5699271646b6994cee51489b114b7413e59b1` |
| 12 | ilya_evolution | `ui_ilya_evolution.png` | 200355 | `6c101e0abb6f2604234cfb20ceeaa3228195c3c729ba961b6e5eeb71c397c429` |
| 13 | sol_evolution | `ui_sol_evolution.png` | 194747 | `a9bf61505c4808a0db23547a1f277fb2b2a2443a02276906eb4bb8608d266e51` |
| 14 | nadi_evolution | `ui_nadi_evolution.png` | 206735 | `b800b898511ead76988bce21be7789ea665de849b59d2deb70a284504186a48c` |
| 15 | escalation | `ui_escalation.png` | 361335 | `1bcefa07fc513d759db3e74b05bedd1994a975410ed96c4fe44669e19cfcf06c` |
| 16 | escalation_operation | `ui_escalation_operation.png` | 217618 | `3616d6d99726fde7bd75a5ea6411aa6f46924f53c9bdd0d5107cadbfb1e2cc3e` |
| 17 | escalation_response | `ui_escalation_response.png` | 401834 | `f058affd2adfe4aec3def0a6e39788b6dc5a07d5697e0d143ea825113eeceefa` |
| 18 | mirexis | `ui_mirexis.png` | 407703 | `4654fa6fd82bff0ffe32b2174cfc4854c808e1e42f8c270a340eff4ae7a519ff` |
| 19 | mirexis_path | `ui_mirexis_path.png` | 362170 | `2c19929a947a19173493b240106f501a287c7849686411e99697090a8151e41f` |
| 20 | redoubt_end | `ui_redoubt_end.png` | 402359 | `753b01e4a7b4d55dd0bc9c64698bd5231585bca4621f8ebb185ab17fea9417a2` |
| 21 | commonwealth_end | `ui_commonwealth_end.png` | 415088 | `b19443ade3dea768830c779cf4261466c616bb9604c4c513396e6f1a484885d2` |
| 22 | threshold_end | `ui_threshold_end.png` | 390398 | `f1d83b21bff10776a5beb62a1f3c04787c27cca6c88b6ac7e742622dfbc53d64` |
| 23 | finale_debrief | `ui_finale_debrief.png` | 199169 | `2ad869e5093f36933d7971a2a373b47c1af6b82de4f14626ec6300993e8c1428` |
| 24 | adaptation_operation | `ui_adaptation_operation.png` | 200811 | `ef9a9709f235443a89ca4082aa18477f10c3898e6f6bef4b619e5101a0644433` |
| 25 | glass_nerve | `ui_glass_nerve.png` | 595638 | `edce61ed7fede7e4d1660931f1eeea99218b3eb2c95f455aca6ea780d93c023e` |
| 26 | three_knives | `ui_three_knives.png` | 597448 | `14d27d772f713230cf6a81217f045bcb0f6130e0f6f0e83ecadb06dcf8f350cf` |
| 27 | reinforcement_warning | `ui_reinforcement_warning.png` | 597778 | `7b624d2e9eec69439507cbc6282e5e020637dd1ddbf591ec286fd9bc7b9d2054` |
| 28 | line_formation | `ui_line_formation.png` | 598984 | `9d35e2a6329073684023b424af3d7022bb3f515b28de9bbb225a56d92fbf72ff` |
| 29 | thin_shelter | `ui_thin_shelter.png` | 601238 | `3b7c69b3c5f08c146508ecd6e1160bc5386aeca5e23df41b7a3f65ff0a0791e4` |
| 30 | breakwater | `ui_breakwater.png` | 591157 | `b5a008e94628ff616a7bb58fe8dd3912295310ef45ceebd89027a32690594ac5` |
| 31 | false_heart | `ui_false_heart.png` | 599360 | `6a3f5092e90f7e451410b3cb7b6e2a6e97a311aff263bc27c9d466e7c292c3cd` |
| 32 | live_wire | `ui_live_wire.png` | 603534 | `cfa68e8f4a5706f42b35404bfaf380b5adaf5c0628ab86b7c4ed24d37f681199` |
| 33 | last_wall | `ui_last_wall.png` | 585982 | `91648c897f3ffe29a6ed715eaa0823e61c78104de4d3a463c0f109faa7d5b21d` |
| 34 | root_choir | `ui_root_choir.png` | 605932 | `b909649fa32d360a3e7ca2289b784901c731a8364151f6372092f245d99cacdc` |
| 35 | door_of_light | `ui_door_of_light.png` | 597082 | `2c8242095e5dafa6045af58c9e7512e367c853b810e8439e9f6c9b11ab277f38` |
| 36 | damage | `ui_damage.png` | 374447 | `9d38d9ca5b33a762c1ddf6525685afeb8c85d1e20a13a3408d499624fe9c279d` |
| 37 | repair | `ui_repair.png` | 381034 | `e6564f089a12f1ea5af612c6563eacd8a499c5e07e980f6a72b042a265ff751f` |
| 38 | power | `ui_power.png` | 372161 | `82ea8b1b6d5cde6ffccffa641fac312f6a1891c0f0554912f220b6ca20fef578` |
| 39 | construction | `ui_construction.png` | 377454 | `7c8d0ca27ee9f590825fc20e0b0abb9b8b17f93026f447bc5ce428959ec2a567` |
| 40 | research | `ui_research.png` | 363093 | `89ac8cf7f1ccf6636279b1ac9aa00a76ff1f6602dd59230fb365c3d33d9e838b` |
| 41 | roster | `ui_roster.png` | 247697 | `f8579212992dcba3f15b59cec46cee55e9aa48d64724786974c6461f0c41001f` |
| 42 | advanced_roster | `ui_advanced_roster.png` | 246527 | `cac6e7d1a343c99f5c59ede4ccdeadfd1e09486429b46eecae10b6ecd3fcc6d6` |
| 43 | relationships | `ui_relationships.png` | 251292 | `15b5a439c9af3268a38b4320ab33546758fc60d72575c0b93e854112b5affbf7` |
| 44 | trauma | `ui_trauma.png` | 251598 | `472ee0084112e92ff4a26ef914fb7dc522ba7818e35523087f5f2db5f21a726a` |
| 45 | bonded_briefing | `ui_bonded_briefing.png` | 206895 | `baabd3261fb6b305c2197497c1a2e440217dd9f5a76b8b0dd02eb7761dcc413d` |
| 46 | legacy | `ui_legacy.png` | 249916 | `c3e1cd3f0632eb958f5e4e1eee240596d98bb45bddc725900f1db4375d6f8097` |
| 47 | briefing | `ui_briefing.png` | 205115 | `b0ece832f45d20cda002d403249dae01dc17a1f82f67221dd758cbccf942318c` |
| 48 | threat_briefing | `ui_threat_briefing.png` | 199470 | `d32ec8b832c2f4c98128a988e51e86778daf0d08618cdc06a210655ee5aed794` |
| 49 | loadout_briefing | `ui_loadout_briefing.png` | 205821 | `3551b70d455864ab88b533de35401d8548dc23c99e0afe7ff96465a9982a9a47` |
| 50 | pressure | `ui_pressure.png` | 214115 | `fadcdebd0794eb52422403c598a0b1b289ff1fec554fce046eaeb49190bfd7db` |
| 51 | gameplay | `ui_gameplay.png` | 597120 | `c8fb1743f9ae48ff8621dee9072f24a2f4189def60f1df1f57af30cff41da7fb` |
| 52 | overwatch | `ui_overwatch.png` | 601828 | `795643f19bf8978d3ffa2e7baecdaef92a3657f484c82db4876ef49039cd6074` |
| 53 | brood_ability | `ui_brood_ability.png` | 693776 | `dc018d63e23bdbc0e71c980e95372c3b35c40ec1a567f8eafa72584b0cc7f509` |
| 54 | directorate_ability | `ui_directorate_ability.png` | 688354 | `3a123502bb662c85a88766ce751452507fa50f6937707ad41d36fd5e0b6423fa` |
| 55 | ascendant_ability | `ui_ascendant_ability.png` | 691440 | `09714f994392647f0ecd9660d2d6a0751639e6cde82a88a4de42dea870db8503` |
| 56 | hazard | `ui_hazard.png` | 701041 | `6bbf21818de5f2eaa02ad132b3957787329fec31daa91e477dcce43547a811d9` |
| 57 | intent | `ui_intent.png` | 613625 | `3632f04d25b5e93de32e3118783210533ed317c9eeb29163379b1f0cfeffb7c8` |
| 58 | action_preview | `ui_action_preview.png` | 659597 | `e70b553683079f97da666d00d69237efaa4b7086601e15120a48ffebe16f6834` |
| 59 | movement_route | `ui_movement_route.png` | 584903 | `5213e9817e88793dbb5086352db66b37a52d02858a75dee2ea49cff93d797eb1` |
| 60 | cover_edges | `ui_cover_edges.png` | 599608 | `49d19af673af75fad066a2b862112a6640fad1829b04c69422c86a3d69a995a4` |
| 61 | invalid_command | `ui_invalid_command.png` | 689083 | `d6ff9a8d63b569d628a909d659b90bca6ee786481f72addff598a07ef1cd2606` |
| 62 | valid_shot | `ui_valid_shot.png` | 655324 | `9cd663e76aeaedef7f09b8f52146f1835a1f175f709e9a75684d88952752a929` |
| 63 | threat_range | `ui_threat_range.png` | 681977 | `a391ae1bc1b30146569199a042a3220e0745028bcc1dc4c54423296850cbd402` |
| 64 | danger_reach | `ui_danger_reach.png` | 672955 | `ed0dae50db609f60911c15701da03952a3b9d42090dabda6fb7714bfabcd8af4` |
| 65 | help | `ui_help.png` | 330190 | `3af9c8bbd2e58234d4b117fd99093f8ce319aab3fc742e1c7a670f806095fadc` |
| 66 | battle_log | `ui_battle_log.png` | 377193 | `f38fb48b0861f656d2fa681a44b15f6182391fa5c6997ef2a6802404fe21f641` |
| 67 | combat_feedback | `ui_combat_feedback.png` | 605809 | `00d2607e68a61182cb09369d85f36c2af35e78225e351e5edd566c455a71b635` |
| 68 | phase_replay | `ui_phase_replay.png` | 569454 | `ece667e7941a0a3573d0821bc264ddea90f39e24011aab43e7b3086ee0db4f68` |
| 69 | end_phase_guard | `ui_end_phase_guard.png` | 597334 | `bef6a4db6ac7ef8e8a1e8054d2c944569826ba05bfa28e6971986f6a6da1f7ff` |
| 70 | readiness_markers | `ui_readiness_markers.png` | 600843 | `dfe8a9558818427f509c19d2c2e6c9ac02df86ed9fa5fcbb1b4cae9029411ced` |
| 71 | vitality_markers | `ui_vitality_markers.png` | 601251 | `10f63a1ebf32d5c8a5fd3d5ab173e301a366b01c10f9f15d7d132f184e93f6aa` |
| 72 | extraction | `ui_extraction.png` | 725586 | `b84edddec5d66f7613c749147e2bffba569fca1c4b30f49b1503ebe4ca08f619` |
| 73 | variant | `ui_variant.png` | 725923 | `cedaaafaaa7d795931da9beea8bea1e0b62880cb29e02edb68cf35a6d1154a72` |
| 74 | sporefield | `ui_sporefield.png` | 602554 | `eed9da0855c8fcbd030a495fee61e5f85c8872b526e9dfb8bf3b632c737017e0` |
| 75 | vault | `ui_vault.png` | 597375 | `56257cbaf3209a474a857dca44c0d4baaf1834901e2444aaf40afc88f0e7be23` |
| 76 | black_channel | `ui_black_channel.png` | 604154 | `ed2caf0635b97220c5b7006ee04cd7453ea1374f774f6c1cfc4a783eef52b76b` |
| 77 | living_chorus | `ui_living_chorus.png` | 615048 | `364095f850a029da9b1812051e073d79f9531291ecf993dde8f8ffb47fe6652e` |
| 78 | open_circuit | `ui_open_circuit.png` | 610762 | `a314c442f35d11e6a85b73948957124a30f6683d3096046bcd4251dbe14eabbb` |
| 79 | trace_active | `ui_trace_active.png` | 594614 | `36dbd7cd423d4fbbf3d9a49c1a13f9b4adac852f86d407ad99c560b2fcc317c9` |
| 80 | equipment | `ui_equipment.png` | 541127 | `a578e21a1a5bfda40d2e7a1cf5b2a17940206643fdeefa002b5ad95b59c172da` |
| 81 | weapon_profile | `ui_weapon_profile.png` | 597187 | `4a2485d99adef1903c8a21202cfe65eb8b56bce1d4357d8b83dcbfa5a92b62e6` |
| 82 | class_target | `ui_class_target.png` | 552305 | `440d7f1b40c14346e68f609c38f227b8b2e6fcb1db918ed14cd98e323c4b38aa` |
| 83 | breach | `ui_breach.png` | 684220 | `d835a567cc701bda9025a02fc1130fcdd45d59069ff9e990afb39024fe76103e` |
| 84 | debrief | `ui_debrief.png` | 195735 | `4990a8de76dd2694877dc7f0d9172e37f9425361ce161e710240f33ed9762015` |
| 85 | trauma_debrief | `ui_trauma_debrief.png` | 202757 | `741df737bd0bc2c157e5e2a1398d3198a8220400f5a3ef7d23f7a62bcef45510` |
