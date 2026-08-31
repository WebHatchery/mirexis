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
| 6 | field_notes | `ui_field_notes.png` | 124519 | `b2452fead59b62828f2a01cc0b6ce958b15a0deb060d47a6c4f68d0d09d84e4e` |
| 7 | memorial | `ui_memorial.png` | 180934 | `7c1c479fa50642e9a3eef2dbf0f1f574b948296fa116e32c4a78a313c8d45b47` |
| 8 | memorial_page_two | `ui_memorial_page_two.png` | 121869 | `3b5c6ee708effca00b2de8d1b913786d09f2695033496d4871b4232817aaa1b0` |
| 9 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739486 | `fc88587d80bfe9e55edcc3c015b272327a86843d487b91ca218e1c15fa314a07` |
| 10 | contact | `ui_contact.png` | 935945 | `50fed33aa575de2e0b6c5c2ef3c472868bc2b96a7792d8e2155fb8a0d13321d4` |
| 11 | contact_gear | `ui_contact_gear.png` | 250666 | `4f696269042dbe8f2a6ce57e7d1b535f8f812ccf8aa127eb8d006da33e83361f` |
| 12 | contact_event | `ui_contact_event.png` | 923388 | `e69291919556b4c99e5509e197111da1ee79643ed787912e8d1daaf17f675a10` |
| 13 | adaptation | `ui_adaptation.png` | 898088 | `db8bb7de4ddd46f8477d2a90c026dde24e78f4a521869f0411404229b145c6a9` |
| 14 | gene_lab | `ui_gene_lab.png` | 182135 | `383b60cb4148f07415865dfc64677371d4a4ccbaffe740eef890fb5c0b735385` |
| 15 | evolution | `ui_evolution.png` | 245242 | `763f45a7ecb722a170258526d757373bff86c840923b87cfeba370fe27c23454` |
| 16 | mara_evolution | `ui_mara_evolution.png` | 180425 | `36aeac0e75ed9e988f7fd2b65dadff0d74efbf66b0ad347f2068f4516c580c1b` |
| 17 | ilya_evolution | `ui_ilya_evolution.png` | 180215 | `c7e600f48fabf0363275cd6a41eaa9af3ee22a42c1a10339eb97d6d8d58dab9d` |
| 18 | sol_evolution | `ui_sol_evolution.png` | 174924 | `d62ee19981a701bb8be3ccea1a7c08f55304fa193e8e14ad22977563030d0eb1` |
| 19 | nadi_evolution | `ui_nadi_evolution.png` | 185539 | `e4985cef48b0ee9eeb3a2a7a6b9419dee5dbe797b60b7912fa10c6fa75aa5eaa` |
| 20 | escalation | `ui_escalation.png` | 900799 | `33ebcf61716855f2948d3967d14ecf1addb2ce373cabb93faf8b4692a5984d6d` |
| 21 | escalation_operation | `ui_escalation_operation.png` | 204929 | `a03ff625ea1d91895884f77b36d4960e9d3c477c15c48563a7b871863f97caa0` |
| 22 | escalation_response | `ui_escalation_response.png` | 942686 | `67e3ac0ec88ad64ac850a54ccfef8744186ba10b3c5654cd5b3b4719bda63672` |
| 23 | mirexis | `ui_mirexis.png` | 942883 | `50a19747a7d1da12fe0e54ac6bfe7f841dd4b0147057db6295c88612d85327ae` |
| 24 | mirexis_path | `ui_mirexis_path.png` | 903078 | `0424e2b0f60df69931c3b74758be9549a80dae4b35bc631854312c0541148fce` |
| 25 | redoubt_end | `ui_redoubt_end.png` | 922584 | `59ec69e107380a67524ee53f39dad7aec54c8b7811600dc88633b09e1a77f8f0` |
| 26 | commonwealth_end | `ui_commonwealth_end.png` | 931700 | `2e6dd8fc24bdd737f9294a9b39f56f4c802d07443616e5878620da88268751e0` |
| 27 | threshold_end | `ui_threshold_end.png` | 932647 | `a44881d6567408f4a66bc3164525ecafff96a2890b7050f9692855e6b5ec5473` |
| 28 | finale_debrief | `ui_finale_debrief.png` | 197645 | `b402a19906425a1f4b7c7afa28a02e8fd5fe46f4cd03c8a6cd085332e4306f6c` |
| 29 | adaptation_operation | `ui_adaptation_operation.png` | 190416 | `3db3df3bdadf1b4915501dc5de977b53822531da113d98c2e46107248402aa66` |
| 30 | glass_nerve | `ui_glass_nerve.png` | 1187625 | `be47f59f9bc832f87b9ea1bc75c066a766035ea7abf0bea0517fee3c3f13eaa5` |
| 31 | three_knives | `ui_three_knives.png` | 1191675 | `cc2fdb8579bf404b1a6ab3ff15c7af9eb6e71852fb94d8f3ed5126a3679ae150` |
| 32 | reinforcement_warning | `ui_reinforcement_warning.png` | 1192002 | `d7de3289adb630a9b57bbe04190c40e776f6677f36be8a5a3b22853c52e8a8da` |
| 33 | line_formation | `ui_line_formation.png` | 1187341 | `e3ce2fe92650df49dbb325e7cad0733912e4794f764d9fa0898bff873bb77023` |
| 34 | thin_shelter | `ui_thin_shelter.png` | 1189236 | `60ac13a69009e0e548132894429a6d665a30b01f5468f5388f7d170ea2d35ad0` |
| 35 | breakwater | `ui_breakwater.png` | 1181508 | `2ec76a0cff2a6092cd352d8fb105d62987ae6a1e0b496dd045658c40690bc867` |
| 36 | false_heart | `ui_false_heart.png` | 1188785 | `60152652b394b76f469294999964148a286668eaa76d5b58fb7c9a40536b0ee4` |
| 37 | live_wire | `ui_live_wire.png` | 1193148 | `9bdb5833c51371573bc62f987636d446a3b06a624cc9802d685cb63ffb70c78f` |
| 38 | last_wall | `ui_last_wall.png` | 1183105 | `c50db7bf474f70eddd1041b480465b9fa5cf898cf9eb386cccf32ec36546d699` |
| 39 | root_choir | `ui_root_choir.png` | 1192938 | `f84209f50425875535a038fc14ec2e16ac577e14c84de48caed15851612d81b6` |
| 40 | door_of_light | `ui_door_of_light.png` | 1185050 | `2b1fa872b64a63b989582b3cb185b1bacd8a6b62ab43c6fc01a2f99418e934ad` |
| 41 | damage | `ui_damage.png` | 883797 | `f73dcb5f4c20aded6d1ec6aafd955a27a4b5c49edf4120d942d893bbfcb90c98` |
| 42 | repair | `ui_repair.png` | 890681 | `8c8de85ec3a3625a7de2490d2b811923372f5764c4f63a183d4a706efea2ab76` |
| 43 | power | `ui_power.png` | 882465 | `f7a88dc5e75392f8203c3343c87c457678a3f64410ea33088ed02e3b6d5cf3c7` |
| 44 | construction | `ui_construction.png` | 884122 | `8f24b5e04d72349b879cccc7769105aee55e2495e24e0119533f2354712b4a07` |
| 45 | research | `ui_research.png` | 908909 | `6df0175559ef4324c7ebf7ebf83c2a70f221b51eeb5bf98e73368657308058af` |
| 46 | roster | `ui_roster.png` | 249353 | `559d7eff409ce5f5cad77594de133ba36c941fa4367e8652ef0a631d75bfb22c` |
| 47 | recruited_roster | `ui_recruited_roster.png` | 281940 | `52023f0363f69b1adb79ef3598b3f423d4ba0cc702691d13de6524ce9d8c1827` |
| 48 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210227 | `cfca7144c37db59649e1d1f204368654842d06f872b921f084b5331c4440b455` |
| 49 | advanced_roster | `ui_advanced_roster.png` | 249209 | `f19f302a71c4bda92f73556bc1be8546911579d4881b91997f353e8f2addeaa3` |
| 50 | relationships | `ui_relationships.png` | 251403 | `b8b0e68384df9d843635250104fadef4f1b065422b0f8e3fa01d0876bf9583df` |
| 51 | trauma | `ui_trauma.png` | 253370 | `5e0f6ff46ac49da9906353bbc4bc53c5355b61323c0f076781c0bda74aabf1dd` |
| 52 | bonded_briefing | `ui_bonded_briefing.png` | 200299 | `ac9ba46fc58894748231a6c43a7773e4612f8d8f512946c471af8a5e1760377f` |
| 53 | legacy | `ui_legacy.png` | 250061 | `a7b62fadf4942838d82398cd2cafd605faeb7fc2cd09cd59a63c04cd86b41bc5` |
| 54 | briefing | `ui_briefing.png` | 209420 | `ee52f0fc7c0ce015f749fbcae40721d46b4d3dcb8eee2fad84e8bf53bd90ce2b` |
| 55 | recruited_briefing | `ui_recruited_briefing.png` | 211698 | `ddf86b62e68ff8592c60f6ce59fec889ae0ee4c58d89dfc59e7f6794bde1fe88` |
| 56 | threat_briefing | `ui_threat_briefing.png` | 190335 | `a16428f73c02abc4a2913570f65f36b3837dcee5072e3e0083d076cfe2421501` |
| 57 | loadout_briefing | `ui_loadout_briefing.png` | 199247 | `84961582332f7c8c1f0658c4c7c17a60a0f0694869badf8147bbe7eeb6de0251` |
| 58 | pressure | `ui_pressure.png` | 204523 | `16f7b780e66205a6f3c171b705de03e451f439525ed0f429143da4c930a88213` |
| 59 | gameplay | `ui_gameplay.png` | 1186733 | `6fbc07ddbd6ced9e97f9ed030387b728be8e7c6a7b79604a184cf433050423f3` |
| 60 | first_hour_tactical | `ui_first_hour_tactical.png` | 1192530 | `0b1cf961fda80778e57e7f47849ffd6a6bf26fd7b001d103b0375ed95cb570d3` |
| 61 | first_hour_attack | `ui_first_hour_attack.png` | 1111689 | `493db6b3be095f3a5ad498688166a31e225ed8d4f6425ef8d67e2f6d335e188a` |
| 62 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1201564 | `8793f2686068d222cabb57ee48ea12be739b94b46de93bd6bec81c471889747d` |
| 63 | first_hour_objective | `ui_first_hour_objective.png` | 1344001 | `1fec90f526a2e258697528d6b2194562dec0c08e8e03fea31343d73d026b452a` |
| 64 | first_hour_replay | `ui_first_hour_replay.png` | 1102796 | `8d926f58c00a89443d24301c16b4b5171c27e92db29004acbb975fd3c876980a` |
| 65 | first_hour_ability | `ui_first_hour_ability.png` | 1191927 | `28879e151f7102915cefd57b2d9702ef608a681ff8906bb5f38f097ba51ff994` |
| 66 | second_operation_tactical | `ui_second_operation_tactical.png` | 1116581 | `5ee0308a14854bc7d10f3fdaeb85ecfa4862b8a1ad62a21ca03a0073e8401d47` |
| 67 | overwatch | `ui_overwatch.png` | 1189800 | `800ef3f41e6c74130a645c75e63f8ba5d75f1a1820bb0a0c43606b476877581b` |
| 68 | brood_ability | `ui_brood_ability.png` | 1245300 | `cfcdd882583cfe7db8880a73fba5030605470581debcbb6e3feb70666bd673ed` |
| 69 | directorate_ability | `ui_directorate_ability.png` | 1245385 | `1ab9ef38dec88b02c0e73e25fe18ef111b303e72c253d44ca47b951f0ddcbd2a` |
| 70 | ascendant_ability | `ui_ascendant_ability.png` | 1245681 | `766367ff1912cb2c08f76d8fa30d49763b8eb07ebf7745eeffd9ee78416cfb9f` |
| 71 | hazard | `ui_hazard.png` | 1251834 | `b5b5739d3af7edf0e3d4bdcaf94d8ccdb9448ee87d587d39f5abfd3b0c4029d4` |
| 72 | intent | `ui_intent.png` | 1152865 | `78f995f0311632605416a807c6f3df7821dc4f37b3c4b030b30608d84ed028f3` |
| 73 | action_preview | `ui_action_preview.png` | 1114174 | `35042c7fe53e7c91a320703f946bfd196e77ef19d634becafbce93898fd227e2` |
| 74 | movement_route | `ui_movement_route.png` | 1148111 | `fc50f088494f1fc91d0768c80f77abfb6c16b6c094ea51a84be8914c2ddd4e41` |
| 75 | cover_edges | `ui_cover_edges.png` | 1187778 | `b9844d420d794d061f16d27b81fa5fb880b14953cfb0bc9c46d2fdd251786a41` |
| 76 | invalid_command | `ui_invalid_command.png` | 1218865 | `72d9bf98b015d3839230597306b6f3844d77c46c06060745b4cba10f61811e52` |
| 77 | valid_shot | `ui_valid_shot.png` | 1109588 | `19c04939328007aecffe3d892b5d4085906bfaef87ca928ed9bcb2cc608fd1e1` |
| 78 | threat_range | `ui_threat_range.png` | 1221843 | `609b0855ecbbf90ece83eda17ab146eb197cf3f86405677683d8fb2a607ab9a8` |
| 79 | danger_reach | `ui_danger_reach.png` | 1218093 | `2c6ebe077a558386078556b8bec9a089a31dfe690f243f519895bf41c87dad02` |
| 80 | help | `ui_help.png` | 359966 | `6da7acf520b16132c8a9a56800079bcfef92092c41e4da0332a39340c81f0c79` |
| 81 | first_hour_guide | `ui_first_hour_guide.png` | 215026 | `7683632999a50317effd606f7a1741790e7deb517bf0c7acffde4d4f5789e902` |
| 82 | first_hour_return | `ui_first_hour_return.png` | 878729 | `7298f2b6c3530423e31288fb6846397ea23070efa7a722d5938e246236727fd1` |
| 83 | first_hour_promise | `ui_first_hour_promise.png` | 881379 | `4b64d7d086f919d7b25a1c0dadfbcb5cb60b67329b3982b6211827ce61dac3fb` |
| 84 | first_hour_operations | `ui_first_hour_operations.png` | 923501 | `f2fcaf384b67781d07c9aeb2c9f671078262480e5439fd50f1e03096d8551ba6` |
| 85 | battle_log | `ui_battle_log.png` | 662141 | `2f8bb16d3dfef3f812cc8e083905a9cce4153842d0c54fd76bc0ec6d9500cccc` |
| 86 | combat_feedback | `ui_combat_feedback.png` | 1187209 | `7a7ae93accd35cf8717ff841803bc6d391fcbbf9ab528d13cb2211174b0b90d3` |
| 87 | phase_replay | `ui_phase_replay.png` | 1096891 | `aa1768f7c83b5dc23a4ef7c737a6b9efdfb743a9856c5b4dafdbcdcd089bbaf2` |
| 88 | end_phase_guard | `ui_end_phase_guard.png` | 1186911 | `90840bc360cfad2f8f0789ed0b940e04a2cba4bedd8295dbc8dac18f59a6851c` |
| 89 | readiness_markers | `ui_readiness_markers.png` | 1190918 | `3c42cfd98c32bc5f5fb1cc44e2319175ad5f508fdc553a5619c3a95ff3fca7e0` |
| 90 | vitality_markers | `ui_vitality_markers.png` | 1191342 | `4ee273579937e8a40a3661058e22e39b0b60dc3a64db521238f5d5f792a15f25` |
| 91 | extraction | `ui_extraction.png` | 1440319 | `f7cf2781a900175fc518e890e269245fd492da846151ce59d247ed8625dc209e` |
| 92 | variant | `ui_variant.png` | 1239067 | `235c0ff149f0153a4cce1958144044b9cec79adccdf8e0c209fe77ce2fceedc4` |
| 93 | sporefield | `ui_sporefield.png` | 1191852 | `5dbbdb74d95781e0973c884bab43d90ed496220c6f04fa85e878b13e4cdfa02f` |
| 94 | vault | `ui_vault.png` | 1187315 | `33b84cac167794d13bb14a689549dbc6e25c6d09542efa2d97022d2efc08994b` |
| 95 | black_channel | `ui_black_channel.png` | 1195552 | `1a3d43350cd47ea64404cceefffb5dfedcc1358a5c1a9b144a889f527c1ed04a` |
| 96 | living_chorus | `ui_living_chorus.png` | 1200128 | `f789357097a6e8bde712417248485efc25ee813eeae2c64c00055e28aa20a3ab` |
| 97 | open_circuit | `ui_open_circuit.png` | 1198961 | `f0b1c0081ceba826910d632659bf34af3fc09e499f34aff542c8490954575159` |
| 98 | trace_active | `ui_trace_active.png` | 1162807 | `d994dc76cd01729a50c6979711acb62e6ff08190beba29799dbee8ff557b83e7` |
| 99 | equipment | `ui_equipment.png` | 1022100 | `f580457ee339489d2ee47f684b1122cee2fcb5126718db5a1350266677217e62` |
| 100 | weapon_profile | `ui_weapon_profile.png` | 1186834 | `061abfc585fc57e6b145005cded11ad88cccb1a8143fa2554a8f6f1a2c9ae6ac` |
| 101 | class_target | `ui_class_target.png` | 1039275 | `a95a4597ae527bf724759f59729fbe90d6fe60dd3282999225c367fd18e28b17` |
| 102 | breach | `ui_breach.png` | 1363058 | `9bd9879be293a03928404ecfc916cc7beb791c23da6fcb59c3dcb9de722692ff` |
| 103 | debrief | `ui_debrief.png` | 218275 | `c95526afa213d39b9505d45cad84cb774a826908cdb90e0cc9a9e13da10007f4` |
| 104 | trauma_debrief | `ui_trauma_debrief.png` | 201683 | `604954c2d0e8d328bd41c55132acbc81a6d0ca39a8f3755c40d686fa992254bd` |
