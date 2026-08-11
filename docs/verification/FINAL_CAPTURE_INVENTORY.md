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
| 4 | colony | `ui_colony.png` | 375881 | `8a94714a8a8326d24782c0c1cd48734de679f2211956e63db995fa414b31c61e` |
| 5 | contact | `ui_contact.png` | 393592 | `3938dd7560c069a86e21fd3938a349bd2b3e33d90b5933569bc2b565bcf6f5ae` |
| 6 | contact_gear | `ui_contact_gear.png` | 248716 | `71e0cb8bd19d483a468acc64d43d8462ed50ed187e40fc853d7b8c6ef607dd64` |
| 7 | contact_event | `ui_contact_event.png` | 378633 | `337d9e6034fbe90465711319c612e559d2404b0c366aee2a352afe80b25f9921` |
| 8 | adaptation | `ui_adaptation.png` | 353422 | `1942272d61dea3650d8352426300cb54cbcc278916bccd0076b6b6b1cd5555a9` |
| 9 | gene_lab | `ui_gene_lab.png` | 202217 | `d56f00963f88145b9cde9d05746647c178818f800a86b5fb4dd292aba3dc6949` |
| 10 | evolution | `ui_evolution.png` | 247486 | `53607749aacd1d80fd2f0fdc4e8701cc628bd54bc0f0ef300a9d4cb094762f76` |
| 11 | mara_evolution | `ui_mara_evolution.png` | 201923 | `d9065e16ca1e21521c8a2f76c2f40e23010624746141e1c7bed46e700a9f1f02` |
| 12 | ilya_evolution | `ui_ilya_evolution.png` | 200185 | `e2856d8f9725bf9ddc5cb8a76e3cfa3562db32b17cc62f8cca53390ff7af111d` |
| 13 | sol_evolution | `ui_sol_evolution.png` | 194576 | `86342e97f330470ef2b1d8a64e2eabc1304ecf85ada6a49d4d75ff0e204f5bd2` |
| 14 | nadi_evolution | `ui_nadi_evolution.png` | 206565 | `084de54f8d78be117e267da6290dbd76209fdebb37725b4b925c2d55b8a6fc5c` |
| 15 | escalation | `ui_escalation.png` | 363460 | `09a162c4c829acea5f5e54274ab24ebcdf2899170253406772657c27fcdb325d` |
| 16 | escalation_operation | `ui_escalation_operation.png` | 217589 | `c3287006e561cbb18a7464f8ab132d902aff46946aa6e5df7c23523d1a28d386` |
| 17 | escalation_response | `ui_escalation_response.png` | 403936 | `d6abd421dadc6304cfcb8b136738ca5beee70cd4afdd2816bbc23435eafbc164` |
| 18 | mirexis | `ui_mirexis.png` | 409666 | `de6928825af3a5cbde5342bf84067f6ec8f91b993346e241828b79bf8032fd5f` |
| 19 | mirexis_path | `ui_mirexis_path.png` | 364295 | `3f222b37c8c2b3eb0bb01dd78b8a2fabfd8736e614f4d5ba3a5ff8b1fd94c9a0` |
| 20 | redoubt_end | `ui_redoubt_end.png` | 404484 | `bb116ffe49e49167b6dbb2962375ab0846199ebbb1d4bbf1ee8a2768a8d20650` |
| 21 | commonwealth_end | `ui_commonwealth_end.png` | 417213 | `b672fba8d64a7c435921ca58d90a414954344a2f3c9b963ae86f87546dfa2225` |
| 22 | threshold_end | `ui_threshold_end.png` | 392523 | `be37e752de4c6b3ac9a167785d75efdd6e51485f56e28606a009e04978304035` |
| 23 | finale_debrief | `ui_finale_debrief.png` | 199169 | `2ad869e5093f36933d7971a2a373b47c1af6b82de4f14626ec6300993e8c1428` |
| 24 | adaptation_operation | `ui_adaptation_operation.png` | 200783 | `ec62b8d31c45642c5fb971128be1da9ad20760193cb72e3b32646e4683c83b26` |
| 25 | glass_nerve | `ui_glass_nerve.png` | 597275 | `cf91bbb42ef2a4e2d6cd0531017fbac19e8114c8a15780bdf9f338d397b8c1d5` |
| 26 | three_knives | `ui_three_knives.png` | 599053 | `b60da499e9622eff345cac9fa558c56e339e8dcda5c59fb4c9f1aa9b25748555` |
| 27 | reinforcement_warning | `ui_reinforcement_warning.png` | 599382 | `51c714881ebcd1608c8f2cd563a81fd44a023b0792f4586e4d7d0cb6e30c31f7` |
| 28 | line_formation | `ui_line_formation.png` | 600634 | `29ec572e728fa803bdae6686a8a61da1e46025c063d469627184ae37a508ef1d` |
| 29 | thin_shelter | `ui_thin_shelter.png` | 602868 | `c67673eb859110b89829c7138c10a38b771552bd23b8cddbe7ed8cd147a07dae` |
| 30 | breakwater | `ui_breakwater.png` | 592793 | `422cd8e46753ab5ba71f453ec02353ecad60d3e38977c850e14a20672bc30b85` |
| 31 | false_heart | `ui_false_heart.png` | 601004 | `4fb7c7796cf0cd67de9b297fec501c3b8d9423cd6b2f23bec6181820d024d20b` |
| 32 | live_wire | `ui_live_wire.png` | 605173 | `d779c5f9bfbe9b7a0efc05de32e038f7957a66bb2ec55059b2502ea16b41b27f` |
| 33 | last_wall | `ui_last_wall.png` | 587727 | `de359201859d920c42f7b1f084436233338ab2f43e22ee0fe360c1353c73e83e` |
| 34 | root_choir | `ui_root_choir.png` | 607372 | `f6461991df3763dade4e5568ee4166a49e7887c3a90b2d3ec041ffbed69cbe8b` |
| 35 | door_of_light | `ui_door_of_light.png` | 598760 | `e61d3098b0fad0a074af04fb4fdc89e2fc65ed093f6529aa5265b568d27df857` |
| 36 | damage | `ui_damage.png` | 376678 | `06505581be25a6f1b377e82a93cb0110e2fb1dae2c84809c49c6459f6222d234` |
| 37 | repair | `ui_repair.png` | 383265 | `0b7ae0ce9f42bd7312a59fd938a04aea59b096c5bd0e9beda650183b135320d7` |
| 38 | power | `ui_power.png` | 374392 | `4a89ec7001acca12e962d161e26020a06a89d1495cdb202862fb44f338c7a853` |
| 39 | construction | `ui_construction.png` | 379848 | `a28fb64cedd6b1afe663c4a6dd036a4bde93ef7d66747bd6bb16e529a301b486` |
| 40 | research | `ui_research.png` | 365300 | `7452014256998c1d942d1467d46c6bdf83bde268a48c942ee533dbf315e0719a` |
| 41 | roster | `ui_roster.png` | 247459 | `a877e6040e227f0f0a5622ca404f64cd63359edc40516fe8ddc25b672f990cf1` |
| 42 | advanced_roster | `ui_advanced_roster.png` | 246289 | `a166e3e0835a5b90f7f692f8729bdb598d8bde30bcf78617a11897be6d7c077f` |
| 43 | relationships | `ui_relationships.png` | 251053 | `be1ce32ee8c1efe67a52a36229796f00af30ed0615f8e8b2738d844bee37e432` |
| 44 | trauma | `ui_trauma.png` | 251359 | `494f9440ead7c57db6ded8a10ea1b12240cb5c8970cbdb76038850b3a23fc09a` |
| 45 | bonded_briefing | `ui_bonded_briefing.png` | 206862 | `2c90c317dfb98b353bdf1f75104e8206c506702b33a8c1cee9aa418ab69ead22` |
| 46 | legacy | `ui_legacy.png` | 249677 | `b53df5beb5aae0a1fd7ab133d51794503d475c8aaa2323326bdc95c605a74d32` |
| 47 | briefing | `ui_briefing.png` | 205087 | `1bda863da75b4b1ca09c566b8c16e9f400638b5296aaf8bdf523861f461a1a19` |
| 48 | threat_briefing | `ui_threat_briefing.png` | 199442 | `5443042289d660d7d8dc47d24408b0c6b8543020f88b58d7027af0f0714dca10` |
| 49 | loadout_briefing | `ui_loadout_briefing.png` | 205793 | `7f85dca6f95826927c08286b7229fc5c9db24c67cf6a8ae0a98ab0d862e78c09` |
| 50 | pressure | `ui_pressure.png` | 214087 | `5c264f72a9a38f1279553f3446366b16bc7234a2cb152723cf0f956ce7aa0e62` |
| 51 | gameplay | `ui_gameplay.png` | 598770 | `4ec1ec24f5659ee6492cdf43667baab6dcbc8cf7699f081308b75a871c2eb21a` |
| 52 | overwatch | `ui_overwatch.png` | 603478 | `3ea9e1c576d6c14276f5e810340ea796cfa5a35945b8813332a268c9b4d5af3a` |
| 53 | brood_ability | `ui_brood_ability.png` | 694964 | `65fdcb44fb75323834fecbee70dde05a56b259191f983b65d46b05d74eeeda5f` |
| 54 | directorate_ability | `ui_directorate_ability.png` | 690064 | `e42eaca6ef07c90204ca912725db4eb78eb3151631176119d2d55f5051c7ce66` |
| 55 | ascendant_ability | `ui_ascendant_ability.png` | 692973 | `2147e2191ecab5b21b413693ca8a57cdb9ff0cef03c4e85a846cebb40423a487` |
| 56 | hazard | `ui_hazard.png` | 702719 | `2005a24bd5ee161a3eeef20e51dc6ccf581333990a73b7291ee3d182b04b058f` |
| 57 | intent | `ui_intent.png` | 614936 | `076399316de448c53acc238fe2ffc2bfacd2f1328b292973e9c138e13a7e2397` |
| 58 | action_preview | `ui_action_preview.png` | 660803 | `4e9a3bd3a8ad167ceb16c9f350c4b1fbdeae69eee334d29eb1cd4c5a7c1c1b33` |
| 59 | movement_route | `ui_movement_route.png` | 586551 | `379225f06a8e084acc084c4f7a80f06ef239eda84d2e381cba75cd4cbf7e967e` |
| 60 | cover_edges | `ui_cover_edges.png` | 601258 | `c6d66181d42599e64d8404d0c1b0ba1e95ba20987a6f5478f1ff5e7b922ed122` |
| 61 | invalid_command | `ui_invalid_command.png` | 690602 | `fd5c75eddd54eef90f95781a3f47bee8cdda9a364c997b485e53da61c3edac95` |
| 62 | valid_shot | `ui_valid_shot.png` | 656978 | `ceda70c242cd63d573e2d64848b35d9fc772394914e95dbcd77e49e0c0287ef1` |
| 63 | threat_range | `ui_threat_range.png` | 683499 | `7f823dae5ba9f45174ba14839217e0141f55d45ef053a3ea14254bd4b46c3bad` |
| 64 | danger_reach | `ui_danger_reach.png` | 674477 | `1e377cc0e2ef9612002cfd550eb1b36ec0774a4ffa59925dd4e6c380aca1611a` |
| 65 | help | `ui_help.png` | 331148 | `645e530c4dd15de3e9fe49a08489f0dd53213a391f2b8f0bdc5bccd1709f6b6d` |
| 66 | battle_log | `ui_battle_log.png` | 378085 | `9901e8e2714fdfb28578e573ae2899a67d2f03572bb48818787482c811829121` |
| 67 | combat_feedback | `ui_combat_feedback.png` | 607459 | `e06115c7f814873dca56e33c22f06ef6246312716a6255f9e7ff812564fde20f` |
| 68 | phase_replay | `ui_phase_replay.png` | 571104 | `b2a0e818829407a0495bf81834d80cdbd59148a98801075d6de014b432d677d3` |
| 69 | end_phase_guard | `ui_end_phase_guard.png` | 598983 | `bc62f1c2ad84a7b631ca9eb2ab9a1b9df7be82c5d835dd1faa6701c82e32cc18` |
| 70 | readiness_markers | `ui_readiness_markers.png` | 602492 | `a8ac927f5a1b81c9832bc48e5e847b08eab97b90e4eb62d5fe698bc099863dcc` |
| 71 | vitality_markers | `ui_vitality_markers.png` | 602901 | `eb37e4b39535b0f2cd1f9cdf21623309256b6387ce93c7677c5f9f09d228a362` |
| 72 | extraction | `ui_extraction.png` | 727106 | `0f30fd4874e94fcebd0aca0338ac3bedc0d4f86f2077cc21e55485394a431ef3` |
| 73 | variant | `ui_variant.png` | 727389 | `9bc07a8dd730a25d70ebd7e9675b08afc333f8345377811b93e3bf3ff09ceb10` |
| 74 | sporefield | `ui_sporefield.png` | 604128 | `5eb2d935b050c190ef66ab067f77868578723bce464ac6fe2a2624ddd4b32d17` |
| 75 | vault | `ui_vault.png` | 599232 | `6a1a9db5f52340d9f7819bc7cc98ed94a04a25ccbad2e59e3801ff90f3155043` |
| 76 | black_channel | `ui_black_channel.png` | 605755 | `dc76a43a9e0e11a7d1269e50c9ce9345cf6289ba17feace5c9e3e51ceee75b5f` |
| 77 | living_chorus | `ui_living_chorus.png` | 616610 | `5cdd84a643c8634ef5a47e2bae8a36212a8e7f05abdc48cecf428de9c8bc4f0a` |
| 78 | open_circuit | `ui_open_circuit.png` | 612016 | `7871aef9ceea503aec515f50e5459ad52395fa12fa4887f9f6b9dfb0ce032b1d` |
| 79 | trace_active | `ui_trace_active.png` | 596030 | `a2c73a932abedbef634f2cf965bb153b34006536c7f55b883a94cdbde7ea2d0a` |
| 80 | equipment | `ui_equipment.png` | 542701 | `b18584ea886fbf77d8c5ceb2a60bc1fa54a2173f810cc51739ce4b0f19108001` |
| 81 | weapon_profile | `ui_weapon_profile.png` | 598837 | `caad26bd4f3b8f1424603d32a9478dbc1b60bc4b78d222a2d3db4269d0bd8a30` |
| 82 | class_target | `ui_class_target.png` | 553955 | `e743507e7c225a425b28496aa828403a30f2ac32a5bbc31e313ddf696e2be924` |
| 83 | breach | `ui_breach.png` | 685917 | `d1cc37a809668aaa1cc8bd76776e67a04d2ef96631f86e072389b80d57a77b36` |
| 84 | debrief | `ui_debrief.png` | 195735 | `4990a8de76dd2694877dc7f0d9172e37f9425361ce161e710240f33ed9762015` |
| 85 | trauma_debrief | `ui_trauma_debrief.png` | 202757 | `741df737bd0bc2c157e5e2a1398d3198a8220400f5a3ef7d23f7a62bcef45510` |
