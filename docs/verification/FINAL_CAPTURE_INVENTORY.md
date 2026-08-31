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
| 4 | colony | `ui_colony.png` | 879197 | `0d243d778864aeae362d4cfe4bd31bf7626e3f6b623b989ddf776fa708ead902` |
| 5 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739486 | `fc88587d80bfe9e55edcc3c015b272327a86843d487b91ca218e1c15fa314a07` |
| 6 | contact | `ui_contact.png` | 931658 | `07c4faaa0e0431aa92145bad841b13913c914c345c59f7cda5cd6eef63b493db` |
| 7 | contact_gear | `ui_contact_gear.png` | 250666 | `4f696269042dbe8f2a6ce57e7d1b535f8f812ccf8aa127eb8d006da33e83361f` |
| 8 | contact_event | `ui_contact_event.png` | 918615 | `f97ff627d8da15fd12ee1d74d37a7f3397c463980386595a3782b57a374988b2` |
| 9 | adaptation | `ui_adaptation.png` | 893328 | `3fdd2ad1b85860e32f09f3a2eb8135aacb7ad245682300e3f7f73d3180c471cc` |
| 10 | gene_lab | `ui_gene_lab.png` | 182135 | `383b60cb4148f07415865dfc64677371d4a4ccbaffe740eef890fb5c0b735385` |
| 11 | evolution | `ui_evolution.png` | 245242 | `763f45a7ecb722a170258526d757373bff86c840923b87cfeba370fe27c23454` |
| 12 | mara_evolution | `ui_mara_evolution.png` | 180425 | `36aeac0e75ed9e988f7fd2b65dadff0d74efbf66b0ad347f2068f4516c580c1b` |
| 13 | ilya_evolution | `ui_ilya_evolution.png` | 180215 | `c7e600f48fabf0363275cd6a41eaa9af3ee22a42c1a10339eb97d6d8d58dab9d` |
| 14 | sol_evolution | `ui_sol_evolution.png` | 174924 | `d62ee19981a701bb8be3ccea1a7c08f55304fa193e8e14ad22977563030d0eb1` |
| 15 | nadi_evolution | `ui_nadi_evolution.png` | 185539 | `e4985cef48b0ee9eeb3a2a7a6b9419dee5dbe797b60b7912fa10c6fa75aa5eaa` |
| 16 | escalation | `ui_escalation.png` | 896045 | `a4df2bd39231e8877ee68b8c95587719fdd0e06d9ec25a6ae2eb9f94b2ce68bf` |
| 17 | escalation_operation | `ui_escalation_operation.png` | 204970 | `307842c55e523dd49fdcee6d454093e00f67495aa7021811a9d5da8f375aba64` |
| 18 | escalation_response | `ui_escalation_response.png` | 937941 | `eee926eb90f6a73892707d22c30ede1363871b3edeb3c18203a5117fedc0aa32` |
| 19 | mirexis | `ui_mirexis.png` | 938130 | `45485b9382fa4b68f5ce0ad0d5c20a1915ac4077e25abe751afbb6103d22d506` |
| 20 | mirexis_path | `ui_mirexis_path.png` | 898325 | `4eff79ab2b863806b3687a09d9d0f6ffd7f3d07e87b9d40f913b34ee913da50d` |
| 21 | redoubt_end | `ui_redoubt_end.png` | 918191 | `7bc21d54d20f4eadc6378d1c86e42f4918d7a284bd3bd58d795eec6450370adf` |
| 22 | commonwealth_end | `ui_commonwealth_end.png` | 927309 | `b33f06f4832cfc041a78b684a18b977740ced03c1e2e928b01ea354b65155400` |
| 23 | threshold_end | `ui_threshold_end.png` | 928259 | `2e79c4122fafd0a165e1462592ae90bedb7acb140aad879504950ad8c0dae99f` |
| 24 | finale_debrief | `ui_finale_debrief.png` | 197952 | `2d6831008b6c0e93eb1d8bb4fe3251fe167084cd257dec7ac62c3308345028ea` |
| 25 | adaptation_operation | `ui_adaptation_operation.png` | 190744 | `56fd331598f712604ee9ce12d11697f1ce5adcce703c534b2d567c3e20f96db6` |
| 26 | glass_nerve | `ui_glass_nerve.png` | 1186908 | `0f1c7c8a90e70f5e05c4deadf68b6fc69b893775e8d572c9fe21eccfe06bb724` |
| 27 | three_knives | `ui_three_knives.png` | 1190962 | `169710cc2b371ccbea90c8fdc5d584ed9040c0af172bf22584be8e80da2e06d4` |
| 28 | reinforcement_warning | `ui_reinforcement_warning.png` | 1191293 | `f4f617a59893d53f63576822a37270d52c23d4f0fa9ff3d5e33ce7a216b70755` |
| 29 | line_formation | `ui_line_formation.png` | 1186617 | `ba3a81c66fd2b127bbe31900b398eb940a90255e4cf5cc2188f6401bd8f53361` |
| 30 | thin_shelter | `ui_thin_shelter.png` | 1188518 | `dedd3111542b3005f5c520a676ffb993a14a97a555e5f15865be5a2dd230f5e6` |
| 31 | breakwater | `ui_breakwater.png` | 1180808 | `f3edd16251908f0a39de03051b88f202302c5092051c11b2374743e8a6a50028` |
| 32 | false_heart | `ui_false_heart.png` | 1188076 | `2243406fddcb9e11e48cc2e7484d1286e58c1dd853b8f6a7ec843a20ac72c580` |
| 33 | live_wire | `ui_live_wire.png` | 1192451 | `d6797ae53fd8eac2091cb35ed2665bac73ab4d830379751df4def90be931e56f` |
| 34 | last_wall | `ui_last_wall.png` | 1182406 | `37bd0391bdd8afd6b80b5c8a3f483d9b9c75a279264343c82ed8515f18ebe69e` |
| 35 | root_choir | `ui_root_choir.png` | 1192272 | `dc2b6b6c91f0de296f67f8e4df9823f07b61674b26a31f4880d0c611a06e3b24` |
| 36 | door_of_light | `ui_door_of_light.png` | 1184361 | `d51baa7f218d9aeec2f0c176a10018d8a2b091e2d108886fb33d1dfd3802046f` |
| 37 | damage | `ui_damage.png` | 883867 | `e509f9ca2ea24ab617e4367f4cccca744be5a357e03d0a4ec5381f0e87f64f14` |
| 38 | repair | `ui_repair.png` | 890783 | `e95132d2fc1cfac0fa097ba4a83a6d8b1aa8b77bada266c19b6d07812ebb5da5` |
| 39 | power | `ui_power.png` | 882536 | `f39253da0f11a09321c24d376f38e882c335c81f6002ac4fad73bf900bade533` |
| 40 | construction | `ui_construction.png` | 884197 | `0370ef65e19d95a60ae7a9a9675a475af1121ed88f05bedbb589060245506353` |
| 41 | research | `ui_research.png` | 904641 | `56041351573ef9fa488d7221854fd32c195e639b2021556d82807319297abd6c` |
| 42 | roster | `ui_roster.png` | 249555 | `7c12925bc5f75d448982314e73d82e5590ceb8aed51d767b5a8598606797f8c8` |
| 43 | recruited_roster | `ui_recruited_roster.png` | 282407 | `c8dd5b9dfd9b990e87a9e2dd8c274a38be6193d8f64b44fd230b3ddc590bf317` |
| 44 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210796 | `c49dd4defff5a4c80695d7aeb7de3abe7020897bdd18a54570ea625cea5a5671` |
| 45 | advanced_roster | `ui_advanced_roster.png` | 249390 | `399a49ac28fdf487a2631ea49c5a7f7848d7fb66eb69f91ca405996d72843b15` |
| 46 | relationships | `ui_relationships.png` | 251602 | `af125988352dc05ca4d665ba9b96847c66dacbe5f8a82f55d820d7c9994ad1a8` |
| 47 | trauma | `ui_trauma.png` | 253572 | `b38bb29757af91d6087194e112a03a7884d9b4704f66dad226bb46cc8a38b276` |
| 48 | bonded_briefing | `ui_bonded_briefing.png` | 200835 | `2b2ec483ed923538df9cac824b0cbf73e0d7486157348a07b89c1be7f33fb078` |
| 49 | legacy | `ui_legacy.png` | 250257 | `5dbdc81696c77fd05342b8fde298c7c60eb195a5fcd05c41434e234f7bd675e8` |
| 50 | briefing | `ui_briefing.png` | 210008 | `5b0a9b290e80cd98fb9dcf1fc4a1d0b738e99c3c34a767e684d7151ee76694a8` |
| 51 | recruited_briefing | `ui_recruited_briefing.png` | 212087 | `8a59f4a6aaef340e3fac809158be242e6727ca626625d088cd72f0fea789ed79` |
| 52 | threat_briefing | `ui_threat_briefing.png` | 190877 | `2ba2b71579583e39a6c9cbe52ed7420ea0d5caed2f9f17ea59e7245d71a0ebc3` |
| 53 | loadout_briefing | `ui_loadout_briefing.png` | 199779 | `6438030a678535bca40347ecc69de1bdb17c5b5454390891fc74cf2146394539` |
| 54 | pressure | `ui_pressure.png` | 205086 | `aa21d40464dd6069210421fcc6ace54fa4e794311022011d4a96e1d7ac1cb3c7` |
| 55 | gameplay | `ui_gameplay.png` | 1186048 | `10a12f0bef958f08a9ff85a1327a0ef5169137556399ec0dc8ef427a9017bc8b` |
| 56 | first_hour_tactical | `ui_first_hour_tactical.png` | 1191745 | `780fd774528a069f29a9ac3236325563786fee9b3eea2cd3f9afacbac748a07f` |
| 57 | first_hour_attack | `ui_first_hour_attack.png` | 1108269 | `6fcac0f930cce7bc4d321d1eb7edad2866274d641255b54eabe4f31459e953c4` |
| 58 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1200907 | `d0d185d50d5bb1ddf5454b8c1083cd06704caed7361f54e3507da32a9a4586bc` |
| 59 | first_hour_objective | `ui_first_hour_objective.png` | 1343206 | `bef8e5bbdb47de46926cd32190ba44800a5d7752d4cecdccb889a788cc3bb100` |
| 60 | first_hour_replay | `ui_first_hour_replay.png` | 1102005 | `3c0bbd00ccf8b635d9002b07e6f22e79e920fcbed2f173f6efb61852f1530fc5` |
| 61 | first_hour_ability | `ui_first_hour_ability.png` | 1191026 | `b5ac024e2fb9debb96a5af39119d41cfd7e7be35f2161a86e59ad96eb9fe99ab` |
| 62 | second_operation_tactical | `ui_second_operation_tactical.png` | 1113129 | `f15734de401a0ac344e2e4e991b6c0f01b2bb934a5fab25b02584dad5f3485e6` |
| 63 | overwatch | `ui_overwatch.png` | 1189004 | `82bf4a630ebf407c279a4e9f671b53cc62c972a1aae2a621aab06ce521e549f9` |
| 64 | brood_ability | `ui_brood_ability.png` | 1245354 | `747e0b752300bf3de09428d9f393221bb6d22b00860396f52a2871763226b9d9` |
| 65 | directorate_ability | `ui_directorate_ability.png` | 1245706 | `7762e016459378d0dea0eab544f569c311116c1daea5946b4bde14dfa5ef74ae` |
| 66 | ascendant_ability | `ui_ascendant_ability.png` | 1245625 | `05829e6cd2b031f011eabc4ab2741687582318593e232b3ccab3791aae2de4c0` |
| 67 | hazard | `ui_hazard.png` | 1251051 | `f11854cb531369527ee2e126d3c1caa433b1f267e672b9707fbe1d8bed6c96ae` |
| 68 | intent | `ui_intent.png` | 1147814 | `63202688cbefb5e321a939f1f1f523584282cf3f3b5e217b172e049663a44e2d` |
| 69 | action_preview | `ui_action_preview.png` | 1110746 | `8ec7a1c3c2088ec1307b8b9a6c3dc1dd81268a0a8791caa5589841fb4409a744` |
| 70 | movement_route | `ui_movement_route.png` | 1147318 | `1aff69df864656690f7f01a07d1deb855c908a0ffd21bd4353a192f9f6cf7824` |
| 71 | cover_edges | `ui_cover_edges.png` | 1186984 | `777bad0c3de8b6639a680c44da2af47349c536b09644c3b76f5bf565ebae8604` |
| 72 | invalid_command | `ui_invalid_command.png` | 1215503 | `869494c9e801aea95a8ed47291b3555f85de46994d43c2117c7aa13cf08cb7dd` |
| 73 | valid_shot | `ui_valid_shot.png` | 1106235 | `85c3da41707b62397a85c4da047148585c40706031ae2cd9d2241af0760eb018` |
| 74 | threat_range | `ui_threat_range.png` | 1218628 | `2c673c4ff2be4bcdf59561b026be15728234db2a80485e000c6e049caa5fd449` |
| 75 | danger_reach | `ui_danger_reach.png` | 1214880 | `97df78b652e07e00c9179ccffec8addb36f1c3e6fc972fa9b1566448b16e8806` |
| 76 | help | `ui_help.png` | 357373 | `4b74054b7a7598fd7437da3f33fdac8421bb9433f894d23716efc8d2cfc4caa9` |
| 77 | first_hour_guide | `ui_first_hour_guide.png` | 215385 | `c189bb5f7bbb69a96335496e9a32aa3c0aa2d8996b770a7c6be20c1e0236552b` |
| 78 | first_hour_return | `ui_first_hour_return.png` | 878773 | `fd0dc04443a8997f75d5df52ccb52be4dd81f50ef99317054f2085f570c9fb38` |
| 79 | first_hour_promise | `ui_first_hour_promise.png` | 881405 | `44fd622d618faf77ff7d1e85fda005e596bb9a65978d0b4a48a3d0df535261bf` |
| 80 | first_hour_operations | `ui_first_hour_operations.png` | 919231 | `7e3facaad6d62f0abbb39378add0d522ec7ee81a13ce2b2d64a0a6b9c3b28d52` |
| 81 | battle_log | `ui_battle_log.png` | 660805 | `5a33953aec0046c48b392da0870b0c09bb33c2cc4788f2a438e98a127ff2f71a` |
| 82 | combat_feedback | `ui_combat_feedback.png` | 1186227 | `3892f2cd673f8f6f5cac55bd882e8aa0de72f9084562f37159e044a5166072a8` |
| 83 | phase_replay | `ui_phase_replay.png` | 1095909 | `160cfad2af9131adf523bb08a3584462e07661dadce14f0063f04477a3b58fbc` |
| 84 | end_phase_guard | `ui_end_phase_guard.png` | 1185930 | `74869a82672c8a0dd53e4f7d36f51726bcda0e4b68c69d161a044d01678d6c53` |
| 85 | readiness_markers | `ui_readiness_markers.png` | 1189986 | `f392d353d2ed1b12f8fc3fff4a59e0b0a171f356d9bfe760d816c1556899c13c` |
| 86 | vitality_markers | `ui_vitality_markers.png` | 1190410 | `a4b628007663a2661a182099e6758e67aaebceb186c57f72c04e2bbbb3775951` |
| 87 | extraction | `ui_extraction.png` | 1439331 | `b7fdea2bd1e12b99811ff9622482333514685da37f2376524ca18ba1bdc9b277` |
| 88 | variant | `ui_variant.png` | 1238076 | `93a4613cdd41ddef3b072644f7660df695c71f282f41fdbe806004a548d53cea` |
| 89 | sporefield | `ui_sporefield.png` | 1190870 | `818042f81b03b5e87548e14aea220bbb0c59b7ab6068bd829d21d160c29e63ff` |
| 90 | vault | `ui_vault.png` | 1186332 | `01e636ea9e0b26d831507ec9d8f68635e987cc162f5c59ff1b23405a9e73f180` |
| 91 | black_channel | `ui_black_channel.png` | 1194525 | `c8078bf702a04e1d6ee3bd96d25ff63094793497e874dfa46807789039e8ec6c` |
| 92 | living_chorus | `ui_living_chorus.png` | 1199097 | `fd16e28837732626265490cd4dfae0809efa26c3e0171b29e107da5da3cf5ec1` |
| 93 | open_circuit | `ui_open_circuit.png` | 1197934 | `b95a3e6df70a470ddc66266d88f06c233eac29bbc28483cef381465280da805a` |
| 94 | trace_active | `ui_trace_active.png` | 1161888 | `d83604dc8e4bcf3a297451d8528d6f8b8ce12d02595da616e8373b2edb2009ad` |
| 95 | equipment | `ui_equipment.png` | 1019074 | `3d7156d3a62167e4c14fb6b782b21197017228ad716ad50d086832d40f4d1254` |
| 96 | weapon_profile | `ui_weapon_profile.png` | 1185852 | `9f3e671028426e34ba3206330b56f6bd6a1f9b74bb63a6b7e1aba98715772d9d` |
| 97 | class_target | `ui_class_target.png` | 1038294 | `24d198153ffb577faae6258bee1ffa4437a113b8bc612dc5a2d41749b46b1f46` |
| 98 | breach | `ui_breach.png` | 1368351 | `a04ad6832ffab1139e47f5156a09256d3df241fcac0f750ce9590ac5c97fb22a` |
| 99 | debrief | `ui_debrief.png` | 218275 | `c95526afa213d39b9505d45cad84cb774a826908cdb90e0cc9a9e13da10007f4` |
| 100 | trauma_debrief | `ui_trauma_debrief.png` | 201683 | `604954c2d0e8d328bd41c55132acbc81a6d0ca39a8f3755c40d686fa992254bd` |
