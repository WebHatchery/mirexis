# Mirexis Final Capture Inventory

Generated: 2026-09-12
Capture root: `docs/verification`
Manifest: `scripts/capture_ui.ps1`

## Integrity summary

- 109 manifest scenes and 109 canonical root PNG files; no scene is stored in a capture subfolder.
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
- `ui_roster.png`, `ui_recruited_roster.png`, `ui_roster_info.png`, and `ui_advanced_roster.png`: the complete workshop equipment catalog stays inside the training panel and above the footer instruction line, with a persistent touch inspection reference.
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
| 4 | colony | `ui_colony.png` | 873196 | `37f72c1a069e8059a944044d526bfd1151596c9fc1707773655715463bc2728f` |
| 5 | facility_upgrades | `ui_facility_upgrades.png` | 698299 | `b57471e73846e33c272bb9c6a7738b91cf73f0ddd6c17a6aebf7b45c2f4e54fd` |
| 6 | settings | `ui_settings.png` | 232785 | `da6dc23c2e5831289923ef43322f9e35df210f21ffd96fdf31faeaacf858b630` |
| 7 | field_notes | `ui_field_notes.png` | 125543 | `0850c6237bee22dc6d0b4cb60cb74d98161c8b71cd21f6ebc10e72a998f198ba` |
| 8 | memorial | `ui_memorial.png` | 169521 | `5fdfdf4fc9b6eb29d7a0b8f19ba632534a028077cc32fbfe30539b57ea1de304` |
| 9 | memorial_page_two | `ui_memorial_page_two.png` | 122914 | `d538a665746c0cb885200c91cd83bc5ce14ec9e7d33062a4c940780ac6ce8208` |
| 10 | first_hour_dialogue | `ui_first_hour_dialogue.png` | 730402 | `cd3c7f49af510eadaedf906c0a7500e107c51dd2f57120e0173b1328c2d8cd9f` |
| 11 | contact | `ui_contact.png` | 931909 | `2df7a95e887a8db6b700d87a1c28f81c81ebec00ae9bf0946cda076784735a88` |
| 12 | contact_gear | `ui_contact_gear.png` | 259245 | `ce87c709e2d073d5448eada90eac1c227cafe2b2906e5af36b6c2aebf36e8dd6` |
| 13 | contact_event | `ui_contact_event.png` | 918487 | `83634461907fb3b9edd7c3e9ded27b140a14d24228ffb59abcfbc5aa4a9ac73e` |
| 14 | ninth_recruitment | `ui_ninth_recruitment.png` | 898543 | `67a81b7bc90fc6d5aea2ee9ea6f55bf2262224caca350ffdc8034d43e72f0528` |
| 15 | adaptation | `ui_adaptation.png` | 893820 | `ce759ce87ea6cdea68c7965a2d6b07ffc5062a768e28d985ae0585b8b0ec169f` |
| 16 | gene_lab | `ui_gene_lab.png` | 182294 | `b94511d1faf0221cc7ade953d8c540d334b8727cf93ed2fffef487e5620c97a8` |
| 17 | evolution | `ui_evolution.png` | 254781 | `25253cc4a005ec8907055e0f9a39317e95f97a0300a00492fbac1cded419e96f` |
| 18 | mara_evolution | `ui_mara_evolution.png` | 180575 | `8ebc2be07af7aacf84ac103907b39e61f21128e68df48a44645373fca7027c4b` |
| 19 | ilya_evolution | `ui_ilya_evolution.png` | 180388 | `ceccb6545ac9d1e119176f35f7217b8272ab58093ded156eb679af6186dae421` |
| 20 | sol_evolution | `ui_sol_evolution.png` | 175088 | `d1dd3440570c56e2281d4474ab222694f0c3f68aa1f35a649551b185126aa90b` |
| 21 | nadi_evolution | `ui_nadi_evolution.png` | 185705 | `344a9ce195d3da9ceb1423fe495b7270c01e29107a15ce09286c5256daa9cc28` |
| 22 | escalation | `ui_escalation.png` | 897060 | `541d560c44afac1d108673fa1c15892abcb52366a726309f53c217a67328eb5b` |
| 23 | escalation_operation | `ui_escalation_operation.png` | 205014 | `49300032c2aafc4e118f48eda185611805d003f318312b8449e7f97d11d3e97e` |
| 24 | escalation_response | `ui_escalation_response.png` | 938949 | `8437c9da3502bc14a0527dc34e260f662e71a79ac770feabd97c562bb1c68c85` |
| 25 | mirexis | `ui_mirexis.png` | 939436 | `8adaee2e5e8357adf179bf779ef0f74c2b065fb950d916d03a00b9382d66fa70` |
| 26 | mirexis_path | `ui_mirexis_path.png` | 899515 | `7df01744b94d7ee5398f36b78527a5dde4d33abe564c2e478f085c945eb63636` |
| 27 | redoubt_end | `ui_redoubt_end.png` | 920478 | `806881157816ca0ba717c2a286122bd4d7005deeec4d4cbfb854fd62d553e8c9` |
| 28 | commonwealth_end | `ui_commonwealth_end.png` | 928988 | `579c472718b4a0f7160cdcae77395bd156e783c29d264348932fb631f2927cdd` |
| 29 | threshold_end | `ui_threshold_end.png` | 928738 | `e29fa36b8e9c20368bc562969ca0b5907e6bf4693d6b59f354f6237ce4c05d3d` |
| 30 | finale_debrief | `ui_finale_debrief.png` | 197645 | `b402a19906425a1f4b7c7afa28a02e8fd5fe46f4cd03c8a6cd085332e4306f6c` |
| 31 | adaptation_operation | `ui_adaptation_operation.png` | 190582 | `afe665897e9bcd8de3f48d978b5c3d00a586aab6586170831a28f1738d874638` |
| 32 | glass_nerve | `ui_glass_nerve.png` | 1137296 | `c9a09087d011667190fdc3ab6482b3bd138aa93ab667fb73e54339d1b7137418` |
| 33 | three_knives | `ui_three_knives.png` | 1144233 | `9f839a542d00c7eb19d471739ef7fe5b9102d4f9dfce3355d9862b962f75728c` |
| 34 | reinforcement_warning | `ui_reinforcement_warning.png` | 1144515 | `710b74aa7f1bbfe0f79b91ea83ccaf1e216644557fcfd22d113d1e374e1d4528` |
| 35 | line_formation | `ui_line_formation.png` | 1136271 | `f41333fa580b180bdeabf8b35004ada6acc92d860b91eee40b0d6d780330a5c3` |
| 36 | thin_shelter | `ui_thin_shelter.png` | 1138355 | `ce45bbdadac9ae1d34cdc1797d2593411d29a5d0312506063b1b06bd6be10113` |
| 37 | breakwater | `ui_breakwater.png` | 1133302 | `a7432882b95a3fa2b3671cb77ba43a8bea0fc3a6dfdc66f6a47ad1bbe64da068` |
| 38 | false_heart | `ui_false_heart.png` | 1137938 | `cbd7d4c3d25b221516ce0219e102b34a26328afe3589f7832e3d566fdbd44fcd` |
| 39 | live_wire | `ui_live_wire.png` | 1143371 | `a69239ad3fb06acf0bd33570ae0ffaea77ba09b80eadca0b9401b52ad7ac976a` |
| 40 | last_wall | `ui_last_wall.png` | 1136995 | `d5b1be9fcfbe477d659a899d9fb0650fe2a7cf3150eafdb6810b8d539a29f856` |
| 41 | root_choir | `ui_root_choir.png` | 1142110 | `b4d62aaf4c209ecd237d9d3e5dec2da7e251e1c725f7c692a43eec5d1d662601` |
| 42 | door_of_light | `ui_door_of_light.png` | 1135476 | `7fc2907f4047171bf694d1ebdd2ce0eb74e97e82ee1522137debf61fd60a86c8` |
| 43 | damage | `ui_damage.png` | 877793 | `816a47c5e98f8fe3979fbcee9e07549a88a76da3f105212d76fd2f74e7dd66f0` |
| 44 | repair | `ui_repair.png` | 884428 | `465e9b35e06b1651d476f62b29fc47a570a5a9c55cbfc192dee60210c8c18d0a` |
| 45 | power | `ui_power.png` | 876142 | `0f83f0050c6981d5e2561e87b0fcc470fe1956d7b563f93fa9eb4ce1cda032d6` |
| 46 | construction | `ui_construction.png` | 877944 | `8ad63df4f84650e63d27515c4b9fb6ab8bfe9c861f8a4b9e59c7ad16dde8ae46` |
| 47 | research | `ui_research.png` | 910988 | `9b72809c2c921deb5c17f75ed745af03363ccc21c6cdafb6884a8ae8befc57b2` |
| 48 | roster | `ui_roster.png` | 258261 | `c6255ea9c5b126e9f73e3f49835e398c2f95ef0f4ebc7df0b0870b83c39c59bc` |
| 49 | recruited_roster | `ui_recruited_roster.png` | 291193 | `7d853154ac750ec83dc617c7e24f688986888be5c061ad3f2b8c5f6938f34c75` |
| 50 | ninth_roster | `ui_ninth_roster.png` | 289433 | `6611b4dd385169378487a112a94db1747fcd3e4a33ae6b628552310bfedf06db` |
| 51 | roster_info | `ui_roster_info.png` | 300599 | `07bea2892a3475d338c40653094bbc25146e1eb512608c58b0cf58d75381a6b1` |
| 52 | recruited_gene_lab | `ui_recruited_gene_lab.png` | 210723 | `c352cda78b5478ae793797d30a8b41579ec89fc260c1089a736aca268d3224c9` |
| 53 | advanced_roster | `ui_advanced_roster.png` | 258091 | `6b75cafaba2a7bd6a46c0b34e91608dd71a920b3eb1236a6a479411f9677aa67` |
| 54 | relationships | `ui_relationships.png` | 260250 | `b9fa30f2758353adb5172275e5f2612c13f4663eb3114a2b07491dcb4e40e9e3` |
| 55 | trauma | `ui_trauma.png` | 262278 | `2532912da35f73e3ad5af19e086b27ecf882dd3c58f09dce14e5febc2945f829` |
| 56 | bonded_briefing | `ui_bonded_briefing.png` | 200378 | `f1abc5df3d005bb1950cfd76fba39a24b91f4ce4538120c56c7fcef0f4f7013c` |
| 57 | legacy | `ui_legacy.png` | 258908 | `491c00f4b859e674f81ef558f6685a6e68873394b055546d97d15bcfb2db6936` |
| 58 | briefing | `ui_briefing.png` | 209603 | `ecf1de2809f66187d75d4f3de60f06e85fa78cde72eaab91450e3530b10863fc` |
| 59 | recruited_briefing | `ui_recruited_briefing.png` | 212052 | `f09ab94ceb3f5b79b88c4b4cf0c035b1e20847a03f9d287b840923c2af20cd81` |
| 60 | threat_briefing | `ui_threat_briefing.png` | 190457 | `223af447ffd91d17462ed76c0a6e23b9d574fdb417b1baf49deeae387080c801` |
| 61 | loadout_briefing | `ui_loadout_briefing.png` | 199364 | `a038308ebff2d78d3d8de0f6c0902a56dbbb7c7f658c34d793adfd490072f24a` |
| 62 | pressure | `ui_pressure.png` | 204641 | `19a23c1b89567a6365f2f206a53b377b29352ae0ceb8ccfb847bccc6843f6280` |
| 63 | gameplay | `ui_gameplay.png` | 1612521 | `6171fcae25ed7ea6d2867b6505065ef16d7fc3fdb323920fc419bfb3c9eee310` |
| 64 | first_hour_tactical | `ui_first_hour_tactical.png` | 1620889 | `faed50f0dd28379a145037d7ce192abcff8303f1c5dc0208737ca1d11224fdd5` |
| 65 | first_hour_attack | `ui_first_hour_attack.png` | 1585191 | `01c51405accc3abd2026459800681c836e85599348ce41340ce42c91f1395212` |
| 66 | first_hour_enemy_phase | `ui_first_hour_enemy_phase.png` | 1629104 | `a7f296a3baecd788e253733faeab11d714565d7dec122834909ceb714e37df88` |
| 67 | first_hour_objective | `ui_first_hour_objective.png` | 1563229 | `885551f99bba866bc16eda43bcc8acad260d1db5406d04f638f3867787172124` |
| 68 | first_hour_replay | `ui_first_hour_replay.png` | 1541313 | `0983d17d25f56a7611d6d35182aa72c1eef4a54b1d67ca9b0ec756b7222a9273` |
| 69 | first_hour_ability | `ui_first_hour_ability.png` | 1148319 | `b4714dd2675f838a38732cdc67a0db250791359167f638b507d3a1a088254ecf` |
| 70 | second_operation_tactical | `ui_second_operation_tactical.png` | 1590430 | `50e3c56d036d35296c0b4900cee7163b6810dd31b31a0991365a6cda2398432a` |
| 71 | overwatch | `ui_overwatch.png` | 1140196 | `3aa9d763463ff5b13bf8079e0e3fa060f3c38575b14683873970e033f619e782` |
| 72 | brood_ability | `ui_brood_ability.png` | 1144934 | `04bfcd71c7714aeaf9b6340b021c025438505606ad0c1cfe8162afe3079dc99b` |
| 73 | directorate_ability | `ui_directorate_ability.png` | 1151992 | `8052dd56a16523f48054f4d0689c18b6115f1731bc86ed4b62138d06f8fba715` |
| 74 | ascendant_ability | `ui_ascendant_ability.png` | 1148389 | `3b883ca86094ef1a1e866f6691e1c8465e5d3666727ab7a98e87f36849b13e0d` |
| 75 | hazard | `ui_hazard.png` | 1151020 | `10df1fadc7fb10fa8982aa0fb3f005824bf1c8f0090fb467cd1623f2d1825055` |
| 76 | intent | `ui_intent.png` | 1742153 | `8c9093df8bbe4db13ac636b52ee5827c18f9083516f198c4cc1c682277a56589` |
| 77 | action_preview | `ui_action_preview.png` | 1121435 | `1cd9d25f85c620540692c368155d4066ba520770fbcfdab1cca768659af4d487` |
| 78 | movement_route | `ui_movement_route.png` | 1602557 | `4e6bf4f72ba82c2e2fe8f7ba1748ee7def79f96ef8f83381069f476ce8c7e942` |
| 79 | cover_edges | `ui_cover_edges.png` | 1612764 | `cda94b0acfb6939d6ef8e13fca30c24221855c72a47541318d10c4f0951ad1ef` |
| 80 | invalid_command | `ui_invalid_command.png` | 1605717 | `c39f9b09513042b1f617a257f5baf388b2895be27ad55266f6ee957755bd4ba9` |
| 81 | valid_shot | `ui_valid_shot.png` | 1576253 | `abcb92a46455c3616145ee806bacd6a8f2659b216deacb0ce3dfdfb7067905c4` |
| 82 | threat_range | `ui_threat_range.png` | 1607776 | `323fc16916d9a735081ee8c6d7f5b6d6919b4b9b4d8a90733177e1c1de666b29` |
| 83 | danger_reach | `ui_danger_reach.png` | 1605089 | `725b7643fd1481e011b69586128fd569320204720daa10a63ab9ca2c2c02501b` |
| 84 | help | `ui_help.png` | 554050 | `b8db406495699f83d7b90cb0ae6b5bbbd12bb366002fb409dbdf9b656e286caa` |
| 85 | first_hour_guide | `ui_first_hour_guide.png` | 218704 | `7f0e4e2092402f8ff48cbc6e3264db58b1828c70804afc3a0e4619c9cffd55fd` |
| 86 | first_hour_return | `ui_first_hour_return.png` | 873238 | `ba8fed498697aefbd4ed81e498880fa49fd45aa24cdde51727983bd35985de4e` |
| 87 | first_hour_promise | `ui_first_hour_promise.png` | 875475 | `3bc6e5786a88ec703a9d5c0a0a2b6a78be415dcd23ac4a927b59741e8d2bf7bf` |
| 88 | first_hour_operations | `ui_first_hour_operations.png` | 919506 | `49390ae2c85017aa00d63531c74f9f6cbeeb750cf1ced2bfe1bed300eb721e92` |
| 89 | battle_log | `ui_battle_log.png` | 684486 | `3679a9b0327b09ccfbeac558bc072b20ab1f5230bfb1104fda4b91a39663539e` |
| 90 | combat_feedback | `ui_combat_feedback.png` | 1611814 | `3d1fcc9cddf4b4954a843d56ab1c188ba63ce65b7b6a08dcb2e9d6a228823044` |
| 91 | phase_replay | `ui_phase_replay.png` | 1529503 | `7d06f0d3788d3507b9b3002e9a802a8ef403d4b2078eb641e561b0bc740544f2` |
| 92 | end_phase_guard | `ui_end_phase_guard.png` | 1612524 | `4ca2612f2ff7bd0902d5d7d9aca9164c7519073e44d607aff387347893f55d7f` |
| 93 | readiness_markers | `ui_readiness_markers.png` | 1141886 | `2174865560f5e866d312e2f9dcbbe10093c616594aa640fd1461a8f3fe00924e` |
| 94 | vitality_markers | `ui_vitality_markers.png` | 1662531 | `116b7af6031b4dc9b292287105d6f730fd598bb5d384db3a11e37870a0784d0c` |
| 95 | extraction | `ui_extraction.png` | 1690238 | `1d27d0c6d8229b60141a5ad7ca49e5425fa0b7858cd95675975c7035e243985d` |
| 96 | variant | `ui_variant.png` | 1541030 | `c5dfbd63d9fccbbf305b1341fd84187a41417f3a389b35681a27b21861f73e42` |
| 97 | sporefield | `ui_sporefield.png` | 1143225 | `3c5ca3089dc5707add718a2f0856396c8bc2d04ce00bfd3d4f03807e545d7dd5` |
| 98 | vault | `ui_vault.png` | 1139272 | `bbc9687cf28b6ba46bfb0421402a298fbe13b2e0b065b47fc3383c5dc84daa83` |
| 99 | black_channel | `ui_black_channel.png` | 1148890 | `84f6d107679246e513669ea386ea8918c6d961d6b06739a17a2cbe6f887280f1` |
| 100 | living_chorus | `ui_living_chorus.png` | 1147770 | `6996cb1f43afb5184fa9e163a4cae687190f7759f8afa49cddb0e0229045deed` |
| 101 | open_circuit | `ui_open_circuit.png` | 1148480 | `4f17b7b69b02eb3f0e7f97ff95cc543972b1b0fdf32d1703f4adedf28c3b8521` |
| 102 | trace_active | `ui_trace_active.png` | 1148290 | `d4f9827173191953a4bd034ae9941669c4ed6829ae1fac29dc602e39be8f2544` |
| 103 | equipment | `ui_equipment.png` | 1028892 | `688f262fc290825156e1c4f45e15ff155b5102b4cb9ba6ccbc5228d9383ff666` |
| 104 | ninth_resonance | `ui_ninth_resonance.png` | 1036107 | `782569376f8d487731ac4d8a35e89230b9002892c1807688c7dbb9d672de76e7` |
| 105 | weapon_profile | `ui_weapon_profile.png` | 1136423 | `9f57e86065c6644e7c0693e98f21680d8f82cd2c059f5e3871fabd0912d44308` |
| 106 | class_target | `ui_class_target.png` | 1022258 | `2fcf45c33f41484ba2baed71bf807cc83ab0576afa033b7a4bc3ff93d6c0b291` |
| 107 | breach | `ui_breach.png` | 1889917 | `f5a17d56e1b7bcfd56fb09bceb9dd6dd52e1cffba15107b16a8b1adb7c4e8065` |
| 108 | debrief | `ui_debrief.png` | 218341 | `14fed24df655c66292009d0d7bd70f5d628f32490a41aafc3794998d06e22a7a` |
| 109 | trauma_debrief | `ui_trauma_debrief.png` | 201683 | `604954c2d0e8d328bd41c55132acbc81a6d0ca39a8f3755c40d686fa992254bd` |
