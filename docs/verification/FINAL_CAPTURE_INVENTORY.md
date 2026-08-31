# Mirexis Final Capture Inventory

Generated: 2026-08-31
Capture root: `docs/verification`
Manifest: `scripts/capture_ui.ps1`

## Integrity summary

- 102 manifest scenes and 102 canonical root PNG files; no scene is stored in a capture subfolder.
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
- `ui_settings.png`: persistent audio and readability controls show the current volume level, mute state, reduced-motion state, and return path.
- `ui_field_notes.png`: the Operations archive keeps chronological speaker, title, transcript, navigation, and return context visible.
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
| 6 | field_notes | `ui_field_notes.png` | 123985 | `2d9bf4c7641a4b04cc97d924bc6310e99e21707b0c0147f0bf28d178d91825d6` |
| 7 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739486 | `fc88587d80bfe9e55edcc3c015b272327a86843d487b91ca218e1c15fa314a07` |
| 8 | contact | `ui_contact.png` | 933781 | `cecd74e0b8bae3df27ed144c97d52ac37cb836c36a846691ff339759138104a3` |
| 9 | contact_gear | `ui_contact_gear.png` | 250666 | `4f696269042dbe8f2a6ce57e7d1b535f8f812ccf8aa127eb8d006da33e83361f` |
| 10 | contact_event | `ui_contact_event.png` | 920930 | `03d0ece88f3e67e72ecf5086c33e2601e36a7c209764928f4a7f8f5c349658cd` |
| 11 | adaptation | `ui_adaptation.png` | 895642 | `f0fca1356b9fb1569b3376cf63994ca931fccdfe68eee88df1d3748dce45c374` |
| 12 | gene_lab | `ui_gene_lab.png` | 182135 | `383b60cb4148f07415865dfc64677371d4a4ccbaffe740eef890fb5c0b735385` |
| 13 | evolution | `ui_evolution.png` | 245242 | `763f45a7ecb722a170258526d757373bff86c840923b87cfeba370fe27c23454` |
| 14 | mara_evolution | `ui_mara_evolution.png` | 180425 | `36aeac0e75ed9e988f7fd2b65dadff0d74efbf66b0ad347f2068f4516c580c1b` |
| 15 | ilya_evolution | `ui_ilya_evolution.png` | 180215 | `c7e600f48fabf0363275cd6a41eaa9af3ee22a42c1a10339eb97d6d8d58dab9d` |
| 16 | sol_evolution | `ui_sol_evolution.png` | 174924 | `d62ee19981a701bb8be3ccea1a7c08f55304fa193e8e14ad22977563030d0eb1` |
| 17 | nadi_evolution | `ui_nadi_evolution.png` | 185539 | `e4985cef48b0ee9eeb3a2a7a6b9419dee5dbe797b60b7912fa10c6fa75aa5eaa` |
| 18 | escalation | `ui_escalation.png` | 898359 | `518fe0cf06b352a435b5e37f51d197b19cfda23df4ea940fd65bb396a6813721` |
| 19 | escalation_operation | `ui_escalation_operation.png` | 204943 | `6632a48570fdbd5f7ebd0d42b7998486a343ca5788926f8ed2d5ef9084d1cec7` |
| 20 | escalation_response | `ui_escalation_response.png` | 939825 | `489f3126e8e2aaf508a6df5c520b699d0b027bfbce7475d09612cee40a79d819` |
| 21 | mirexis | `ui_mirexis.png` | 940147 | `a5038c41318aed154c229e144d3b624fe8d0affcbfdee8cc86d589a3b2d2b37b` |
| 22 | mirexis_path | `ui_mirexis_path.png` | 900716 | `3e705fba756a12ec891104e4bb5d9dbff61eed9cab3fce8f79e37318c421cf46` |
| 23 | redoubt_end | `ui_redoubt_end.png` | 920141 | `812edd411b7fd93db5e1f1cfe7d4e9386b6c7ba8b26ad6c23dae09c949087814` |
| 24 | commonwealth_end | `ui_commonwealth_end.png` | 929176 | `a90237b05f405ab4c5a0f9e8dc9a51eaaa6cfa1fc8713d412f1bfb3d7b5236cb` |
| 25 | threshold_end | `ui_threshold_end.png` | 930082 | `6247bfe4cafdccf8df9538418504cb1240e04bad79f7f83493bb666348bbca01` |
| 26 | finale_debrief | `ui_finale_debrief.png` | 197645 | `b402a19906425a1f4b7c7afa28a02e8fd5fe46f4cd03c8a6cd085332e4306f6c` |
| 27 | adaptation_operation | `ui_adaptation_operation.png` | 190472 | `c3238cb8edf0bca5ef764fcc92a36f65c3b33a35ba4ac6d3f297e8aa1677ee39` |
| 28 | glass_nerve | `ui_glass_nerve.png` | 1187625 | `be47f59f9bc832f87b9ea1bc75c066a766035ea7abf0bea0517fee3c3f13eaa5` |
| 29 | three_knives | `ui_three_knives.png` | 1191670 | `0fe138036709f224bc43fef9bf7c3b491370325d7310e83c8c27435d478fcd57` |
| 30 | reinforcement_warning | `ui_reinforcement_warning.png` | 1191998 | `34b13be007de1074f083b82b1cbf4e168ce794e5cd1e5afaffadbcf552ebf970` |
| 31 | line_formation | `ui_line_formation.png` | 1187341 | `e3ce2fe92650df49dbb325e7cad0733912e4794f764d9fa0898bff873bb77023` |
| 32 | thin_shelter | `ui_thin_shelter.png` | 1189236 | `60ac13a69009e0e548132894429a6d665a30b01f5468f5388f7d170ea2d35ad0` |
| 33 | breakwater | `ui_breakwater.png` | 1181508 | `2ec76a0cff2a6092cd352d8fb105d62987ae6a1e0b496dd045658c40690bc867` |
| 34 | false_heart | `ui_false_heart.png` | 1188785 | `89b06aac22c50895a3843f88fbed26952d84fcb3175011b7123ed806d7b41635` |
| 35 | live_wire | `ui_live_wire.png` | 1193148 | `1c7358a5e1448b0e7a13c0066ea3ba03a88a805c7c54391f2708d1152272c7d0` |
| 36 | last_wall | `ui_last_wall.png` | 1183105 | `c50db7bf474f70eddd1041b480465b9fa5cf898cf9eb386cccf32ec36546d699` |
| 37 | root_choir | `ui_root_choir.png` | 1192938 | `f84209f50425875535a038fc14ec2e16ac577e14c84de48caed15851612d81b6` |
| 38 | door_of_light | `ui_door_of_light.png` | 1185049 | `ee2ae5b4b427e307d599bc131454566aad0b1c0e6d2421c006a8dc9777c401cd` |
| 39 | damage | `ui_damage.png` | 883797 | `f73dcb5f4c20aded6d1ec6aafd955a27a4b5c49edf4120d942d893bbfcb90c98` |
| 40 | repair | `ui_repair.png` | 890681 | `8c8de85ec3a3625a7de2490d2b811923372f5764c4f63a183d4a706efea2ab76` |
| 41 | power | `ui_power.png` | 882465 | `f7a88dc5e75392f8203c3343c87c457678a3f64410ea33088ed02e3b6d5cf3c7` |
| 42 | construction | `ui_construction.png` | 884122 | `8f24b5e04d72349b879cccc7769105aee55e2495e24e0119533f2354712b4a07` |
| 43 | research | `ui_research.png` | 906937 | `30f36343c7cb0248a851c250a758361be43e8748d1330459ec319aea717efe2a` |
| 44 | roster | `ui_roster.png` | 249353 | `559d7eff409ce5f5cad77594de133ba36c941fa4367e8652ef0a631d75bfb22c` |
| 45 | recruited_roster | `ui_recruited_roster.png` | 281898 | `d11314213dc979688de505d3b3a9e51bef038366cdf6845b99ba58ede7b775d5` |
| 46 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210227 | `cfca7144c37db59649e1d1f204368654842d06f872b921f084b5331c4440b455` |
| 47 | advanced_roster | `ui_advanced_roster.png` | 249209 | `f19f302a71c4bda92f73556bc1be8546911579d4881b91997f353e8f2addeaa3` |
| 48 | relationships | `ui_relationships.png` | 251403 | `b8b0e68384df9d843635250104fadef4f1b065422b0f8e3fa01d0876bf9583df` |
| 49 | trauma | `ui_trauma.png` | 253370 | `5e0f6ff46ac49da9906353bbc4bc53c5355b61323c0f076781c0bda74aabf1dd` |
| 50 | bonded_briefing | `ui_bonded_briefing.png` | 200319 | `75699be9900eae2026b18c5a759b7144328134b4a11ccb8e0e49f8cc85f5d378` |
| 51 | legacy | `ui_legacy.png` | 250061 | `a7b62fadf4942838d82398cd2cafd605faeb7fc2cd09cd59a63c04cd86b41bc5` |
| 52 | briefing | `ui_briefing.png` | 209434 | `fc32d2208fe9447437c0769dae8e8ce55db8ba61dc2f3953b32cd9c476048721` |
| 53 | recruited_briefing | `ui_recruited_briefing.png` | 211759 | `91994f62c8c55b7cd33bfb9e5b268cc7749854732e3df68c9654817257a8dd91` |
| 54 | threat_briefing | `ui_threat_briefing.png` | 190349 | `77c25e354dfd2c7f451151fee7ca5d1f1c8182eb6ea1c909fde38c12e0860102` |
| 55 | loadout_briefing | `ui_loadout_briefing.png` | 199261 | `0c8c2bba22948f97d5947f618d0b4895f7816d8244a473efc24f8b549945f740` |
| 56 | pressure | `ui_pressure.png` | 204537 | `feb473941a1bac1e2c45326abbebc90f5c201037ad3477b1a1438e73d359740f` |
| 57 | gameplay | `ui_gameplay.png` | 1186733 | `6fbc07ddbd6ced9e97f9ed030387b728be8e7c6a7b79604a184cf433050423f3` |
| 58 | first_hour_tactical | `ui_first_hour_tactical.png` | 1192529 | `f0c0bfe806841d04d63759644c54d0b16cfa57ddfa65cb70ae6f0cc2a4a69fad` |
| 59 | first_hour_attack | `ui_first_hour_attack.png` | 1109096 | `40c989b6b8bdb39974c418dac4a759a0d2b6923394a28dbafbb25437447ce970` |
| 60 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1201564 | `8793f2686068d222cabb57ee48ea12be739b94b46de93bd6bec81c471889747d` |
| 61 | first_hour_objective | `ui_first_hour_objective.png` | 1344004 | `f63b852e4c45c7b1f959600e763ab46d5b966d80bd666f804a43b75159689192` |
| 62 | first_hour_replay | `ui_first_hour_replay.png` | 1102796 | `8d926f58c00a89443d24301c16b4b5171c27e92db29004acbb975fd3c876980a` |
| 63 | first_hour_ability | `ui_first_hour_ability.png` | 1191927 | `28879e151f7102915cefd57b2d9702ef608a681ff8906bb5f38f097ba51ff994` |
| 64 | second_operation_tactical | `ui_second_operation_tactical.png` | 1113988 | `b5cd545d93935cac4a6288e81d4c6d60184096583590d13f61d5fdf82e78ffd2` |
| 65 | overwatch | `ui_overwatch.png` | 1189800 | `800ef3f41e6c74130a645c75e63f8ba5d75f1a1820bb0a0c43606b476877581b` |
| 66 | brood_ability | `ui_brood_ability.png` | 1245293 | `6bfdfb3b6a7a48e526124f1fb1777b266b08394a1a4216f7d932e45aea97fed1` |
| 67 | directorate_ability | `ui_directorate_ability.png` | 1245392 | `8f88bf83d8fabbbaa51011b5f9aa5e65ba5c4f88f2b314949593abb12aa03be6` |
| 68 | ascendant_ability | `ui_ascendant_ability.png` | 1245684 | `bb52f33eb26c198c81b56902e2628dd6deca8aafe25c7eecace491e075887f79` |
| 69 | hazard | `ui_hazard.png` | 1251831 | `5450600560e6c664277f93dce19081cd809bc6c958eb71bb1f306b06a785e4b0` |
| 70 | intent | `ui_intent.png` | 1148560 | `57f63c2cfecc3a4eef4415bbb969ea0c46a7967c4adbfd7bb68f075bc9fdd686` |
| 71 | action_preview | `ui_action_preview.png` | 1111573 | `ea04f8b02b99dd7c4e17ea274fe8cc154584fe841ce002d2b907f380b78e1315` |
| 72 | movement_route | `ui_movement_route.png` | 1148111 | `fc50f088494f1fc91d0768c80f77abfb6c16b6c094ea51a84be8914c2ddd4e41` |
| 73 | cover_edges | `ui_cover_edges.png` | 1187778 | `b9844d420d794d061f16d27b81fa5fb880b14953cfb0bc9c46d2fdd251786a41` |
| 74 | invalid_command | `ui_invalid_command.png` | 1216275 | `09d9ce5618b52ff29a02bed3c6b18bb37bc09ead28c2c7a7d8da6f5ce22b0b0e` |
| 75 | valid_shot | `ui_valid_shot.png` | 1106997 | `dac1b570efa868a75039bc8d82491e7c73c6c2900975c900e07c60e64871cdc9` |
| 76 | threat_range | `ui_threat_range.png` | 1219388 | `3536012d5486632291b8450e84b61a6ccf70aeef4fd15a0894458c790a80c0e4` |
| 77 | danger_reach | `ui_danger_reach.png` | 1215638 | `2c78905489d526a38488115d4d915a419257308c85d58bf141b448d100094608` |
| 78 | help | `ui_help.png` | 359966 | `6da7acf520b16132c8a9a56800079bcfef92092c41e4da0332a39340c81f0c79` |
| 79 | first_hour_guide | `ui_first_hour_guide.png` | 215026 | `7683632999a50317effd606f7a1741790e7deb517bf0c7acffde4d4f5789e902` |
| 80 | first_hour_return | `ui_first_hour_return.png` | 878729 | `7298f2b6c3530423e31288fb6846397ea23070efa7a722d5938e246236727fd1` |
| 81 | first_hour_promise | `ui_first_hour_promise.png` | 881379 | `4b64d7d086f919d7b25a1c0dadfbcb5cb60b67329b3982b6211827ce61dac3fb` |
| 82 | first_hour_operations | `ui_first_hour_operations.png` | 921979 | `05c8031bdfb912b6880dd423a6fdd240150f1f41f43b1ac136270b2b9acdd68d` |
| 83 | battle_log | `ui_battle_log.png` | 662141 | `2f8bb16d3dfef3f812cc8e083905a9cce4153842d0c54fd76bc0ec6d9500cccc` |
| 84 | combat_feedback | `ui_combat_feedback.png` | 1187209 | `332ba438fbe0bd9d5d769d948768927182a3c50b96f8f5d206010ea70631b65d` |
| 85 | phase_replay | `ui_phase_replay.png` | 1096891 | `aa1768f7c83b5dc23a4ef7c737a6b9efdfb743a9856c5b4dafdbcdcd089bbaf2` |
| 86 | end_phase_guard | `ui_end_phase_guard.png` | 1186911 | `90840bc360cfad2f8f0789ed0b940e04a2cba4bedd8295dbc8dac18f59a6851c` |
| 87 | readiness_markers | `ui_readiness_markers.png` | 1190909 | `aa5c2907b0aac905281a609887c813ea6856eb69ef7b5fed7e183b56d8670ed0` |
| 88 | vitality_markers | `ui_vitality_markers.png` | 1191333 | `2aeb44413b4a15d58417bcaf8eede932ccc8925fda55ae484f99b6c537a54f3f` |
| 89 | extraction | `ui_extraction.png` | 1440314 | `d79b01048b2596733464d05206b521f7627d1ef8f64ec802db4e1183ea675986` |
| 90 | variant | `ui_variant.png` | 1239063 | `f7ada2e1c2ef292ca2bbaf3427f476f5450bd9c854589def6f5e19fe89211b2d` |
| 91 | sporefield | `ui_sporefield.png` | 1191852 | `5dbbdb74d95781e0973c884bab43d90ed496220c6f04fa85e878b13e4cdfa02f` |
| 92 | vault | `ui_vault.png` | 1187313 | `a0b16f5b43a5f61657740be3c6d72d18cb2b35123b05c2fff4e33bcf9ea4fced` |
| 93 | black_channel | `ui_black_channel.png` | 1195552 | `1a3d43350cd47ea64404cceefffb5dfedcc1358a5c1a9b144a889f527c1ed04a` |
| 94 | living_chorus | `ui_living_chorus.png` | 1200124 | `94efa214ce21133fc852a31e5dd925ef68f0b0da2cfda8598a92c59439d8799b` |
| 95 | open_circuit | `ui_open_circuit.png` | 1198961 | `e2cb103590642b87bec0dbd922e378b9dd567640e322fd3317acc62694e093b0` |
| 96 | trace_active | `ui_trace_active.png` | 1162807 | `d994dc76cd01729a50c6979711acb62e6ff08190beba29799dbee8ff557b83e7` |
| 97 | equipment | `ui_equipment.png` | 1020054 | `1c607720096783608046c539ef80641dd4a6255581b2c682a2189f87d90f4dcb` |
| 98 | weapon_profile | `ui_weapon_profile.png` | 1186834 | `061abfc585fc57e6b145005cded11ad88cccb1a8143fa2554a8f6f1a2c9ae6ac` |
| 99 | class_target | `ui_class_target.png` | 1039275 | `a95a4597ae527bf724759f59729fbe90d6fe60dd3282999225c367fd18e28b17` |
| 100 | breach | `ui_breach.png` | 1363059 | `fbad1035b2fdb662ee1aa32530352747add2e18ce37b0905bbc39249e79cfa29` |
| 101 | debrief | `ui_debrief.png` | 218275 | `c95526afa213d39b9505d45cad84cb774a826908cdb90e0cc9a9e13da10007f4` |
| 102 | trauma_debrief | `ui_trauma_debrief.png` | 201683 | `604954c2d0e8d328bd41c55132acbc81a6d0ca39a8f3755c40d686fa992254bd` |
