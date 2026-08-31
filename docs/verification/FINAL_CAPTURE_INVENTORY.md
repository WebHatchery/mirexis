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
| 1 | title | `ui_title.png` | 441384 | `dfc5d8e493afe2853b512e305d21b1a0d478f961fd2af99adc666d0a16e5d843` |
| 2 | title_controller | `ui_title_controller.png` | 443845 | `4a7478afd3002516eab073145fd6c2042827d6e30a133a43980ad8d416c87bde` |
| 3 | title_hover | `ui_title_hover.png` | 441362 | `6497b59ed1afb8904a8af3cb1998171716cf7a958b71f8bf76b9034a67f070f8` |
| 4 | colony | `ui_colony.png` | 879197 | `0d243d778864aeae362d4cfe4bd31bf7626e3f6b623b989ddf776fa708ead902` |
| 5 | settings | `ui_settings.png` | 230014 | `d97d87daedfa342142fc669ee19164de8b981f214a0781ad8be4dd52e887e0df` |
| 6 | field_notes | `ui_field_notes.png` | 123986 | `13254245437de7060f4524f5ac99ab780d0647c3d74e8a52c28be759d6c112b9` |
| 7 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739486 | `fc88587d80bfe9e55edcc3c015b272327a86843d487b91ca218e1c15fa314a07` |
| 8 | contact | `ui_contact.png` | 934236 | `81a400f223b589b1380ccd83ec3df2bf3981547a1075e3ec2489b2ca550786e7` |
| 9 | contact_gear | `ui_contact_gear.png` | 250666 | `4f696269042dbe8f2a6ce57e7d1b535f8f812ccf8aa127eb8d006da33e83361f` |
| 10 | contact_event | `ui_contact_event.png` | 921193 | `0ace3aa30fabaa407a4cf5e3160157e689abf39d3ae3a681ae9767cb95d0f470` |
| 11 | adaptation | `ui_adaptation.png` | 895906 | `ab335691e4c1beab44c597854deef34be600939c13cc054ec115247da3b4d0c5` |
| 12 | gene_lab | `ui_gene_lab.png` | 182135 | `383b60cb4148f07415865dfc64677371d4a4ccbaffe740eef890fb5c0b735385` |
| 13 | evolution | `ui_evolution.png` | 245242 | `763f45a7ecb722a170258526d757373bff86c840923b87cfeba370fe27c23454` |
| 14 | mara_evolution | `ui_mara_evolution.png` | 180425 | `36aeac0e75ed9e988f7fd2b65dadff0d74efbf66b0ad347f2068f4516c580c1b` |
| 15 | ilya_evolution | `ui_ilya_evolution.png` | 180215 | `c7e600f48fabf0363275cd6a41eaa9af3ee22a42c1a10339eb97d6d8d58dab9d` |
| 16 | sol_evolution | `ui_sol_evolution.png` | 174924 | `d62ee19981a701bb8be3ccea1a7c08f55304fa193e8e14ad22977563030d0eb1` |
| 17 | nadi_evolution | `ui_nadi_evolution.png` | 185539 | `e4985cef48b0ee9eeb3a2a7a6b9419dee5dbe797b60b7912fa10c6fa75aa5eaa` |
| 18 | escalation | `ui_escalation.png` | 898616 | `f66d2c142532655c8938053a83f38e23dd3f4470e95832679dd7b0acf7450fdc` |
| 19 | escalation_operation | `ui_escalation_operation.png` | 204997 | `75f90c897e359ed5cc246d3f5df55690090a19addd39ebd3c61cbd7aa5ad5874` |
| 20 | escalation_response | `ui_escalation_response.png` | 940515 | `1e268374514525f5724b62b65c0c21f58571c01d7983b3ccd996ea438ebe202f` |
| 21 | mirexis | `ui_mirexis.png` | 940700 | `900354937ede05868b13218b28b2ec473fe87791b4f35d8282cfa3b469e5d2cb` |
| 22 | mirexis_path | `ui_mirexis_path.png` | 900895 | `b8f1675c6fa18759a27e56638bdd7dd28bea41c34193882e751c43f513fd129d` |
| 23 | redoubt_end | `ui_redoubt_end.png` | 920397 | `feac6e586eea372772317b3097ea3f5f44ce425cab9f3b11c1749cdb1ccf8069` |
| 24 | commonwealth_end | `ui_commonwealth_end.png` | 929518 | `aa9617dfb21df242693cd8b8f4c36df53584da50d44d90e1f8117f746bb360c7` |
| 25 | threshold_end | `ui_threshold_end.png` | 930464 | `9e7ade0d7d4f4b309db63138d4a4b051113decf90ffa39ba8d55368ca55c214c` |
| 26 | finale_debrief | `ui_finale_debrief.png` | 197645 | `b402a19906425a1f4b7c7afa28a02e8fd5fe46f4cd03c8a6cd085332e4306f6c` |
| 27 | adaptation_operation | `ui_adaptation_operation.png` | 190557 | `a6ea5372fcb95829bee6ef5505b2e9bd9224b8ae140a296b2f9c522e073b2e7e` |
| 28 | glass_nerve | `ui_glass_nerve.png` | 1187625 | `be47f59f9bc832f87b9ea1bc75c066a766035ea7abf0bea0517fee3c3f13eaa5` |
| 29 | three_knives | `ui_three_knives.png` | 1191670 | `435afa0946e02124561edbd9bee792f1a8b7b0782e59bf8b71d5dbcd4dc1683b` |
| 30 | reinforcement_warning | `ui_reinforcement_warning.png` | 1191998 | `8f2bb43705416e5a7c0a58c602c41d559af198a9cc4adedbe76ba8b6db18369e` |
| 31 | line_formation | `ui_line_formation.png` | 1187341 | `e3ce2fe92650df49dbb325e7cad0733912e4794f764d9fa0898bff873bb77023` |
| 32 | thin_shelter | `ui_thin_shelter.png` | 1189236 | `60ac13a69009e0e548132894429a6d665a30b01f5468f5388f7d170ea2d35ad0` |
| 33 | breakwater | `ui_breakwater.png` | 1181513 | `a9e344a0d0f5521d2bcc05e162e8439bd0808f596fb354d890b1be7fe007e164` |
| 34 | false_heart | `ui_false_heart.png` | 1188785 | `89b06aac22c50895a3843f88fbed26952d84fcb3175011b7123ed806d7b41635` |
| 35 | live_wire | `ui_live_wire.png` | 1193148 | `5519060cd42521bc79b4480235329c8fe10e4bf0f5279a095a84fac39388de72` |
| 36 | last_wall | `ui_last_wall.png` | 1183105 | `c50db7bf474f70eddd1041b480465b9fa5cf898cf9eb386cccf32ec36546d699` |
| 37 | root_choir | `ui_root_choir.png` | 1192938 | `f84209f50425875535a038fc14ec2e16ac577e14c84de48caed15851612d81b6` |
| 38 | door_of_light | `ui_door_of_light.png` | 1185049 | `f634364931691760ed752b678377ca8a2659afe65b2956e4ae2d1c5602b9cf4d` |
| 39 | damage | `ui_damage.png` | 883797 | `f73dcb5f4c20aded6d1ec6aafd955a27a4b5c49edf4120d942d893bbfcb90c98` |
| 40 | repair | `ui_repair.png` | 890681 | `8c8de85ec3a3625a7de2490d2b811923372f5764c4f63a183d4a706efea2ab76` |
| 41 | power | `ui_power.png` | 882465 | `f7a88dc5e75392f8203c3343c87c457678a3f64410ea33088ed02e3b6d5cf3c7` |
| 42 | construction | `ui_construction.png` | 884122 | `8f24b5e04d72349b879cccc7769105aee55e2495e24e0119533f2354712b4a07` |
| 43 | research | `ui_research.png` | 907200 | `b4b43295b0c4cd5aaa9c2e54a4205bb51cd218d88323de9e87ad16cde7526339` |
| 44 | roster | `ui_roster.png` | 249353 | `559d7eff409ce5f5cad77594de133ba36c941fa4367e8652ef0a631d75bfb22c` |
| 45 | recruited_roster | `ui_recruited_roster.png` | 281971 | `668dc94b80ee0c7cc1861073b4b7db0bc7ce73ca728839404e436d539529545f` |
| 46 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210301 | `6a4f1d76146218377d6e8a2acc08a9e4c5a145c271dafebcdcc890722cd34dfc` |
| 47 | advanced_roster | `ui_advanced_roster.png` | 249209 | `f19f302a71c4bda92f73556bc1be8546911579d4881b91997f353e8f2addeaa3` |
| 48 | relationships | `ui_relationships.png` | 251403 | `b8b0e68384df9d843635250104fadef4f1b065422b0f8e3fa01d0876bf9583df` |
| 49 | trauma | `ui_trauma.png` | 253370 | `5e0f6ff46ac49da9906353bbc4bc53c5355b61323c0f076781c0bda74aabf1dd` |
| 50 | bonded_briefing | `ui_bonded_briefing.png` | 200405 | `f26b8e3bd29759488afaaae5039a060102a89f9f2a3f8d10762ae4b66add81e3` |
| 51 | legacy | `ui_legacy.png` | 250061 | `a7b62fadf4942838d82398cd2cafd605faeb7fc2cd09cd59a63c04cd86b41bc5` |
| 52 | briefing | `ui_briefing.png` | 209518 | `dca0b5f14c7f60b05892626c8d8a19c4a6438feed547657139df0450e4d74c02` |
| 53 | recruited_briefing | `ui_recruited_briefing.png` | 211770 | `5f1c37940b808b6c9c73a1b135a5195c63071b83ad5b985eab78a57bb6a697fa` |
| 54 | threat_briefing | `ui_threat_briefing.png` | 190434 | `05ac99885be1398d2159878bdc1d899ae27e3b68a1ed35b86fbf3dc3c484c8ad` |
| 55 | loadout_briefing | `ui_loadout_briefing.png` | 199346 | `0044de1a978fa153cece9f0fa8f62eacb1e167c33c8cec0d8c080a4de3a9e1b3` |
| 56 | pressure | `ui_pressure.png` | 204622 | `86ce78ca80ca73196c745eae417fe2abc86d56c7c12bd997ef53a307f1b90edc` |
| 57 | gameplay | `ui_gameplay.png` | 1186733 | `6fbc07ddbd6ced9e97f9ed030387b728be8e7c6a7b79604a184cf433050423f3` |
| 58 | first_hour_tactical | `ui_first_hour_tactical.png` | 1192528 | `fc55c00407f57f0171b361aff63eadcadb41540198cf4a1ae3a0330a4a07e13a` |
| 59 | first_hour_attack | `ui_first_hour_attack.png` | 1109097 | `b33429962572d000be60d0518e9f0310f295b1b7dd9c962f590d9cc7fa2583f2` |
| 60 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1201564 | `8793f2686068d222cabb57ee48ea12be739b94b46de93bd6bec81c471889747d` |
| 61 | first_hour_objective | `ui_first_hour_objective.png` | 1344000 | `ff19c774e1fd5202b092a8256b248293a0d36b204117014dfd56873fa8125f77` |
| 62 | first_hour_replay | `ui_first_hour_replay.png` | 1102796 | `8d926f58c00a89443d24301c16b4b5171c27e92db29004acbb975fd3c876980a` |
| 63 | first_hour_ability | `ui_first_hour_ability.png` | 1191927 | `28879e151f7102915cefd57b2d9702ef608a681ff8906bb5f38f097ba51ff994` |
| 64 | second_operation_tactical | `ui_second_operation_tactical.png` | 1113988 | `535b0babd805222f5b6269e665872cc8b6991045e7f5707614663815df72deaf` |
| 65 | overwatch | `ui_overwatch.png` | 1189800 | `800ef3f41e6c74130a645c75e63f8ba5d75f1a1820bb0a0c43606b476877581b` |
| 66 | brood_ability | `ui_brood_ability.png` | 1245292 | `56a05ee506c9bc137835ca2c6bacc7050256c1ece321fbd01378cc5f68085249` |
| 67 | directorate_ability | `ui_directorate_ability.png` | 1245394 | `0f852c611ddfda8a12b1e5cde41487dd15ddd4b873d67872f152ab3892e78e06` |
| 68 | ascendant_ability | `ui_ascendant_ability.png` | 1245683 | `b6bd6b2aa3d68655199f3a162650dde14f0c6192f7a905f61db016ca8e3febd6` |
| 69 | hazard | `ui_hazard.png` | 1251830 | `f6b613e37230d1f7a468a288b72e9f491b62e4221a9f68c8ab28dff9165b7795` |
| 70 | intent | `ui_intent.png` | 1148559 | `87534d47945b781d53679c041e39566ed663852a6160c0aae6bbe916f4b6e712` |
| 71 | action_preview | `ui_action_preview.png` | 1111573 | `32db52efab6126d9d16d0db1f97d26a6b3b9c8bafddeb3d9cdc502b1f682ed2f` |
| 72 | movement_route | `ui_movement_route.png` | 1148111 | `fc50f088494f1fc91d0768c80f77abfb6c16b6c094ea51a84be8914c2ddd4e41` |
| 73 | cover_edges | `ui_cover_edges.png` | 1187778 | `b9844d420d794d061f16d27b81fa5fb880b14953cfb0bc9c46d2fdd251786a41` |
| 74 | invalid_command | `ui_invalid_command.png` | 1216272 | `fba6b0077ee329f2c403a01f5099b92ad874a12fbff4c6eaa07815e0bef00dd9` |
| 75 | valid_shot | `ui_valid_shot.png` | 1106997 | `593b4efdc0bb8133d1f1b0804fce91a9cb267df5517b441b0a26c9c0ac46f8b3` |
| 76 | threat_range | `ui_threat_range.png` | 1219385 | `59be1c868df0df73ba98a85b6ca4af34af9d11d360f9b9bfd9d0bd03de647e0d` |
| 77 | danger_reach | `ui_danger_reach.png` | 1215635 | `726b815b630e1cbebc190ee805d6c117dd32d8c638a1596c37b63d9d8469f293` |
| 78 | help | `ui_help.png` | 359966 | `6da7acf520b16132c8a9a56800079bcfef92092c41e4da0332a39340c81f0c79` |
| 79 | first_hour_guide | `ui_first_hour_guide.png` | 215026 | `7683632999a50317effd606f7a1741790e7deb517bf0c7acffde4d4f5789e902` |
| 80 | first_hour_return | `ui_first_hour_return.png` | 878729 | `7298f2b6c3530423e31288fb6846397ea23070efa7a722d5938e246236727fd1` |
| 81 | first_hour_promise | `ui_first_hour_promise.png` | 881379 | `4b64d7d086f919d7b25a1c0dadfbcb5cb60b67329b3982b6211827ce61dac3fb` |
| 82 | first_hour_operations | `ui_first_hour_operations.png` | 921784 | `6582d07ed3cf6c2d72beb86148ce0fca6adfb5c5bc2c1492521b5707ff7b0b6e` |
| 83 | battle_log | `ui_battle_log.png` | 662141 | `2f8bb16d3dfef3f812cc8e083905a9cce4153842d0c54fd76bc0ec6d9500cccc` |
| 84 | combat_feedback | `ui_combat_feedback.png` | 1187209 | `332ba438fbe0bd9d5d769d948768927182a3c50b96f8f5d206010ea70631b65d` |
| 85 | phase_replay | `ui_phase_replay.png` | 1096891 | `aa1768f7c83b5dc23a4ef7c737a6b9efdfb743a9856c5b4dafdbcdcd089bbaf2` |
| 86 | end_phase_guard | `ui_end_phase_guard.png` | 1186911 | `90840bc360cfad2f8f0789ed0b940e04a2cba4bedd8295dbc8dac18f59a6851c` |
| 87 | readiness_markers | `ui_readiness_markers.png` | 1190922 | `5a2ecafea3e2a43d79d058704e8dd62ea182a04fc5a2e21ff536147f9e283226` |
| 88 | vitality_markers | `ui_vitality_markers.png` | 1191345 | `9a40438bcdbbfd4e97019bea09faf4c31fcdc4555421327f0da45d192921e27d` |
| 89 | extraction | `ui_extraction.png` | 1440310 | `99dcbc9573af9be6eacaa4b1e5fed95e363f7d80168eb1b7217ad7be64175a4d` |
| 90 | variant | `ui_variant.png` | 1239064 | `5590c5fa7038bb0b30f2f64c3ee3564c20fb1c98c83392bfa1b8aa5e7224eb43` |
| 91 | sporefield | `ui_sporefield.png` | 1191852 | `5dbbdb74d95781e0973c884bab43d90ed496220c6f04fa85e878b13e4cdfa02f` |
| 92 | vault | `ui_vault.png` | 1187315 | `4d8306b3f71cbf1ff5545678f75772e2b8b4da42395d04c608b55ba85703f3a0` |
| 93 | black_channel | `ui_black_channel.png` | 1195552 | `1a3d43350cd47ea64404cceefffb5dfedcc1358a5c1a9b144a889f527c1ed04a` |
| 94 | living_chorus | `ui_living_chorus.png` | 1200124 | `042bccba8c92582d68fc4f44bf221c46e6a505c8391e454e1aaae87ae8e18b7a` |
| 95 | open_circuit | `ui_open_circuit.png` | 1198961 | `f0b1c0081ceba826910d632659bf34af3fc09e499f34aff542c8490954575159` |
| 96 | trace_active | `ui_trace_active.png` | 1162807 | `d994dc76cd01729a50c6979711acb62e6ff08190beba29799dbee8ff557b83e7` |
| 97 | equipment | `ui_equipment.png` | 1020054 | `d56c44a93d5181e715630ba73ee2d4fb58d83a4624d3122a537ab1e976d291eb` |
| 98 | weapon_profile | `ui_weapon_profile.png` | 1186834 | `061abfc585fc57e6b145005cded11ad88cccb1a8143fa2554a8f6f1a2c9ae6ac` |
| 99 | class_target | `ui_class_target.png` | 1039275 | `a95a4597ae527bf724759f59729fbe90d6fe60dd3282999225c367fd18e28b17` |
| 100 | breach | `ui_breach.png` | 1363056 | `ec888c84bcef10229d328e7cd6079e4748c546ab696b70d02f685aefd97b5582` |
| 101 | debrief | `ui_debrief.png` | 218275 | `c95526afa213d39b9505d45cad84cb774a826908cdb90e0cc9a9e13da10007f4` |
| 102 | trauma_debrief | `ui_trauma_debrief.png` | 201683 | `604954c2d0e8d328bd41c55132acbc81a6d0ca39a8f3755c40d686fa992254bd` |
