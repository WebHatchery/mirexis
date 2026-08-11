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
| 4 | colony | `ui_colony.png` | 376830 | `217730803fd4ad59a523f7e9040df7bd086ed53e21a368474d585d6495875b71` |
| 5 | contact | `ui_contact.png` | 393708 | `715da5b517341ccad13c259d46975154dbcf7452a8623dc63b82caff34f91639` |
| 6 | contact_gear | `ui_contact_gear.png` | 248716 | `71e0cb8bd19d483a468acc64d43d8462ed50ed187e40fc853d7b8c6ef607dd64` |
| 7 | contact_event | `ui_contact_event.png` | 379274 | `bfdf9aad4c038f6a65af7c178db0b63c7219389b9335aab20eac3fa3017f5596` |
| 8 | adaptation | `ui_adaptation.png` | 354290 | `9cbcdf01ab81393c8867b3d3ad8766f15ad0db24d71322a1b5116b1228cd3e6e` |
| 9 | gene_lab | `ui_gene_lab.png` | 200935 | `946b6e59f2ad3ae00f07b4655a6462ee7d1e5d11b732efc3452646a089d25082` |
| 10 | evolution | `ui_evolution.png` | 248627 | `c20da5d9dd5d45fdf689d68fe8dc575ae21f690d6d59f9e17db102aa045f07e0` |
| 11 | mara_evolution | `ui_mara_evolution.png` | 200616 | `c747d06959a6b02d1accf899cf1b17306b13f2ef6a66e0f693c5698b26d8d9e9` |
| 12 | ilya_evolution | `ui_ilya_evolution.png` | 198888 | `3334495a0381125e7a6277d8c32ea14b6ad7002066857e7f17c489762cb7205d` |
| 13 | sol_evolution | `ui_sol_evolution.png` | 193280 | `7695761c1f102531ae60531287d21d69831939254fd312bb85639f2233366869` |
| 14 | nadi_evolution | `ui_nadi_evolution.png` | 205274 | `37630ad1669bec2342da035133ba7e76a7e2515708f83a901afe216a4a4764fc` |
| 15 | escalation | `ui_escalation.png` | 362371 | `4dca60385c1315e25df418a31da668d1c3323bff58750f56db17a2395b32de7c` |
| 16 | escalation_operation | `ui_escalation_operation.png` | 217652 | `251933e351d1a7a789832a3267bb97277988bcaafc56aa4df769e06ccd80fafb` |
| 17 | escalation_response | `ui_escalation_response.png` | 402884 | `621bdaa843604eccd91d2f9a928e52bcd2dbdfcbc29158e895dcd2eec0d7e811` |
| 18 | mirexis | `ui_mirexis.png` | 408168 | `659212e15a94ed6de8088004a9b4098b578b7d0875dfe5b3409c32ae34820344` |
| 19 | mirexis_path | `ui_mirexis_path.png` | 363078 | `fb79a447920c68bd8ccb0464c6b755471e15b1d032e5a3ce865153acaaf86460` |
| 20 | redoubt_end | `ui_redoubt_end.png` | 402955 | `3ee79a012c1dc930c63dd5f1f7ea4e7826151e291d8c710bc7e61ca769966f55` |
| 21 | commonwealth_end | `ui_commonwealth_end.png` | 415187 | `37d1db40b865886b54d499dee0a63d48c194e097fa0932f1dd94362790dd91c9` |
| 22 | threshold_end | `ui_threshold_end.png` | 391019 | `097f5769a59667a5bc173dc125d003e6dad24ecb2a995ff08db4d79494c6624c` |
| 23 | finale_debrief | `ui_finale_debrief.png` | 199169 | `2ad869e5093f36933d7971a2a373b47c1af6b82de4f14626ec6300993e8c1428` |
| 24 | adaptation_operation | `ui_adaptation_operation.png` | 200846 | `66ceebd20c84174ccd6c17e88a9c8c3ad473a9f9b32263049c046620e3036835` |
| 25 | glass_nerve | `ui_glass_nerve.png` | 589551 | `340d3bc3070fe99160f6ae34288b55e2cccb3ee7f8c2d095135e3752cf20b86b` |
| 26 | three_knives | `ui_three_knives.png` | 590606 | `d48450f136707163d971b601620e6d1ec877db00e39ce78ec45901486d01dc08` |
| 27 | reinforcement_warning | `ui_reinforcement_warning.png` | 590939 | `454bc86973abe3d46089ec5c85994cd2a96c767ef83ede8bd490ba82f368385d` |
| 28 | line_formation | `ui_line_formation.png` | 591842 | `0d2c5f0f815c27ffe5bfb25ecb355a1c7ca3c221f9e4cc4952a095cbaba12008` |
| 29 | thin_shelter | `ui_thin_shelter.png` | 594865 | `b0cf7a86596803a818084a26584a59f713ff671c6aa4c0120f022d1d5d5e015a` |
| 30 | breakwater | `ui_breakwater.png` | 584856 | `d97c24b7a2a4df0fed113e5d2c88bc1a5d6759f4475702e0e966f525fbf7b777` |
| 31 | false_heart | `ui_false_heart.png` | 594669 | `48539d6d860d6e464748d88a7146d5335ce27ed924e580953407168030768564` |
| 32 | live_wire | `ui_live_wire.png` | 596444 | `2124d0eff847992f5c79365692814cd7aca79db8ad3547ccc3418e1a1c897d9a` |
| 33 | last_wall | `ui_last_wall.png` | 578993 | `7fb87c2766335b2400dfeb9b625beb92973cee85770af7c08d5a507fc013d9a9` |
| 34 | root_choir | `ui_root_choir.png` | 598884 | `99c1de7ac253e2d2d28b91892f2fba77f475e2257702d4151821423d5f780099` |
| 35 | door_of_light | `ui_door_of_light.png` | 590706 | `af51852e2fe389c1d37301e95fc7adc576e8e3248e494fb3e7a4e0e8dfaa4645` |
| 36 | damage | `ui_damage.png` | 376934 | `f9b6c3cb6cb4d0f4482ee1755df0e0f8ba53469bcf4bc5145976b35ee32eb115` |
| 37 | repair | `ui_repair.png` | 383921 | `79a6f7f239a6638e20c1447cfc575810fd0bbd4f726e15147d2d89cd4a33babc` |
| 38 | power | `ui_power.png` | 375195 | `0ca2b22eefc22a282e9929754a7cd51dca92485e802e88e1d7d3c049c0324102` |
| 39 | construction | `ui_construction.png` | 380927 | `e867ce136cc9f7ea2c344983a048055be47b8ff978da0ae57ed0e1ff8c751d5f` |
| 40 | research | `ui_research.png` | 365816 | `e40a5fed080d82287b48c7bd8a7afa228a3eea95e14091d7f4b35912a4381d98` |
| 41 | roster | `ui_roster.png` | 248628 | `ab7b20d1cf773328ceca779831f316cc25ca36f636c2f629db88884b7574e8f8` |
| 42 | advanced_roster | `ui_advanced_roster.png` | 247375 | `1b186947c5ea5b66b3b62295520d5c34103c5523975c2c72d55d21033a70885a` |
| 43 | relationships | `ui_relationships.png` | 252205 | `ccc107991e97e8187785da58947276acaa02712ba81607115d5e51cba268d72a` |
| 44 | trauma | `ui_trauma.png` | 252530 | `62764dc8471f4db90543e4f596cd30cd889e79de77ead56b16fa1c445c80dc2f` |
| 45 | bonded_briefing | `ui_bonded_briefing.png` | 205917 | `42b00fa60e24b10da9ae4832612842c3f4cb637610c3e050dc9df4b6326de382` |
| 46 | legacy | `ui_legacy.png` | 249677 | `b53df5beb5aae0a1fd7ab133d51794503d475c8aaa2323326bdc95c605a74d32` |
| 47 | briefing | `ui_briefing.png` | 204123 | `286dd6cd94dc1527a6914d7ef6b7e1920cfbbdd43826c93b5b231695da6b5ba4` |
| 48 | threat_briefing | `ui_threat_briefing.png` | 198512 | `09c543674b46de2c7212542528cb738360035cc233438d3ebda259702bc39a2e` |
| 49 | loadout_briefing | `ui_loadout_briefing.png` | 204829 | `fe775860495ba8ee63e8d5529b66bc32feadca59eafeca48aa961aa4de41b7f7` |
| 50 | pressure | `ui_pressure.png` | 214150 | `971b3d4489798f612ee7a90e3142ac6ae76cc0a6a06bc81d4ddf90428dc2b669` |
| 51 | gameplay | `ui_gameplay.png` | 590212 | `746756b396f36f5a625efdbd486d9b13f45209405d7875f34761d16b0b234e65` |
| 52 | overwatch | `ui_overwatch.png` | 594553 | `9c9c905e085842b24541d7317136e5e3cd9201c9367a2f8749193c2fd409cfad` |
| 53 | brood_ability | `ui_brood_ability.png` | 629107 | `b0181ea1aa5f2574bd626c693c9d4d492814bcf83bfe26ea42b7d99ed46c13fa` |
| 54 | directorate_ability | `ui_directorate_ability.png` | 621870 | `efa08068f846369869873fabd7238b179b437d2bc483e9beddd2509d7ba4b9b7` |
| 55 | ascendant_ability | `ui_ascendant_ability.png` | 626776 | `9a3c56b543491646dff83ce2971ea98706f8867bee7c1efac5dd4bff5c72c1bb` |
| 56 | hazard | `ui_hazard.png` | 633141 | `9ba478d365884ea232e89c5f1193123bd57230689d15b224da2e292885e6412f` |
| 57 | intent | `ui_intent.png` | 598531 | `020cd911941d397e82250ee0fda6370111d26e08881e9546a67a2207d8cafa57` |
| 58 | action_preview | `ui_action_preview.png` | 598836 | `a3ff49effa92c28356ffd1b99867b2d3f4e77929983aa6b2ce0b916c20f90c3f` |
| 59 | movement_route | `ui_movement_route.png` | 577187 | `f2c6aa06ce1786bb0a6ed62779b8050d338dee95e3ffb847eb4d2cf8ab3ae97b` |
| 60 | cover_edges | `ui_cover_edges.png` | 592229 | `4c90252215aeba43eb1e3c78b1bff729c9321cabaa574b7b6286f74992e05554` |
| 61 | invalid_command | `ui_invalid_command.png` | 621738 | `8e1e7be2ac1ffef62cb7c7b3f3024b21d191e7d57d2f41ee29e1d39a6f66ef45` |
| 62 | valid_shot | `ui_valid_shot.png` | 598793 | `84d08c9106f328d51ed6b87f9a142973437297388a0e4601edcac5cf63746d56` |
| 63 | threat_range | `ui_threat_range.png` | 621576 | `f2fa3b72edbdacf8dc08b85a8867f869e3f152d0a066c2321db1a803267a1cdf` |
| 64 | danger_reach | `ui_danger_reach.png` | 612766 | `8320457b0690714cd707aa158ef6e70de38ee1229e711d095ea51e0b06d30567` |
| 65 | help | `ui_help.png` | 332717 | `0191f03e9b73186d936aa0446b7a6f2c08cc3246a6be816fdc10c292c75b67b1` |
| 66 | battle_log | `ui_battle_log.png` | 372008 | `3bbc442f2f8608c8b5cf352fab1a629a467ce8a32563e2bb51592a891544cb79` |
| 67 | combat_feedback | `ui_combat_feedback.png` | 598655 | `10aa17a6d5fcc76f52af19f76576e34f6dbfc3dde810fac30d7f2a9a9fd58a7f` |
| 68 | phase_replay | `ui_phase_replay.png` | 559393 | `84efbf8e41997a20c3017c1b6988b675f05964a48c68faa3082b2c6d85c411e3` |
| 69 | end_phase_guard | `ui_end_phase_guard.png` | 590435 | `d90ed462845a8937fe6f880add4b3f985577e01379eb313ef9fa195535557f11` |
| 70 | readiness_markers | `ui_readiness_markers.png` | 592962 | `5d1aee3884b407762c04adaaeacd2c36463357a2c8e4305cae228ac8bda661df` |
| 71 | vitality_markers | `ui_vitality_markers.png` | 593403 | `c1e432775b1a3b82cc6f4bb41daa78f77585f1485726c31cbafd7580e9e97812` |
| 72 | extraction | `ui_extraction.png` | 727021 | `d9c559ff975796fb9e1487e2381b0d9665e49713d864176d334d8bf1fc7b17e2` |
| 73 | variant | `ui_variant.png` | 705566 | `5fbf827a62eea22b1f3de38f5d82c88cee289c6737ec3d45ba62089c61da1e3a` |
| 74 | sporefield | `ui_sporefield.png` | 594184 | `851b03b3803fc0022fc1d8f7a3a446b7c02a055d78f0dceed73ef428a229b9fa` |
| 75 | vault | `ui_vault.png` | 591068 | `ebbd0541264e8d76c8764f441e890d028347078cdf93fa90ccbf235aa6b40921` |
| 76 | black_channel | `ui_black_channel.png` | 597461 | `72ee0827a3e4a836245fca23e4985f979e28bad40322e74f44f763442652c110` |
| 77 | living_chorus | `ui_living_chorus.png` | 608881 | `39d396ceb5072561c4b8eb5d33f9baf5c8d76262ae80fff80bb4affd96f61cbb` |
| 78 | open_circuit | `ui_open_circuit.png` | 601720 | `553f022ebe2baa2d5201df58ae63d35a40f642129d7584f91ed6aac7d4af9bb0` |
| 79 | trace_active | `ui_trace_active.png` | 586985 | `003c8651ffad0eea8e110aa929735fc62c78a43184f78f7028b5772a74bf74d3` |
| 80 | equipment | `ui_equipment.png` | 538235 | `924203d077cd07fab30c19d38f0c8fcdb2a2f86f0be63242f7a44f32b82de22e` |
| 81 | weapon_profile | `ui_weapon_profile.png` | 590280 | `29dc30638b18e9bd37a5ddb2eaf40b26dc05829d4d6201f33fb255b1d74d9921` |
| 82 | class_target | `ui_class_target.png` | 545216 | `9587e29f47569bf6cd05eab9f35a9dbca2f815da605f861632f7bbc237dd5af3` |
| 83 | breach | `ui_breach.png` | 685836 | `03442630a758f273ae0e12da41c309b1908ddf696c3c96bce6dcfc9f2cb5195c` |
| 84 | debrief | `ui_debrief.png` | 196112 | `a0a45966812d5b309c7431876938f3d304d64882757df64e4a6287a6dade09a5` |
| 85 | trauma_debrief | `ui_trauma_debrief.png` | 203134 | `8f79c52fc2bcc5fd150d3fb074f674849e25bb84ddfb1cf33d30c5756585751c` |
