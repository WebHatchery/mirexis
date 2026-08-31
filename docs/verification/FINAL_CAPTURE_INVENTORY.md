# Mirexis Final Capture Inventory

Generated: 2026-08-31
Capture root: `docs/verification`
Manifest: `scripts/capture_ui.ps1`

## Integrity summary

- 102 manifest scenes and 102 canonical root PNG files; no scene is stored in a capture subfolder.
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
| 6 | field_notes | `ui_field_notes.png` | 123992 | `3c47d9fad111c738323f2430cc4aa448eb1ce7077197887d4b0aadab1bb86458` |
| 7 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 739486 | `fc88587d80bfe9e55edcc3c015b272327a86843d487b91ca218e1c15fa314a07` |
| 8 | contact | `ui_contact.png` | 934271 | `f32d908845897894c1a8f75a1ca50e7bcbebbc582e80f49fe5f34804474668b6` |
| 9 | contact_gear | `ui_contact_gear.png` | 250666 | `78f11ada2890beb4d7c638268dc90aa8ea9f23259c2b6a3690e8e8732f35a67e` |
| 10 | contact_event | `ui_contact_event.png` | 921228 | `13e5c68394c20adf906d6d1b1b695a949a8c30a755650820adae3d31533a008d` |
| 11 | adaptation | `ui_adaptation.png` | 895940 | `202e6bf152911c2820abfd47c96643a00684b81e5143369984716a60cbe2aa1f` |
| 12 | gene_lab | `ui_gene_lab.png` | 182135 | `383b60cb4148f07415865dfc64677371d4a4ccbaffe740eef890fb5c0b735385` |
| 13 | evolution | `ui_evolution.png` | 245242 | `763f45a7ecb722a170258526d757373bff86c840923b87cfeba370fe27c23454` |
| 14 | mara_evolution | `ui_mara_evolution.png` | 180425 | `36aeac0e75ed9e988f7fd2b65dadff0d74efbf66b0ad347f2068f4516c580c1b` |
| 15 | ilya_evolution | `ui_ilya_evolution.png` | 180215 | `c7e600f48fabf0363275cd6a41eaa9af3ee22a42c1a10339eb97d6d8d58dab9d` |
| 16 | sol_evolution | `ui_sol_evolution.png` | 174924 | `d62ee19981a701bb8be3ccea1a7c08f55304fa193e8e14ad22977563030d0eb1` |
| 17 | nadi_evolution | `ui_nadi_evolution.png` | 185539 | `e4985cef48b0ee9eeb3a2a7a6b9419dee5dbe797b60b7912fa10c6fa75aa5eaa` |
| 18 | escalation | `ui_escalation.png` | 898650 | `bab7320ddb87197ca0a300c7b37efa3baf59beaa9d4db1a09b9c497491405823` |
| 19 | escalation_operation | `ui_escalation_operation.png` | 204915 | `4fb71465eab6287385909a8f5c5bb6474b012ee7d19f9b8ceffc3ed783165ca2` |
| 20 | escalation_response | `ui_escalation_response.png` | 940620 | `7565e46b5a4b0381571ad5d5298d6f51ee003bba003c58a382f8c3456d001a10` |
| 21 | mirexis | `ui_mirexis.png` | 940735 | `a6c184829ed7d4a47f171e8d9ae59e27d0327b3bc679a54c44d6ce8be0f29d68` |
| 22 | mirexis_path | `ui_mirexis_path.png` | 900930 | `b505626043a18dda3ef5ba697d2779dc1b15751575cdac6e9c2abc396bab901e` |
| 23 | redoubt_end | `ui_redoubt_end.png` | 920431 | `fb721609bdcd8eea70d3c725d0a81427e2e4de7fd5edc508906fc49ebbfd8915` |
| 24 | commonwealth_end | `ui_commonwealth_end.png` | 929552 | `509ba8136542e125d4ce76c67b76b2ff9cccd212ab68697e54be0da0ff807bdc` |
| 25 | threshold_end | `ui_threshold_end.png` | 930498 | `4e027644b05c61b1a6d5c28e64a7c3a3a0e5725c842cc32f02c96648b7490fb8` |
| 26 | finale_debrief | `ui_finale_debrief.png` | 197645 | `b402a19906425a1f4b7c7afa28a02e8fd5fe46f4cd03c8a6cd085332e4306f6c` |
| 27 | adaptation_operation | `ui_adaptation_operation.png` | 190444 | `a6d6f6bcc2718cfb4de050fb6db58e7ae40a525855bf0500f0c14340f8db7857` |
| 28 | glass_nerve | `ui_glass_nerve.png` | 1187625 | `be47f59f9bc832f87b9ea1bc75c066a766035ea7abf0bea0517fee3c3f13eaa5` |
| 29 | three_knives | `ui_three_knives.png` | 1191670 | `435afa0946e02124561edbd9bee792f1a8b7b0782e59bf8b71d5dbcd4dc1683b` |
| 30 | reinforcement_warning | `ui_reinforcement_warning.png` | 1191998 | `8f2bb43705416e5a7c0a58c602c41d559af198a9cc4adedbe76ba8b6db18369e` |
| 31 | line_formation | `ui_line_formation.png` | 1187341 | `e3ce2fe92650df49dbb325e7cad0733912e4794f764d9fa0898bff873bb77023` |
| 32 | thin_shelter | `ui_thin_shelter.png` | 1189236 | `c76b70d6a91b95a88c70bd7d84fa398f86e98557311326403b7a76a2c2c8a110` |
| 33 | breakwater | `ui_breakwater.png` | 1181508 | `2ec76a0cff2a6092cd352d8fb105d62987ae6a1e0b496dd045658c40690bc867` |
| 34 | false_heart | `ui_false_heart.png` | 1188785 | `89b06aac22c50895a3843f88fbed26952d84fcb3175011b7123ed806d7b41635` |
| 35 | live_wire | `ui_live_wire.png` | 1193148 | `5519060cd42521bc79b4480235329c8fe10e4bf0f5279a095a84fac39388de72` |
| 36 | last_wall | `ui_last_wall.png` | 1183105 | `182109c1e9005b20793e1c7add0d57804a0f40809f1da4d3acbf78ce606ceaf4` |
| 37 | root_choir | `ui_root_choir.png` | 1192938 | `f84209f50425875535a038fc14ec2e16ac577e14c84de48caed15851612d81b6` |
| 38 | door_of_light | `ui_door_of_light.png` | 1185049 | `f634364931691760ed752b678377ca8a2659afe65b2956e4ae2d1c5602b9cf4d` |
| 39 | damage | `ui_damage.png` | 883797 | `f73dcb5f4c20aded6d1ec6aafd955a27a4b5c49edf4120d942d893bbfcb90c98` |
| 40 | repair | `ui_repair.png` | 890681 | `f7c1c2af8f4bb2f7d8eb4282cbd5102d7de9606dd8ed86e608752fe367862266` |
| 41 | power | `ui_power.png` | 882465 | `f7a88dc5e75392f8203c3343c87c457678a3f64410ea33088ed02e3b6d5cf3c7` |
| 42 | construction | `ui_construction.png` | 884122 | `8f24b5e04d72349b879cccc7769105aee55e2495e24e0119533f2354712b4a07` |
| 43 | research | `ui_research.png` | 907235 | `eca9d88e79481472734d398531bb06efa8bcded7c6167894f26c25059bafaf06` |
| 44 | roster | `ui_roster.png` | 249353 | `0250ae92bea8c9c642231c91b33d5fdcf427004b574343d038299103d7ac8787` |
| 45 | recruited_roster | `ui_recruited_roster.png` | 281867 | `85fccb20526b48652191329bb1d6d4ca6ba78b58728bc5cc6f2d08858cbfbb75` |
| 46 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210227 | `cfca7144c37db59649e1d1f204368654842d06f872b921f084b5331c4440b455` |
| 47 | advanced_roster | `ui_advanced_roster.png` | 249209 | `f19f302a71c4bda92f73556bc1be8546911579d4881b91997f353e8f2addeaa3` |
| 48 | relationships | `ui_relationships.png` | 251402 | `b0893d4ec7199e24ab38bad721401c3c4a01b53c18595eddc2f9a6ce69e44e71` |
| 49 | trauma | `ui_trauma.png` | 253370 | `aed24d7e55f4f4bfc956aec502e2d1989da1847197228692c668d1a2179d2f71` |
| 50 | bonded_briefing | `ui_bonded_briefing.png` | 200285 | `b90aa90d890fd45ddcc04342637a3ca831bea904bc3bbbb3388c73925da83b9b` |
| 51 | legacy | `ui_legacy.png` | 250061 | `c35e8546cdf906bfc93d0835188d3bd88afcea217a133a21e7635afe076403ad` |
| 52 | briefing | `ui_briefing.png` | 209406 | `a296cae1c5fa6e32d328b2ae81e3b221fd235aae8e0437edd7ca99e90f2b6515` |
| 53 | recruited_briefing | `ui_recruited_briefing.png` | 211755 | `df68d31c698293699b0a69b445215ba67278faa604478e5f9083f6693723ed11` |
| 54 | threat_briefing | `ui_threat_briefing.png` | 190321 | `d72ba7ecf0228ab1ee513d122e98f816058abbcac7d40079609e13413553ff0c` |
| 55 | loadout_briefing | `ui_loadout_briefing.png` | 199233 | `9aae6e2b278913fcdd2850da48c4ef2d07d861af3e8e54667b7e3c316837139f` |
| 56 | pressure | `ui_pressure.png` | 204509 | `66efa4ff1b72cc07ec99c0013cc2f0aa996b28dc41ac86632ae19511c41fe03a` |
| 57 | gameplay | `ui_gameplay.png` | 1186733 | `6fbc07ddbd6ced9e97f9ed030387b728be8e7c6a7b79604a184cf433050423f3` |
| 58 | first_hour_tactical | `ui_first_hour_tactical.png` | 1192527 | `6e8d5fea1c343863757c0569ea158cd4954b733b6cd3706bc7ddb489338e8000` |
| 59 | first_hour_attack | `ui_first_hour_attack.png` | 1111689 | `493db6b3be095f3a5ad498688166a31e225ed8d4f6425ef8d67e2f6d335e188a` |
| 60 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1201564 | `8793f2686068d222cabb57ee48ea12be739b94b46de93bd6bec81c471889747d` |
| 61 | first_hour_objective | `ui_first_hour_objective.png` | 1344002 | `8930ca26a12301656ef5c930975c0453cb23d9b327b331a574a2233d821980b6` |
| 62 | first_hour_replay | `ui_first_hour_replay.png` | 1102796 | `8d926f58c00a89443d24301c16b4b5171c27e92db29004acbb975fd3c876980a` |
| 63 | first_hour_ability | `ui_first_hour_ability.png` | 1191927 | `28879e151f7102915cefd57b2d9702ef608a681ff8906bb5f38f097ba51ff994` |
| 64 | second_operation_tactical | `ui_second_operation_tactical.png` | 1116581 | `5ee0308a14854bc7d10f3fdaeb85ecfa4862b8a1ad62a21ca03a0073e8401d47` |
| 65 | overwatch | `ui_overwatch.png` | 1189800 | `800ef3f41e6c74130a645c75e63f8ba5d75f1a1820bb0a0c43606b476877581b` |
| 66 | brood_ability | `ui_brood_ability.png` | 1245294 | `89c0b12e1fb5a75a0b0161068cee234e1f688ef82d5e08b9293d7dcd6088c616` |
| 67 | directorate_ability | `ui_directorate_ability.png` | 1245389 | `5810bcfdcb3b8025b8ed92f657652bb89365101f415c628701ec45b686cb8aba` |
| 68 | ascendant_ability | `ui_ascendant_ability.png` | 1245685 | `6b170ec077f7b04af7a23a75113ae796a3bbb4626757a52c3d76ef45c18c6da5` |
| 69 | hazard | `ui_hazard.png` | 1251830 | `f6b613e37230d1f7a468a288b72e9f491b62e4221a9f68c8ab28dff9165b7795` |
| 70 | intent | `ui_intent.png` | 1152858 | `8c3b10152b32a4bcf161b3963fdbc745c70017590a2e0649b48a502bc807d68c` |
| 71 | action_preview | `ui_action_preview.png` | 1114174 | `7ae2efa53c06b74a0bc527c4164d8ea4bdb6fa899acffc08c124eb6ceac1a605` |
| 72 | movement_route | `ui_movement_route.png` | 1148111 | `fc50f088494f1fc91d0768c80f77abfb6c16b6c094ea51a84be8914c2ddd4e41` |
| 73 | cover_edges | `ui_cover_edges.png` | 1187778 | `b9844d420d794d061f16d27b81fa5fb880b14953cfb0bc9c46d2fdd251786a41` |
| 74 | invalid_command | `ui_invalid_command.png` | 1218865 | `72d9bf98b015d3839230597306b6f3844d77c46c06060745b4cba10f61811e52` |
| 75 | valid_shot | `ui_valid_shot.png` | 1109588 | `26ed2afae0361001189fb35bf551fcaec2fb9bb377aa564a519b67735a95594d` |
| 76 | threat_range | `ui_threat_range.png` | 1221843 | `609b0855ecbbf90ece83eda17ab146eb197cf3f86405677683d8fb2a607ab9a8` |
| 77 | danger_reach | `ui_danger_reach.png` | 1218093 | `2c6ebe077a558386078556b8bec9a089a31dfe690f243f519895bf41c87dad02` |
| 78 | help | `ui_help.png` | 359966 | `6da7acf520b16132c8a9a56800079bcfef92092c41e4da0332a39340c81f0c79` |
| 79 | first_hour_guide | `ui_first_hour_guide.png` | 215026 | `7683632999a50317effd606f7a1741790e7deb517bf0c7acffde4d4f5789e902` |
| 80 | first_hour_return | `ui_first_hour_return.png` | 878729 | `7298f2b6c3530423e31288fb6846397ea23070efa7a722d5938e246236727fd1` |
| 81 | first_hour_promise | `ui_first_hour_promise.png` | 881379 | `4b64d7d086f919d7b25a1c0dadfbcb5cb60b67329b3982b6211827ce61dac3fb` |
| 82 | first_hour_operations | `ui_first_hour_operations.png` | 921819 | `e47297247e7739e6bdf5601fe3ab231d6a46e1b9bc01f80ce5a8fa22ebd440e0` |
| 83 | battle_log | `ui_battle_log.png` | 662141 | `2f8bb16d3dfef3f812cc8e083905a9cce4153842d0c54fd76bc0ec6d9500cccc` |
| 84 | combat_feedback | `ui_combat_feedback.png` | 1187209 | `332ba438fbe0bd9d5d769d948768927182a3c50b96f8f5d206010ea70631b65d` |
| 85 | phase_replay | `ui_phase_replay.png` | 1096891 | `aa1768f7c83b5dc23a4ef7c737a6b9efdfb743a9856c5b4dafdbcdcd089bbaf2` |
| 86 | end_phase_guard | `ui_end_phase_guard.png` | 1186911 | `90840bc360cfad2f8f0789ed0b940e04a2cba4bedd8295dbc8dac18f59a6851c` |
| 87 | readiness_markers | `ui_readiness_markers.png` | 1190972 | `dab88f6c0f9c6fed396b27dc17e0d18259884fcdedfceb177425c4f75ed2aba9` |
| 88 | vitality_markers | `ui_vitality_markers.png` | 1191396 | `3bdbd0d07b97589959c80186c04e8e991884e6373b8b37c612ea517f870c6ed3` |
| 89 | extraction | `ui_extraction.png` | 1440313 | `8df7b4b51a0f0ff64f6e3eb1ba5d2d9139dc10a4f8d2a8f400c280cc8732e895` |
| 90 | variant | `ui_variant.png` | 1239058 | `6bd8491ecd0aebf029764f106fca14a91abfd9770f4cd353c21c9baa0c24a77f` |
| 91 | sporefield | `ui_sporefield.png` | 1191852 | `5dbbdb74d95781e0973c884bab43d90ed496220c6f04fa85e878b13e4cdfa02f` |
| 92 | vault | `ui_vault.png` | 1187315 | `4d8306b3f71cbf1ff5545678f75772e2b8b4da42395d04c608b55ba85703f3a0` |
| 93 | black_channel | `ui_black_channel.png` | 1195552 | `1a3d43350cd47ea64404cceefffb5dfedcc1358a5c1a9b144a889f527c1ed04a` |
| 94 | living_chorus | `ui_living_chorus.png` | 1200124 | `042bccba8c92582d68fc4f44bf221c46e6a505c8391e454e1aaae87ae8e18b7a` |
| 95 | open_circuit | `ui_open_circuit.png` | 1198961 | `e2cb103590642b87bec0dbd922e378b9dd567640e322fd3317acc62694e093b0` |
| 96 | trace_active | `ui_trace_active.png` | 1162807 | `d994dc76cd01729a50c6979711acb62e6ff08190beba29799dbee8ff557b83e7` |
| 97 | equipment | `ui_equipment.png` | 1022099 | `5bdadd5e40cf42b4c5316db53cc393d3842666f1d848cb9e72955b61461e6fa8` |
| 98 | weapon_profile | `ui_weapon_profile.png` | 1186834 | `061abfc585fc57e6b145005cded11ad88cccb1a8143fa2554a8f6f1a2c9ae6ac` |
| 99 | class_target | `ui_class_target.png` | 1039275 | `a95a4597ae527bf724759f59729fbe90d6fe60dd3282999225c367fd18e28b17` |
| 100 | breach | `ui_breach.png` | 1363057 | `510c402a194f7ccd27c0be4824a3f952c94bf730ddfe9a340322d648a2d49113` |
| 101 | debrief | `ui_debrief.png` | 218275 | `c95526afa213d39b9505d45cad84cb774a826908cdb90e0cc9a9e13da10007f4` |
| 102 | trauma_debrief | `ui_trauma_debrief.png` | 201683 | `604954c2d0e8d328bd41c55132acbc81a6d0ca39a8f3755c40d686fa992254bd` |
