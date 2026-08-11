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
| 4 | colony | `ui_colony.png` | 376001 | `48a37a0ab09e0da596b82d4eef8854540e423083c48e6fe4a7350fb76935e661` |
| 5 | contact | `ui_contact.png` | 393718 | `2d25a29873d3e4710fa08a608b5e0ee27a84a2b8e0de57fec1702d749f16ba32` |
| 6 | contact_gear | `ui_contact_gear.png` | 248716 | `71e0cb8bd19d483a468acc64d43d8462ed50ed187e40fc853d7b8c6ef607dd64` |
| 7 | contact_event | `ui_contact_event.png` | 378554 | `09d0201eeaa0ff0d2d1d437ee7fb0bb9fccb28adff251b95fc08f02f6ddeb9de` |
| 8 | adaptation | `ui_adaptation.png` | 353208 | `2ceb4e47ecb6a7a71c693fd62dfb671ecb0bc5a101dae5ae996ff4b70e8af79a` |
| 9 | gene_lab | `ui_gene_lab.png` | 202217 | `d56f00963f88145b9cde9d05746647c178818f800a86b5fb4dd292aba3dc6949` |
| 10 | evolution | `ui_evolution.png` | 247486 | `53607749aacd1d80fd2f0fdc4e8701cc628bd54bc0f0ef300a9d4cb094762f76` |
| 11 | mara_evolution | `ui_mara_evolution.png` | 201923 | `d9065e16ca1e21521c8a2f76c2f40e23010624746141e1c7bed46e700a9f1f02` |
| 12 | ilya_evolution | `ui_ilya_evolution.png` | 200185 | `e2856d8f9725bf9ddc5cb8a76e3cfa3562db32b17cc62f8cca53390ff7af111d` |
| 13 | sol_evolution | `ui_sol_evolution.png` | 194576 | `86342e97f330470ef2b1d8a64e2eabc1304ecf85ada6a49d4d75ff0e204f5bd2` |
| 14 | nadi_evolution | `ui_nadi_evolution.png` | 206565 | `084de54f8d78be117e267da6290dbd76209fdebb37725b4b925c2d55b8a6fc5c` |
| 15 | escalation | `ui_escalation.png` | 362563 | `3f6ad8c7e60fac8ee9d8dbeca199b7ec417b0415d36471c17c55311e989de01c` |
| 16 | escalation_operation | `ui_escalation_operation.png` | 217752 | `cb37801a581c22e2fb77d19a903234a096f0eb98f3ecdba10ffc0cf3d044b607` |
| 17 | escalation_response | `ui_escalation_response.png` | 403504 | `9240e06f5ca6d01478601eb6865c61de5c39b654b60d8d92d5ffa18a9c5c0737` |
| 18 | mirexis | `ui_mirexis.png` | 408921 | `8c6fa2e3ffefbb00f3334560c7d726928ae97d11ed649a9636bf39ef9473d447` |
| 19 | mirexis_path | `ui_mirexis_path.png` | 363378 | `e17d44cf1852cd7443b8c098420a0a213ad5ba7d2271dce9dbe974a263424e03` |
| 20 | redoubt_end | `ui_redoubt_end.png` | 403773 | `f63e71ea57365dc01b55660ab583e2c6f6fda58e3aeff0fab8739f879360fea0` |
| 21 | commonwealth_end | `ui_commonwealth_end.png` | 415424 | `922488088facf8522d8f880c7a0c87ec0eba7e02cd1af9302627f1e959f3a126` |
| 22 | threshold_end | `ui_threshold_end.png` | 391241 | `5e1d6d062771af443a58b9afe7e2bd711c59b656fb50d2d87e917ec5633ad616` |
| 23 | finale_debrief | `ui_finale_debrief.png` | 199169 | `2ad869e5093f36933d7971a2a373b47c1af6b82de4f14626ec6300993e8c1428` |
| 24 | adaptation_operation | `ui_adaptation_operation.png` | 200945 | `dacbe841aa1bce0e5d75e3b523c592579b1ebd98fe2961542a8b91cabdd5cb47` |
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
| 36 | damage | `ui_damage.png` | 376492 | `8dc8126c7ad9d603d5002fe971f5558478ef13a7a0466411d965ffdc5e37183c` |
| 37 | repair | `ui_repair.png` | 383385 | `ab15650689e94f86b013667f366f7f7b20b0271727016b80faab62291a3e8ecd` |
| 38 | power | `ui_power.png` | 374533 | `c80e2ea7b3371d9899820dad3473026c046c3d65be332db218fbb4616aa9d05d` |
| 39 | construction | `ui_construction.png` | 379894 | `f648329847b5007a46092556e29237287f48ba8304e4d20f98c58fdccffd62f2` |
| 40 | research | `ui_research.png` | 365415 | `ec2e29ae7dfab4304e08e8841b1768826321d2ed6b1cbac9725c67bd315a50aa` |
| 41 | roster | `ui_roster.png` | 247459 | `a877e6040e227f0f0a5622ca404f64cd63359edc40516fe8ddc25b672f990cf1` |
| 42 | advanced_roster | `ui_advanced_roster.png` | 246289 | `a166e3e0835a5b90f7f692f8729bdb598d8bde30bcf78617a11897be6d7c077f` |
| 43 | relationships | `ui_relationships.png` | 251053 | `be1ce32ee8c1efe67a52a36229796f00af30ed0615f8e8b2738d844bee37e432` |
| 44 | trauma | `ui_trauma.png` | 251359 | `494f9440ead7c57db6ded8a10ea1b12240cb5c8970cbdb76038850b3a23fc09a` |
| 45 | bonded_briefing | `ui_bonded_briefing.png` | 207044 | `67e0252a077e0d595a407fd5591dadc1adeac23297544349f8a512b39020cb20` |
| 46 | legacy | `ui_legacy.png` | 249677 | `b53df5beb5aae0a1fd7ab133d51794503d475c8aaa2323326bdc95c605a74d32` |
| 47 | briefing | `ui_briefing.png` | 205249 | `ca17ba75b0d81367a253158ba2c29ad35589a859bea905de2628821c390bafb9` |
| 48 | threat_briefing | `ui_threat_briefing.png` | 199604 | `5a2d92b5aa0b19a855078082bf644b2559a32f6b8184f80b067404b1e4cfa318` |
| 49 | loadout_briefing | `ui_loadout_briefing.png` | 205955 | `accca357fad9757113d084058dc947b41379037a11d66e76dd65cfc42374cb65` |
| 50 | pressure | `ui_pressure.png` | 214249 | `f9a4bf8b2c439dc2addfb6123adf903ba98e1d5e8910319991c05d5fb813d7e9` |
| 51 | gameplay | `ui_gameplay.png` | 590212 | `746756b396f36f5a625efdbd486d9b13f45209405d7875f34761d16b0b234e65` |
| 52 | overwatch | `ui_overwatch.png` | 594553 | `9c9c905e085842b24541d7317136e5e3cd9201c9367a2f8749193c2fd409cfad` |
| 53 | brood_ability | `ui_brood_ability.png` | 682717 | `a3d1b829563948b58c18aaf2748f4a288ddccfdd273af2d2de9415865f208d81` |
| 54 | directorate_ability | `ui_directorate_ability.png` | 679247 | `97be5770bdd70686f8766bc5f81282f416663ec389d02b65a0d4e203548d63e0` |
| 55 | ascendant_ability | `ui_ascendant_ability.png` | 681606 | `ef54e29d99fa3354f59ead876751d68564f1e14358fd9e49fd2550c3d25f4131` |
| 56 | hazard | `ui_hazard.png` | 689261 | `b87f6217f5af419c463b2dbe263adfb89946d66c38478870aaa40470e5370852` |
| 57 | intent | `ui_intent.png` | 601470 | `c07f5ea3b0961a19daa634d14339eb0e8c13de725d26211ab3f84f7eff4dc9db` |
| 58 | action_preview | `ui_action_preview.png` | 648997 | `f7350c892a132c6dbbdb05f2e849a5b38bc600314a1a4853fb9f4316675b89e7` |
| 59 | movement_route | `ui_movement_route.png` | 577187 | `f2c6aa06ce1786bb0a6ed62779b8050d338dee95e3ffb847eb4d2cf8ab3ae97b` |
| 60 | cover_edges | `ui_cover_edges.png` | 592229 | `4c90252215aeba43eb1e3c78b1bff729c9321cabaa574b7b6286f74992e05554` |
| 61 | invalid_command | `ui_invalid_command.png` | 672650 | `4b4c3097281b5347449d579d4865c40d07ec036bf086bd6d058f61b9418a3f34` |
| 62 | valid_shot | `ui_valid_shot.png` | 645804 | `1af16bf4fe6f5cf432d176864c10cad4a84e60c9ce54b2c5fc6a46be2ff25bfd` |
| 63 | threat_range | `ui_threat_range.png` | 668271 | `0d1687313c74bfd1eb8be7675f0f7edf3e7e44692c74bde45871b24114d2cba5` |
| 64 | danger_reach | `ui_danger_reach.png` | 659876 | `eaf9bc97ccf150d7d3239e589a71bf466af8bf417000af4516c53a426c0d5c10` |
| 65 | help | `ui_help.png` | 329869 | `93fa0976e405e804898220a561aa894f7fcbaf5086337168128ea57f31425ae2` |
| 66 | battle_log | `ui_battle_log.png` | 372008 | `25d6c218125989d845649d76bfb610f500a43ebb0e1c26def559c799712df591` |
| 67 | combat_feedback | `ui_combat_feedback.png` | 598655 | `10aa17a6d5fcc76f52af19f76576e34f6dbfc3dde810fac30d7f2a9a9fd58a7f` |
| 68 | phase_replay | `ui_phase_replay.png` | 562129 | `606ef468d539fa0a00fce711f6a365d853cd3d594733079e707feb4b445b4e76` |
| 69 | end_phase_guard | `ui_end_phase_guard.png` | 590435 | `d90ed462845a8937fe6f880add4b3f985577e01379eb313ef9fa195535557f11` |
| 70 | readiness_markers | `ui_readiness_markers.png` | 592962 | `5d1aee3884b407762c04adaaeacd2c36463357a2c8e4305cae228ac8bda661df` |
| 71 | vitality_markers | `ui_vitality_markers.png` | 593403 | `c1e432775b1a3b82cc6f4bb41daa78f77585f1485726c31cbafd7580e9e97812` |
| 72 | extraction | `ui_extraction.png` | 727021 | `d9c559ff975796fb9e1487e2381b0d9665e49713d864176d334d8bf1fc7b17e2` |
| 73 | variant | `ui_variant.png` | 719074 | `9cb29db76c85dcfd416cae4d83b1269df0c11c422227a4308290019ad9546843` |
| 74 | sporefield | `ui_sporefield.png` | 594184 | `851b03b3803fc0022fc1d8f7a3a446b7c02a055d78f0dceed73ef428a229b9fa` |
| 75 | vault | `ui_vault.png` | 591068 | `ebbd0541264e8d76c8764f441e890d028347078cdf93fa90ccbf235aa6b40921` |
| 76 | black_channel | `ui_black_channel.png` | 597461 | `72ee0827a3e4a836245fca23e4985f979e28bad40322e74f44f763442652c110` |
| 77 | living_chorus | `ui_living_chorus.png` | 608881 | `39d396ceb5072561c4b8eb5d33f9baf5c8d76262ae80fff80bb4affd96f61cbb` |
| 78 | open_circuit | `ui_open_circuit.png` | 601720 | `553f022ebe2baa2d5201df58ae63d35a40f642129d7584f91ed6aac7d4af9bb0` |
| 79 | trace_active | `ui_trace_active.png` | 586985 | `003c8651ffad0eea8e110aa929735fc62c78a43184f78f7028b5772a74bf74d3` |
| 80 | equipment | `ui_equipment.png` | 536938 | `7fc993ea89ee0a92553546c9de1ffeee6db778154a920e19c25227dd91d6e144` |
| 81 | weapon_profile | `ui_weapon_profile.png` | 590280 | `29dc30638b18e9bd37a5ddb2eaf40b26dc05829d4d6201f33fb255b1d74d9921` |
| 82 | class_target | `ui_class_target.png` | 547781 | `27beb2e1aa2af719747141f3bd53ee51d56ab6ef631180d7a87a25270b7c259e` |
| 83 | breach | `ui_breach.png` | 685836 | `03442630a758f273ae0e12da41c309b1908ddf696c3c96bce6dcfc9f2cb5195c` |
| 84 | debrief | `ui_debrief.png` | 195735 | `4990a8de76dd2694877dc7f0d9172e37f9425361ce161e710240f33ed9762015` |
| 85 | trauma_debrief | `ui_trauma_debrief.png` | 202757 | `741df737bd0bc2c157e5e2a1398d3198a8220400f5a3ef7d23f7a62bcef45510` |
