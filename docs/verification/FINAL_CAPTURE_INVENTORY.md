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
- `ui_contact.png`, `ui_research.png`, `ui_adaptation.png`, `ui_escalation_response.png`, and `ui_mirexis_path.png`: progression references open the Operations drawer at the decision surface they represent, keeping dossier, every available doctrine, and path context visible.
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
| 1 | title | `ui_title.png` | 441810 | `625b56021556f91092bc11e165ef6ccce21678f1ed62d458edaf0bfe1b84d69b` |
| 2 | title_controller | `ui_title_controller.png` | 444314 | `0ef437423c16cd2e8bbe2111539eda265081ccbb99767924690de8aa845dcf10` |
| 3 | title_hover | `ui_title_hover.png` | 441788 | `f789766f7c06c340799fa6ea87e6456a321093019eb9113368b7591131e96d87` |
| 4 | colony | `ui_colony.png` | 879226 | `f94f870e3deb43e536bd3f26e8fc585f01939d7f65d89f218902cc96c6a3335e` |
| 5 | settings | `ui_settings.png` | 230015 | `39835fd8a7028594e4aaf62fbc27989381fa73530192fd6a9c19ea11cb7e1b60` |
| 6 | field_notes | `ui_field_notes.png` | 124905 | `13cc8f3d57d3fc1976856e606dc41029b9918f41b2c6ec19d15c0726bb2d4d5c` |
| 7 | memorial | `ui_memorial.png` | 168386 | `1d233bebffc9f9b65e6bddf3189a85b2253e07a34644e169439fad89bc3aab47` |
| 8 | memorial_page_two | `ui_memorial_page_two.png` | 121902 | `d8e7ae8eb7e6d5b69eaec6fb0a17a300ff85f915d7b0cd4569306e7da66d4e6e` |
| 9 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739634 | `f7c89eaa7dce3e02606aab83c719d5b1a7ed8fb86bcfc4eeb04f10114f75b9bb` |
| 10 | contact | `ui_contact.png` | 936291 | `7329b4d24c7ac7684abfb41df62dfc95e921503609dc9118824476cae477203f` |
| 11 | contact_gear | `ui_contact_gear.png` | 250804 | `8220d1546956ba2031440dff90c7a1a144cd1bd88cc9a5a81b0be3f6bec62cbb` |
| 12 | contact_event | `ui_contact_event.png` | 923793 | `0125ecf4f9b09acbc6cfc6e62b4e2128cbfb838702507908c6467912d3953c9c` |
| 13 | adaptation | `ui_adaptation.png` | 898399 | `b33841b8aaaa6d8b54677b7384e7704544f7201b8be4ae42c5034bdeca673973` |
| 14 | gene_lab | `ui_gene_lab.png` | 182184 | `42d41fb26581c59f368962eaebe04e019446ac55edac593a17d4e97876067e90` |
| 15 | evolution | `ui_evolution.png` | 245376 | `4e5d75b61cfe6673c6729cb72e8cfb61cf2a031d4876b284118f29590deb98b1` |
| 16 | mara_evolution | `ui_mara_evolution.png` | 180469 | `d54a00b49166f68bcfe31a8dbbb02b420fe145dd11334b465080d95e6eb74acf` |
| 17 | ilya_evolution | `ui_ilya_evolution.png` | 180258 | `bd24e9e3c851bf7fd62a38745f683b5eee48d2865bbcf9ee1631e9d2a9134ad8` |
| 18 | sol_evolution | `ui_sol_evolution.png` | 174968 | `9e7e65e42c9568bef7546451199ed4cd78114bb6e75bf2932d781ff7250ceaff` |
| 19 | nadi_evolution | `ui_nadi_evolution.png` | 185582 | `3c2505ba718540d9259bb0c3e09d8c6895313536f3a3bb668e015f555c0db665` |
| 20 | escalation | `ui_escalation.png` | 901120 | `e15206ea0fce48750a78b8de589ec0185c4512bc8edfa69144741b12a469aece` |
| 21 | escalation_operation | `ui_escalation_operation.png` | 205323 | `08dcfb0530bdf17581b9838557bd249005ec1327715dab56e2d468b7246da3b4` |
| 22 | escalation_response | `ui_escalation_response.png` | 943082 | `244c6a5ed9d270b14030dca16adad586fe40ea7afc608d543acb3fa6a9d888d4` |
| 23 | mirexis | `ui_mirexis.png` | 943113 | `46d7b9175fbae9fc1bc4dd967909b3ef02108de3391f3785a8873ae84635ac7c` |
| 24 | mirexis_path | `ui_mirexis_path.png` | 903271 | `061a4442fb08e3fc9e8f68034333f0b4199be4739603d2dcf28cdf2cf71c491e` |
| 25 | redoubt_end | `ui_redoubt_end.png` | 922941 | `bef8026924078ad38196481aa8c4ac1645eb33829be81b24d8ed22d2096495a0` |
| 26 | commonwealth_end | `ui_commonwealth_end.png` | 932028 | `7e2e65889fc17fbe48f62138f96d9452f771032ed20c74f299566284dc2323d6` |
| 27 | threshold_end | `ui_threshold_end.png` | 933005 | `4e4b0e8a472095b99200488ff4562811662dcbda754cd64a2cd0fc7e4326b557` |
| 28 | finale_debrief | `ui_finale_debrief.png` | 198033 | `de347de84bd00bc495367b08c96293a155c08d3350833083765083a867572f65` |
| 29 | adaptation_operation | `ui_adaptation_operation.png` | 190749 | `8f1f3b2a7d76e93f82f79843109fe10795dee10e6d21ed1de2962c2e8f2c5b59` |
| 30 | glass_nerve | `ui_glass_nerve.png` | 1187797 | `563f638cbc1f830eff1eec85878878493e8f07d3fd1d7092467f92642421c88c` |
| 31 | three_knives | `ui_three_knives.png` | 1191845 | `fd10d4a9b7b87ea46ed6ffdec09c729f0373170c5daded152441a8775450e349` |
| 32 | reinforcement_warning | `ui_reinforcement_warning.png` | 1192173 | `4aae2b21cc9896d02de499d5ba87500f5eafb861fee9a308ee64bab7fd26fe89` |
| 33 | line_formation | `ui_line_formation.png` | 1187512 | `f8d242af6da3f9a63c471e5b4a096910f6c1f46f182647ea5a1a55a24060c181` |
| 34 | thin_shelter | `ui_thin_shelter.png` | 1189416 | `f3210578ad2625e282da62a320e4bda871e0354bbd5acc2cc89e34ed3e19944c` |
| 35 | breakwater | `ui_breakwater.png` | 1181687 | `9b6f5cbc02202fc0bdbf55a40044b8f6088e0bdd7bf83701020f154de9b35947` |
| 36 | false_heart | `ui_false_heart.png` | 1188957 | `7b11523df3c6b55631ffac72045d00f5b52d16e65c45210de6621769d6045cfd` |
| 37 | live_wire | `ui_live_wire.png` | 1193332 | `60557563afec31cb3f79c1a1118f617947172ce3c292f5a82c6adb6fd28dd58c` |
| 38 | last_wall | `ui_last_wall.png` | 1183299 | `1aa7a21644f4342b58f487d1331d9fb0f09f0c51fa6db5e97a05f5cb24da3be4` |
| 39 | root_choir | `ui_root_choir.png` | 1193121 | `037aab21601775890ba8a951e5e070a4621f42061d5a13c439e640335ee08d16` |
| 40 | door_of_light | `ui_door_of_light.png` | 1185245 | `35b082b67cf1fcd602aec438ef022414c14135e956c34bb173f1e99706d1e34a` |
| 41 | damage | `ui_damage.png` | 883827 | `ccb3e072db96cc7b6b2116463575b30c797e1f0ec97c0a0bca6dc9381505f2dd` |
| 42 | repair | `ui_repair.png` | 890713 | `1d6d71a43d72a8e93ed605e82574ac7cb707266f69c00bf6331df938a9dc871d` |
| 43 | power | `ui_power.png` | 882495 | `7e17ca9d4d6c7c327fd03446e80b8310fd967e20b1307b7cfbd194ce05296965` |
| 44 | construction | `ui_construction.png` | 884154 | `847a80a93d785a6e4824173fd875a1815356c8926f82e8269d331af43c5d168d` |
| 45 | research | `ui_research.png` | 916106 | `98cf7d2c3f5223866077959b68eb977ccad44f3f38b912bc2ecd763e6f9d008b` |
| 46 | roster | `ui_roster.png` | 249491 | `02a658da2bc1ccd28cbbaa0084fc65c312633c74d3469ba278951b2356a9dc4c` |
| 47 | recruited_roster | `ui_recruited_roster.png` | 282123 | `74b75f7e14d5d9959c9fa018b4973c2a130296ee415fd4f065c7ba8345ee7540` |
| 48 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210355 | `654c0df29945971abf76aed4667fbbc76ac39f2fc228d45b7bd645282ae82b26` |
| 49 | advanced_roster | `ui_advanced_roster.png` | 249340 | `cd93837df26e41b34aa5ba75926f0609996badb15895666cbdf03df0db7755a0` |
| 50 | relationships | `ui_relationships.png` | 251541 | `f918592e5dc16ab2f13665c3d43ee67a13b80933687439961fec2615d9765ecf` |
| 51 | trauma | `ui_trauma.png` | 253508 | `99606a4501a4336d55474dbad69d360f057ebc521f9826de090b290e43f06910` |
| 52 | bonded_briefing | `ui_bonded_briefing.png` | 200759 | `51f888dbdf74d6650b0cc6b52e27e7d415211779dfaaa315f98e03d7017a7847` |
| 53 | legacy | `ui_legacy.png` | 250202 | `6bf71b5c45b141091cd37c3b3458fa2c913b887f3c0c70a438d98745863cf603` |
| 54 | briefing | `ui_briefing.png` | 209840 | `f9935ad46303b09130da5af32cb02bc8529cacb38cf7f72b4f3fe849d196f804` |
| 55 | recruited_briefing | `ui_recruited_briefing.png` | 212019 | `767f32b80dfbc6fd59e0ef216e15738af0d99a7bbbce6953316a55c26efeb52e` |
| 56 | threat_briefing | `ui_threat_briefing.png` | 190720 | `b6c4504edbabe2d9d1cbfa8f6cdf5e267ce360bfe7cec7b8f6a54ffa412f6fdd` |
| 57 | loadout_briefing | `ui_loadout_briefing.png` | 199667 | `88af477934ba34fade160978faa80f6af7245b216415a30ae62fad2a6ad697fe` |
| 58 | pressure | `ui_pressure.png` | 204992 | `d031961db941b0954669b96c5cb9d6e9c6ce76179a1333fcae0682e011a93b8b` |
| 59 | gameplay | `ui_gameplay.png` | 1186905 | `8703ccd6267255c5dd66aaf9eae1480c765c0ac4213ace1d9f2cef84260d1ce7` |
| 60 | first_hour_tactical | `ui_first_hour_tactical.png` | 1192657 | `a92b9d1f8884be6b1f25d8319010a46c3c1a7bf51265607494911f17e2eb4310` |
| 61 | first_hour_attack | `ui_first_hour_attack.png` | 1111735 | `d07f213deabca2ab14b3c4d6e4fce82e6571b23cd69b50264d4c37669631c2c8` |
| 62 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1201693 | `61957156a5d110495bdc9d40c9d6453994a5e388e601e66f4bc69ddeb5c9d62f` |
| 63 | first_hour_objective | `ui_first_hour_objective.png` | 1344097 | `c4d92aed7ff39592bc37ef106e99b7de8c8c09a9af221112e19156841bf4d9a5` |
| 64 | first_hour_replay | `ui_first_hour_replay.png` | 1102922 | `448eaf98ad00880d075b772024d984a1c5c946b7a8218811cf6c652b1a3d8c2b` |
| 65 | first_hour_ability | `ui_first_hour_ability.png` | 1192054 | `095731799d8662c0ec4038568c7e1607957ea2aeafdf78f3a71ca615315e7cf9` |
| 66 | second_operation_tactical | `ui_second_operation_tactical.png` | 1116626 | `32ad973415399e5a75c03e02d842ee815da7d870bf994feaefe2bc8585007330` |
| 67 | overwatch | `ui_overwatch.png` | 1189967 | `b63ef7c65f0b6f33f3259af020ea12230bb245fd7df7670a964e400c6f9881fd` |
| 68 | brood_ability | `ui_brood_ability.png` | 1245456 | `9979f5b514ec675b170b8db4f297fdf4f5fc26570859699bbde14f39516fbbec` |
| 69 | directorate_ability | `ui_directorate_ability.png` | 1245577 | `750d2b76c3f28a1f87c8aad00501d002b1dbd78b268346e55d13cc726c7fa279` |
| 70 | ascendant_ability | `ui_ascendant_ability.png` | 1245862 | `18308e46fe14a7b7dd1d0ab32f1be46711bad7d951d346320786a3e7bbbaddac` |
| 71 | hazard | `ui_hazard.png` | 1251994 | `70b67016c121653dfb138bcbc94a7e611b1f6832f66667fe499334a856d9a767` |
| 72 | intent | `ui_intent.png` | 1152982 | `ef5a65d3c7bb7a277014fbe35421a21731f7445f687a1278a417ba850fcd70e1` |
| 73 | action_preview | `ui_action_preview.png` | 1114299 | `f1f43555ee366636ce8b7f9dacece63b361856dab73731c442e54080d04ae654` |
| 74 | movement_route | `ui_movement_route.png` | 1148283 | `d8aeffdf433431c5c8514795e4ba55a693faf2aa6f9b0b28c29dcafeb8e0540f` |
| 75 | cover_edges | `ui_cover_edges.png` | 1187950 | `daa9235182868acccf7c882b9ba2bbc06749af85fbe0407504f9f08dc59df4ad` |
| 76 | invalid_command | `ui_invalid_command.png` | 1218990 | `7116defc2cf0826c550bdc8238c420f3a5922fc560706b670ddb3b4841246308` |
| 77 | valid_shot | `ui_valid_shot.png` | 1109712 | `a7d39fb01d7da7bbb6f7a66fb627938aa01177316733d9a75e1a1d4de7ea324d` |
| 78 | threat_range | `ui_threat_range.png` | 1221967 | `11dd6bb3aa018fb81094ba839d0967955c31c0a76dc6f62f284bde828a9bf2e5` |
| 79 | danger_reach | `ui_danger_reach.png` | 1218218 | `c7dd069fa1641288aa43b9e5070df11077552c196b506182bfabab7fa487ac72` |
| 80 | help | `ui_help.png` | 360034 | `72f30bd42e2ee0b077e834de177ed2a575b2f7078cd7c5e533462a8307810296` |
| 81 | first_hour_guide | `ui_first_hour_guide.png` | 215035 | `e4b06fcf4fd419f67c2763e5945fdeb2bac23e6a987e75f62fb5863f673231b8` |
| 82 | first_hour_return | `ui_first_hour_return.png` | 878760 | `7a5f67586c0a9bbc89360c65b80f8cc21e8d087a1e9520bf1f9511d7cb3eed90` |
| 83 | first_hour_promise | `ui_first_hour_promise.png` | 881409 | `ed696174bda1ee8845b07931a057d9df970a9ccb11d6f472c31a857c48495a43` |
| 84 | first_hour_operations | `ui_first_hour_operations.png` | 923898 | `008f0d6f8e9220690f929a0e6b383f29efe84c421254d11eabf041226556d115` |
| 85 | battle_log | `ui_battle_log.png` | 662213 | `6feccc6d114cc93d0d14d3da2a89117986a14371f3df79e85827b6ab2d1f86db` |
| 86 | combat_feedback | `ui_combat_feedback.png` | 1187382 | `014866fae0308323828d1e60234a035188e507d96e3f75a38349d5bb7e76e7a1` |
| 87 | phase_replay | `ui_phase_replay.png` | 1097062 | `40314303265391215f9f98e02cf34b580a9691f56c3a9e117d628d84058ea964` |
| 88 | end_phase_guard | `ui_end_phase_guard.png` | 1187083 | `834a981a1f69b3225bf41e6c99b45f6d92df48963e17e62a2064b89db5ee8d88` |
| 89 | readiness_markers | `ui_readiness_markers.png` | 1191174 | `cd5c86144ac05201b6b78ab93ab5e5044c702895a24768f77e5f2be4cb610d22` |
| 90 | vitality_markers | `ui_vitality_markers.png` | 1191598 | `a46af891b5b9d5e59fb103567fe04b6cee9e7cde347ca4c370dbaa1cec09ed97` |
| 91 | extraction | `ui_extraction.png` | 1440460 | `794df4da0baed20b921ae6cb9186acc361a099df1b9234da305f459249452ebb` |
| 92 | variant | `ui_variant.png` | 1239239 | `ca5e9265340d62fbeb109cae7221e8ee3c9dc81f673ca53f683ad1cb22fb4743` |
| 93 | sporefield | `ui_sporefield.png` | 1192025 | `f1f4fe7b9d705f19f569f21eb1ad21f154e8b3aafef5b8dadba4770e73983cae` |
| 94 | vault | `ui_vault.png` | 1187494 | `c87aa0152208c1a5c4e5a128e06577d4a64926a4b8fd60368246d4978294de74` |
| 95 | black_channel | `ui_black_channel.png` | 1195716 | `05ed7083daef751aca5984cd40cd9e0794ae8dfdd5333ade030a1440429c6ecd` |
| 96 | living_chorus | `ui_living_chorus.png` | 1200310 | `61c4cc4b50f336e215cfdd471e219eb36b5a2cb3bf2b98a9b15b92757f8dcc20` |
| 97 | open_circuit | `ui_open_circuit.png` | 1199123 | `2a248d22f0f015747647fba9ea8745127fc500ce073ff437cff8fbe61a4e9e54` |
| 98 | trace_active | `ui_trace_active.png` | 1162975 | `b805b8618b902e67ae0b7a3da606a15880eb18044ae3afb68f81eb90fe70d0b3` |
| 99 | equipment | `ui_equipment.png` | 1022292 | `86d1483c4a3280546c0126c29a1e85eb548ff0c54e24ac0d1a0c6c25bd49c057` |
| 100 | weapon_profile | `ui_weapon_profile.png` | 1187011 | `8a126be73153ce7a4f39b5a98849c858e952e46bd5df3ef754e9e3db588cfe30` |
| 101 | class_target | `ui_class_target.png` | 1039444 | `83724a53dcb5e54bc7b3ee341825353c913fbeca8c0e9277b9336ccf2eb0a6a4` |
| 102 | breach | `ui_breach.png` | 1363210 | `07060de9c23d2507053de56d3624f90025f88b6536d7aa2d0c66130fef77e12e` |
| 103 | debrief | `ui_debrief.png` | 218725 | `64897813a10d07aa81596e13e7d61ffba435f88617fd15e9db18efbb9b251681` |
| 104 | trauma_debrief | `ui_trauma_debrief.png` | 202174 | `154bbe7407f20db084407ead0c4690503175561ec4ca1045cbd5b106925645c2` |
