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
- `ui_memorial.png`: the Operations register keeps persistent scars, recovery records, lost objectives, and character legacies visible with an explicit return path.
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
| 5 | settings | `ui_settings.png` | 230014 | `d97d87daedfa342142fc669ee19164de8b981f214a0781ad8be4dd52e887e0df` |
| 6 | field_notes | `ui_field_notes.png` | 124524 | `d05eee2b5bd80313181daadf9d1115ed5719c6d4e629807731940e443313433a` |
| 7 | memorial | `ui_memorial.png` | 148154 | `614a50690cddf30c73e06bb2c121eac64b24d0abed4ec38654ecff24ac9e9a7f` |
| 8 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739486 | `fc88587d80bfe9e55edcc3c015b272327a86843d487b91ca218e1c15fa314a07` |
| 9 | contact | `ui_contact.png` | 935969 | `e9f224863b3ded70d4e10dc8d687f848fbc86a297a8e6d752e1b61aa63180a0e` |
| 10 | contact_gear | `ui_contact_gear.png` | 250666 | `78f11ada2890beb4d7c638268dc90aa8ea9f23259c2b6a3690e8e8732f35a67e` |
| 11 | contact_event | `ui_contact_event.png` | 923412 | `d1bb5e65fcfb905b6cb7a9327c276bdd0cf1b3c37c6414cc0b4007f17fda8091` |
| 12 | adaptation | `ui_adaptation.png` | 898112 | `7831d59fb86963bb170ace836caacfedb57f3e5468b341d9e1797dfd8e3f6229` |
| 13 | gene_lab | `ui_gene_lab.png` | 182135 | `383b60cb4148f07415865dfc64677371d4a4ccbaffe740eef890fb5c0b735385` |
| 14 | evolution | `ui_evolution.png` | 245242 | `763f45a7ecb722a170258526d757373bff86c840923b87cfeba370fe27c23454` |
| 15 | mara_evolution | `ui_mara_evolution.png` | 180425 | `36aeac0e75ed9e988f7fd2b65dadff0d74efbf66b0ad347f2068f4516c580c1b` |
| 16 | ilya_evolution | `ui_ilya_evolution.png` | 180215 | `c7e600f48fabf0363275cd6a41eaa9af3ee22a42c1a10339eb97d6d8d58dab9d` |
| 17 | sol_evolution | `ui_sol_evolution.png` | 174924 | `d62ee19981a701bb8be3ccea1a7c08f55304fa193e8e14ad22977563030d0eb1` |
| 18 | nadi_evolution | `ui_nadi_evolution.png` | 185539 | `e4985cef48b0ee9eeb3a2a7a6b9419dee5dbe797b60b7912fa10c6fa75aa5eaa` |
| 19 | escalation | `ui_escalation.png` | 900822 | `e81dab57d3ee122bd266c85b140376038ea14100945416e5c11bf265ef597bf5` |
| 20 | escalation_operation | `ui_escalation_operation.png` | 205072 | `751fe78297642fd5d6220367527d1f63a6c997cf7041379068f79b77e333029f` |
| 21 | escalation_response | `ui_escalation_response.png` | 942728 | `2b44f294dcc4bde318078ee740ab8c710bc3efa9c2bf14e7ec6223af031aabd8` |
| 22 | mirexis | `ui_mirexis.png` | 942907 | `6dd7a9ec97ac6b2df39caf0ef48b7c77411d1bab61771342363c4d286112be6a` |
| 23 | mirexis_path | `ui_mirexis_path.png` | 903102 | `5f5393fb7a5dc00bb5b89f144b73b1235ac277343b82714590ff8d5169434499` |
| 24 | redoubt_end | `ui_redoubt_end.png` | 922607 | `6cfc4d270043a5fc2e7936c525615838c2d64a10199bec0c54b17f1a2136b4d4` |
| 25 | commonwealth_end | `ui_commonwealth_end.png` | 931724 | `77e4317abb060654abf07f531a9a08cd5caef6a5140d81dc6cf8dc8f2bb6f328` |
| 26 | threshold_end | `ui_threshold_end.png` | 932671 | `1a12ded5315cb16aff6e487ebdc724deb0ea853c19e62b1c15b0b65ab3b1f970` |
| 27 | finale_debrief | `ui_finale_debrief.png` | 197645 | `b402a19906425a1f4b7c7afa28a02e8fd5fe46f4cd03c8a6cd085332e4306f6c` |
| 28 | adaptation_operation | `ui_adaptation_operation.png` | 190638 | `70617791844f874d5f6239496fccbf77e64a89f4ef141abdad32e946dcd01a40` |
| 29 | glass_nerve | `ui_glass_nerve.png` | 1187625 | `be47f59f9bc832f87b9ea1bc75c066a766035ea7abf0bea0517fee3c3f13eaa5` |
| 30 | three_knives | `ui_three_knives.png` | 1191674 | `f9488dad67568ab0bd74b16492482c126618a989c398181232c3fccbaa79474e` |
| 31 | reinforcement_warning | `ui_reinforcement_warning.png` | 1192002 | `9cf52c866f157dad4456a901b8c24ba6767a1ac61ab40ee784c6debcd72923d9` |
| 32 | line_formation | `ui_line_formation.png` | 1187341 | `e3ce2fe92650df49dbb325e7cad0733912e4794f764d9fa0898bff873bb77023` |
| 33 | thin_shelter | `ui_thin_shelter.png` | 1189236 | `60ac13a69009e0e548132894429a6d665a30b01f5468f5388f7d170ea2d35ad0` |
| 34 | breakwater | `ui_breakwater.png` | 1181508 | `2ec76a0cff2a6092cd352d8fb105d62987ae6a1e0b496dd045658c40690bc867` |
| 35 | false_heart | `ui_false_heart.png` | 1188785 | `60152652b394b76f469294999964148a286668eaa76d5b58fb7c9a40536b0ee4` |
| 36 | live_wire | `ui_live_wire.png` | 1193148 | `87c3a1c36b75b9c93f3c485005038749e23978626ba2732483c4e5af70dfaf0e` |
| 37 | last_wall | `ui_last_wall.png` | 1183105 | `c50db7bf474f70eddd1041b480465b9fa5cf898cf9eb386cccf32ec36546d699` |
| 38 | root_choir | `ui_root_choir.png` | 1192938 | `f84209f50425875535a038fc14ec2e16ac577e14c84de48caed15851612d81b6` |
| 39 | door_of_light | `ui_door_of_light.png` | 1185050 | `1d064fceab844de6e3c955e6aea1e26fe67520b9abf84768dff7c3257c5d1947` |
| 40 | damage | `ui_damage.png` | 883797 | `f73dcb5f4c20aded6d1ec6aafd955a27a4b5c49edf4120d942d893bbfcb90c98` |
| 41 | repair | `ui_repair.png` | 890681 | `f7c1c2af8f4bb2f7d8eb4282cbd5102d7de9606dd8ed86e608752fe367862266` |
| 42 | power | `ui_power.png` | 882465 | `f7a88dc5e75392f8203c3343c87c457678a3f64410ea33088ed02e3b6d5cf3c7` |
| 43 | construction | `ui_construction.png` | 884122 | `8f24b5e04d72349b879cccc7769105aee55e2495e24e0119533f2354712b4a07` |
| 44 | research | `ui_research.png` | 908933 | `d62e23b9b6200cd338661ebdc357e2513b4147926c2fd9aa1a146a952e3b090e` |
| 45 | roster | `ui_roster.png` | 249353 | `0250ae92bea8c9c642231c91b33d5fdcf427004b574343d038299103d7ac8787` |
| 46 | recruited_roster | `ui_recruited_roster.png` | 281931 | `65eefd17eeb33917b44ac0654cb661945075967e43311919848cb0f6bdd6ed80` |
| 47 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210440 | `95ce33643fa1a7022d6d82f567dfb75b89bc30e9d8773aa7227ccf9e9b91bc09` |
| 48 | advanced_roster | `ui_advanced_roster.png` | 249209 | `f19f302a71c4bda92f73556bc1be8546911579d4881b91997f353e8f2addeaa3` |
| 49 | relationships | `ui_relationships.png` | 251402 | `b0893d4ec7199e24ab38bad721401c3c4a01b53c18595eddc2f9a6ce69e44e71` |
| 50 | trauma | `ui_trauma.png` | 253370 | `aed24d7e55f4f4bfc956aec502e2d1989da1847197228692c668d1a2179d2f71` |
| 51 | bonded_briefing | `ui_bonded_briefing.png` | 200435 | `faf84665fa4151337fc06b3ab1108d0a7172dd6b8baa12af917c2f9110f9efce` |
| 52 | legacy | `ui_legacy.png` | 250061 | `c35e8546cdf906bfc93d0835188d3bd88afcea217a133a21e7635afe076403ad` |
| 53 | briefing | `ui_briefing.png` | 209600 | `6dfd2799afd68cb5aece8dbae7432e717b2633ebf088bae2f58e9bf76f2844a4` |
| 54 | recruited_briefing | `ui_recruited_briefing.png` | 211762 | `8ff0a4ac38a4eb77b21a380395e2b1da2fa94d260ae2ef958b0601fadf1f1e5c` |
| 55 | threat_briefing | `ui_threat_briefing.png` | 190515 | `15024c6f106b3d922887e05741684c1d73de3090180a12ded111aa4dc2c57c26` |
| 56 | loadout_briefing | `ui_loadout_briefing.png` | 199427 | `4106b7288189b4ee30b1f215ca2d4b376600cc3de7a7a4cbeebb6a1109d0a28f` |
| 57 | pressure | `ui_pressure.png` | 204703 | `99488348682e274285aa09f4323a7874d57477a32436dedf96c91272563f0b23` |
| 58 | gameplay | `ui_gameplay.png` | 1186733 | `6fbc07ddbd6ced9e97f9ed030387b728be8e7c6a7b79604a184cf433050423f3` |
| 59 | first_hour_tactical | `ui_first_hour_tactical.png` | 1192529 | `80849307b13540a988ef30294068959f8ac7fa2ec9a2ebe2be98543df026d6d7` |
| 60 | first_hour_attack | `ui_first_hour_attack.png` | 1111690 | `15a505e2344381cc08072f3fd472e0e8d15e59b4b6e9d6f59c5faf3513f8d68d` |
| 61 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1201564 | `8793f2686068d222cabb57ee48ea12be739b94b46de93bd6bec81c471889747d` |
| 62 | first_hour_objective | `ui_first_hour_objective.png` | 1343998 | `a38b27f164f8f97be1207b70a5601207c8669cd73a27e49de44a78fc25591fee` |
| 63 | first_hour_replay | `ui_first_hour_replay.png` | 1102796 | `8d926f58c00a89443d24301c16b4b5171c27e92db29004acbb975fd3c876980a` |
| 64 | first_hour_ability | `ui_first_hour_ability.png` | 1191927 | `28879e151f7102915cefd57b2d9702ef608a681ff8906bb5f38f097ba51ff994` |
| 65 | second_operation_tactical | `ui_second_operation_tactical.png` | 1116582 | `8a4527346ecb8da027ae87df72cb9a00c5cad048bcc92fe1b98e4ec17ece15c2` |
| 66 | overwatch | `ui_overwatch.png` | 1189800 | `800ef3f41e6c74130a645c75e63f8ba5d75f1a1820bb0a0c43606b476877581b` |
| 67 | brood_ability | `ui_brood_ability.png` | 1245301 | `56c23b0e8887f85672e2cc1a475e04cff676dad7552314ba9f03fa271fa8eaaf` |
| 68 | directorate_ability | `ui_directorate_ability.png` | 1245391 | `d2c58358e82b68ba90b4ae681e97f4a697921057f5fc3227291a70adde3c40d2` |
| 69 | ascendant_ability | `ui_ascendant_ability.png` | 1245684 | `72d6b648b845ed731867d18db804d2c00a8dd3ae961a762bc446263e95282318` |
| 70 | hazard | `ui_hazard.png` | 1251832 | `659cb8f6059e323649a91fc5eeadd63a3dc074502984805f7ced0516962d52e6` |
| 71 | intent | `ui_intent.png` | 1152865 | `743b4815ad82d6eb52be134eca94bed18b0a3e713b4fec2c0eaf90daf269be66` |
| 72 | action_preview | `ui_action_preview.png` | 1114174 | `65c0db564b280b9a2487b972f80026328979c177b89d2b340463b26f7acf1e1f` |
| 73 | movement_route | `ui_movement_route.png` | 1148111 | `fc50f088494f1fc91d0768c80f77abfb6c16b6c094ea51a84be8914c2ddd4e41` |
| 74 | cover_edges | `ui_cover_edges.png` | 1187778 | `b9844d420d794d061f16d27b81fa5fb880b14953cfb0bc9c46d2fdd251786a41` |
| 75 | invalid_command | `ui_invalid_command.png` | 1218865 | `19588da6a12c8fc56f2ef5a7a5635d37ce926e6564b441c000d666b47115a832` |
| 76 | valid_shot | `ui_valid_shot.png` | 1109590 | `9235e6c7c195dad0e2e72ffa9e81c31086a039999376b70202aad4ce154f6c03` |
| 77 | threat_range | `ui_threat_range.png` | 1221842 | `4e2807deecea9ad8a6392bcb6637c394127f9f2c1b423952e16f4d00bcadf1db` |
| 78 | danger_reach | `ui_danger_reach.png` | 1218093 | `6b1bd4b258b0b821fc843fa83401b950f8868466d7b2b4dd16a86d0b4c58d8c3` |
| 79 | help | `ui_help.png` | 359966 | `6da7acf520b16132c8a9a56800079bcfef92092c41e4da0332a39340c81f0c79` |
| 80 | first_hour_guide | `ui_first_hour_guide.png` | 215026 | `69ff1d3367854044cea0e75a0a594e1b54f724e05a288d3c30c3671e8a792e03` |
| 81 | first_hour_return | `ui_first_hour_return.png` | 878729 | `7298f2b6c3530423e31288fb6846397ea23070efa7a722d5938e246236727fd1` |
| 82 | first_hour_promise | `ui_first_hour_promise.png` | 881379 | `4b64d7d086f919d7b25a1c0dadfbcb5cb60b67329b3982b6211827ce61dac3fb` |
| 83 | first_hour_operations | `ui_first_hour_operations.png` | 923525 | `9036ad9ee3ebd87532414717445396eb51e2a9e3e7786e17329ed86355d00e53` |
| 84 | battle_log | `ui_battle_log.png` | 662141 | `4a897614520f69efaa1ff786a1d997af006c70ab98da0bdacee3afb4c44da6bc` |
| 85 | combat_feedback | `ui_combat_feedback.png` | 1187209 | `7a7ae93accd35cf8717ff841803bc6d391fcbbf9ab528d13cb2211174b0b90d3` |
| 86 | phase_replay | `ui_phase_replay.png` | 1096891 | `aa1768f7c83b5dc23a4ef7c737a6b9efdfb743a9856c5b4dafdbcdcd089bbaf2` |
| 87 | end_phase_guard | `ui_end_phase_guard.png` | 1186911 | `90840bc360cfad2f8f0789ed0b940e04a2cba4bedd8295dbc8dac18f59a6851c` |
| 88 | readiness_markers | `ui_readiness_markers.png` | 1190922 | `5a2ecafea3e2a43d79d058704e8dd62ea182a04fc5a2e21ff536147f9e283226` |
| 89 | vitality_markers | `ui_vitality_markers.png` | 1191345 | `9a40438bcdbbfd4e97019bea09faf4c31fcdc4555421327f0da45d192921e27d` |
| 90 | extraction | `ui_extraction.png` | 1440317 | `df856144f544a5be29fe109837c51846448efdaa5d90c9a967ea3f5864975052` |
| 91 | variant | `ui_variant.png` | 1239074 | `5d6940b5661799c6426ebf2b0c434f109b6296e00a847469a00b56fd1a1a8029` |
| 92 | sporefield | `ui_sporefield.png` | 1191852 | `5dbbdb74d95781e0973c884bab43d90ed496220c6f04fa85e878b13e4cdfa02f` |
| 93 | vault | `ui_vault.png` | 1187316 | `d89c7e36ee9e9cc1deb011b93a2101b49a6b65a2f912eacdcb1323fb23896257` |
| 94 | black_channel | `ui_black_channel.png` | 1195552 | `1a3d43350cd47ea64404cceefffb5dfedcc1358a5c1a9b144a889f527c1ed04a` |
| 95 | living_chorus | `ui_living_chorus.png` | 1200128 | `553ab13e5ae961dd77de0e7608d7da271a9c5c437ff421355b27ceae42ac49f0` |
| 96 | open_circuit | `ui_open_circuit.png` | 1198961 | `f0b1c0081ceba826910d632659bf34af3fc09e499f34aff542c8490954575159` |
| 97 | trace_active | `ui_trace_active.png` | 1162807 | `d994dc76cd01729a50c6979711acb62e6ff08190beba29799dbee8ff557b83e7` |
| 98 | equipment | `ui_equipment.png` | 1022099 | `d5d02d3a31c6456433939cd084778991b3f3cffee49b8050c23097c20819d9f9` |
| 99 | weapon_profile | `ui_weapon_profile.png` | 1186834 | `061abfc585fc57e6b145005cded11ad88cccb1a8143fa2554a8f6f1a2c9ae6ac` |
| 100 | class_target | `ui_class_target.png` | 1039275 | `a95a4597ae527bf724759f59729fbe90d6fe60dd3282999225c367fd18e28b17` |
| 101 | breach | `ui_breach.png` | 1363056 | `a7d82774ea875e29cdd76fbb0c161ae629792a09986c1f36678ab1cdd373f057` |
| 102 | debrief | `ui_debrief.png` | 218275 | `c95526afa213d39b9505d45cad84cb774a826908cdb90e0cc9a9e13da10007f4` |
| 103 | trauma_debrief | `ui_trauma_debrief.png` | 201683 | `604954c2d0e8d328bd41c55132acbc81a6d0ca39a8f3755c40d686fa992254bd` |
