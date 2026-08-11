# Mirexis Final Capture Inventory

Generated: 2026-08-12  
Capture root: `docs/verification`  
Manifest: `scripts/capture_ui.ps1`

## Integrity summary

- 85 manifest scenes and 85 canonical root PNG files; no scene is stored in a capture subfolder.
- Every capture uses the standard 1280 x 720 logical target.
- The harness overwrites each canonical `ui_<scene>.png` file in place.
- Tactical inspection covered default gameplay, movement routes, class targeting, hazards, and the off-center 78% zoom variant.
- Colony inspection covered the default settlement, construction placement, damage, and power loss.
- `capture_audit.json` reports no missing, wrong-size, duplicate, magenta-diagnostic, blank, or low-detail frames.
- The acceptance-document test fails if the manifest, screen matrix, and this inventory drift apart.

## Spatial acceptance evidence

- `ui_gameplay.png`: default 100% tactical camera; 40 x 40 world visibly continues beyond the cropped viewport.
- `ui_variant.png`: off-center 78% tactical camera with selection, objective, hazard, unit, and contextual UI overlays intact.
- `ui_movement_route.png`: transformed tile route and action preview remain aligned.
- `ui_class_target.png`: transformed unit targeting and contextual card remain aligned.
- `ui_colony.png`: default 100% colony camera; 20 x 20 build grid extends far beyond the initial settlement.
- `ui_construction.png`: placement blueprint and reserved resources remain visible at fixed building scale.
- `ui_damage.png` and `ui_power.png`: building state markers remain aligned after the spatial rework.

## Files

| # | Scene | PNG | Bytes | SHA-256 |
|---:|---|---|---:|---|
| 1 | title | `ui_title.png` | 473799 | `4f0c9301efa7fac64babab26252a1d83617d73a2a523a91f959f76e3cb40aa3d` |
| 2 | title_controller | `ui_title_controller.png` | 476379 | `27bbfa8d56d8898970f65e82625691ed1fff328985af6d454f32dde6f5ff10e6` |
| 3 | title_hover | `ui_title_hover.png` | 476367 | `d4aaef506d3bfdc8faff49a8a142fe34f36f38567df07b95a835f9b31115f03a` |
| 4 | colony | `ui_colony.png` | 299761 | `7a425ca1c1b771e97e5520c27977a1f3b4549303e0d87128200d09c24a4e24bb` |
| 5 | contact | `ui_contact.png` | 315658 | `e9e6fe0ff535bb1a529d08cad4efb31967916faf62da470a41cef2aed9cc24b4` |
| 6 | contact_gear | `ui_contact_gear.png` | 241214 | `a88997133ffe8a9e04bb83fefeeaca68d8fb02b31636c8add5e4cb8a13e7ef08` |
| 7 | contact_event | `ui_contact_event.png` | 301814 | `12da1c294d9cfee9f8c4dc1e6bfcdf109cd66b1f9082b50ba3e216aab1f08b53` |
| 8 | adaptation | `ui_adaptation.png` | 277869 | `5dbe63fc06e0ab5ea5fd316b940eb461efbbb3dc1b90503467734eb077e7b4f3` |
| 9 | gene_lab | `ui_gene_lab.png` | 201082 | `373ca5c4e3387f8244f1f2939c5444798cebc97c474b6ff51eaad34c8c134903` |
| 10 | evolution | `ui_evolution.png` | 239902 | `f50dfdd66580939cd4874c3bfe8b2c7f715a1dbcc6bff43fa852347725e311ab` |
| 11 | mara_evolution | `ui_mara_evolution.png` | 202048 | `576f8cf4a5a538cc4b8ddb4a960b1932789f1669cf141d852cb7751ff86b98d7` |
| 12 | ilya_evolution | `ui_ilya_evolution.png` | 200309 | `69bcf21cba1b163449186a8c74ab55a1886517cc6ec0770745c9f0db2563e6a4` |
| 13 | sol_evolution | `ui_sol_evolution.png` | 194701 | `76b929c092db53b7b5bfe636ba700b54e66e3f2b586b86ff37e014fab8095d5f` |
| 14 | nadi_evolution | `ui_nadi_evolution.png` | 206690 | `c06bc07ed7b8f5b6dc0b3c96ffae767431eb2467f3dd2a4da22b53a30b240f1e` |
| 15 | escalation | `ui_escalation.png` | 285750 | `957585b5e745d531ac9fe04064cf1b551ef236cd9598ec1594faeaab0d846d63` |
| 16 | escalation_operation | `ui_escalation_operation.png` | 205105 | `6b23b277db8944b589f2b3bac53e45f880a55d03a8a6807408d96cc8204f89f5` |
| 17 | escalation_response | `ui_escalation_response.png` | 326018 | `f4aadef347c29adf66cea9f7f7bd5d5b487a22c4f8c6157e59d0bc852a88d4f5` |
| 18 | mirexis | `ui_mirexis.png` | 327788 | `ce63941465aa8cf2c227f5587ddb04877f219384ef48280c019fe64814788274` |
| 19 | mirexis_path | `ui_mirexis_path.png` | 286506 | `88900e7ba37014a5f057fc01b29188b9ea4e2450a29d214a7096924c82d94448` |
| 20 | redoubt_end | `ui_redoubt_end.png` | 314174 | `e0a59798389bc5e920caa052f61e3c1a59222a4fe2382ebd826f2696d3e2b7a9` |
| 21 | commonwealth_end | `ui_commonwealth_end.png` | 325839 | `a81dcf3dc3db0c034877a5c459eecf6fab73667d5cef114c6696c3355939cca3` |
| 22 | threshold_end | `ui_threshold_end.png` | 315154 | `20b17216379bffdd323cc42f31af5e46ea8a6e93d9e1aef4201819ebc8e99888` |
| 23 | finale_debrief | `ui_finale_debrief.png` | 192381 | `bc879015026c05b03d155a6c808d8f60f1a39967b4f1b23e8e3869f196b96f3c` |
| 24 | adaptation_operation | `ui_adaptation_operation.png` | 189420 | `eb703d74ce6e198195a6d93940f294996944cdf1c7b009fb45a176b262ef8a14` |
| 25 | glass_nerve | `ui_glass_nerve.png` | 1443331 | `a85704d56a3796b6c4a78b5ad8f2200d3d499263649eab335cef317bf7a305f6` |
| 26 | three_knives | `ui_three_knives.png` | 1432866 | `04bbb24a25b30efa2d4361a61466906d92f9df25620151b14f3c6ff7798c202e` |
| 27 | reinforcement_warning | `ui_reinforcement_warning.png` | 1433421 | `a1073f1bf9f83d67cb8db5da881a756d432ea389335d290abae69253ee1b9a15` |
| 28 | line_formation | `ui_line_formation.png` | 1448316 | `0e4f5c3d447c804bd7195a83a671913a34779ef7f922a12cbb4e44bbbccfb19c` |
| 29 | thin_shelter | `ui_thin_shelter.png` | 1449405 | `45238c24ac7f02dc1feecdf89c991e08082a9754cda95a803676930843c528c3` |
| 30 | breakwater | `ui_breakwater.png` | 1418533 | `eb1ada2d85aa944390339f9a30cb08af235a3775eebf4964ba44d41dfce6d2b1` |
| 31 | false_heart | `ui_false_heart.png` | 1451908 | `5a64bb3c9673395c286d94b242f130c74d0a12643edeb87d39903b0d36a4eafb` |
| 32 | live_wire | `ui_live_wire.png` | 1451026 | `2949c85e08fc92abf058b8a0d32c6860be8da2b3dfb25bd0e6cff9b4bc641119` |
| 33 | last_wall | `ui_last_wall.png` | 1418758 | `cacf7749fb93393174d148c3ca5ddec9673b3b29cd56aaa20b62ffa79d1ae277` |
| 34 | root_choir | `ui_root_choir.png` | 1455291 | `e64e330310ce46f0f1c0bb0251427847fe027d280a8dfc112015d4a74afb8f4b` |
| 35 | door_of_light | `ui_door_of_light.png` | 1442668 | `52290dd46731d0e030cd82d35ff9aeac08549abf783eb894e9d8c98ddd679168` |
| 36 | damage | `ui_damage.png` | 304617 | `e028c1ea45ec3f78e85e4b7a0ff1b6de730479a65fcb0910ee30aab792f34e05` |
| 37 | repair | `ui_repair.png` | 311710 | `e6b0470687924b8f3214629cfdae8190b5758a68a4938f606a372fbb3dee24bd` |
| 38 | power | `ui_power.png` | 300729 | `c9f6c7bbd9f2594314d694109d668789634e36bf32e39d961b2f8c95ec1a4786` |
| 39 | construction | `ui_construction.png` | 307413 | `cc7292224c9484b9fa57186bc7e9f726e0c5041701efbbd90a860fa84fb9fc93` |
| 40 | research | `ui_research.png` | 293557 | `13f853d5e6048ab247fdef7be0d3712bc3d4558934c4f1d818961e7589a390eb` |
| 41 | roster | `ui_roster.png` | 247549 | `41d2c7754a5feadee3a554c65901b09d8b203df9cf2b0c99342fa04bd3ee2400` |
| 42 | advanced_roster | `ui_advanced_roster.png` | 246379 | `8d0c9c6a3819b996344e784d4a0e42481fa88ece78da926d5ebad671de92e279` |
| 43 | relationships | `ui_relationships.png` | 251144 | `6843153e316a5cfb556f214398eb15926800cf4ac5705b7ef42a152b881b3fb8` |
| 44 | trauma | `ui_trauma.png` | 251449 | `4a6409936123e788329841122df2ba32d717170a0d001ef4eda848219b26b33a` |
| 45 | bonded_briefing | `ui_bonded_briefing.png` | 206924 | `b5c2d03539a693d8795f3f3e0a81bc42e0e87d3c4714ea7155da46fc8e45d978` |
| 46 | legacy | `ui_legacy.png` | 249768 | `abc499452866ed2f5642d2289227e8ecb94bf2bfd213d2c843119f833663d633` |
| 47 | briefing | `ui_briefing.png` | 205141 | `cfa798aaec3544ebc1b45379e1d96d9b0b4c676531aa559371c9f5871c26d6f3` |
| 48 | threat_briefing | `ui_threat_briefing.png` | 199531 | `d87634a751f8e6bba07e427c41d394ecf2cba616b00cab12b555d96221550efd` |
| 49 | loadout_briefing | `ui_loadout_briefing.png` | 205848 | `240c5ead983a1cc6eb80bdd9e9bd6c7b9c2f513d30aacc8174030231b6b8261e` |
| 50 | pressure | `ui_pressure.png` | 214142 | `f570776cab99fcb57a0e3941161ca4746e56820e3f8f991bfcd89be0464bcc81` |
| 51 | gameplay | `ui_gameplay.png` | 1448594 | `237d700b267f5c2c220ad67d2e4c312a2e383b4585969025fe4bc59f263d800b` |
| 52 | overwatch | `ui_overwatch.png` | 1450017 | `f86f4fa54f180667035978498f3ae5be2d9713a713755a7a30caa5ca6d31cd5f` |
| 53 | brood_ability | `ui_brood_ability.png` | 1445371 | `7e7e36b276c015adbc4ac73dd7a517de837bf4ec99914ae6f4c7586b4adb68e0` |
| 54 | directorate_ability | `ui_directorate_ability.png` | 1421936 | `c547ec1edd7f00d473fbf9739144b2ea83a3652031d504d59ebfd529b73dcb0e` |
| 55 | ascendant_ability | `ui_ascendant_ability.png` | 1441626 | `dc50cffefe97ed4e2205e75f157804596d384487df83f6332bfdfe4825a24f77` |
| 56 | hazard | `ui_hazard.png` | 1451403 | `87156b1e38088422bb6c4b648896cbc904b56e59483fad76b234439ec55e68ee` |
| 57 | intent | `ui_intent.png` | 1413693 | `8c5ee55bf76d736035a069385dc29042dbd0d47f4a84f1c51c880c8642ff9dce` |
| 58 | action_preview | `ui_action_preview.png` | 1312043 | `b7302223d46611762f8e5a06b03299b6b831cd8d51e524d47ecfb415e32059c6` |
| 59 | movement_route | `ui_movement_route.png` | 1407483 | `283482de68e3ebf3e5e66c5cb4f1a11827d9c2459e057d5b38730ef4ccafcdae` |
| 60 | cover_edges | `ui_cover_edges.png` | 1448650 | `104addfd5a42803e71950169e9314afe29ee3f91b63ec624bae40c46608285a6` |
| 61 | invalid_command | `ui_invalid_command.png` | 1417605 | `243363b8cab9ed84cc77777aa0c6c10888d4a8a51f62bdc49295b5b316523029` |
| 62 | valid_shot | `ui_valid_shot.png` | 1317914 | `f05614b627fcdbc18944272185e46acf697674ebd78e4ac1440fc060ee7123a6` |
| 63 | threat_range | `ui_threat_range.png` | 1320808 | `845afd93e285bbac14e21886bcec9a17a0513479a711f3fed3b46c2d88c8e113` |
| 64 | danger_reach | `ui_danger_reach.png` | 1318730 | `91001a872b02e109280b9979b9cc9b7bdcf53584308fe759c47800fdd9ae267e` |
| 65 | help | `ui_help.png` | 420481 | `7c01f648e4563a18c088d87348dad6676de093c5911edbb0c0c2fc74852725fe` |
| 66 | battle_log | `ui_battle_log.png` | 895534 | `a7aec09983bcd2afb46a9c3d54993773b158b4d0826796d10f5fd40ecb35bbb0` |
| 67 | combat_feedback | `ui_combat_feedback.png` | 1455016 | `7292a12cb3471e0f19dbd42edec1b308624c32ac9b3ec760418f077be3b8a663` |
| 68 | phase_replay | `ui_phase_replay.png` | 1366499 | `fbcd614ed4ebb688a811445b9a45e10dbdcb179e5005cd1050d6c81e27c4062f` |
| 69 | end_phase_guard | `ui_end_phase_guard.png` | 1448770 | `bd7e4ab61250ac46ea49476d0b4f2cd995fa118dd6771dc15fb348c64defdb84` |
| 70 | readiness_markers | `ui_readiness_markers.png` | 1448582 | `752c4cec5e0295e789f5709545817a1004088df23caac43154c7805d73e2b7d1` |
| 71 | vitality_markers | `ui_vitality_markers.png` | 1448483 | `25209e427e26a6962fb3542017a94c66e906f6a4f60ada09e9c2ec7a03e2913d` |
| 72 | extraction | `ui_extraction.png` | 1418071 | `f5b2dc122ce15e3b66dfef871d9ecf2a4a5332d4ffda332de34fa2fc97384f4d` |
| 73 | variant | `ui_variant.png` | 1431614 | `17cb8944938efdc5804ff1b316d698ac2aa8c766328158f3d39320681a96dd47` |
| 74 | sporefield | `ui_sporefield.png` | 1449583 | `ab6989143859fbf760e9cdc473df8e601d829c08a8f8541263f9d0d77fc48a83` |
| 75 | vault | `ui_vault.png` | 1440006 | `8695dc7ff8934434d68a6bb23b772b7658e9ae12c1cb4db38319e46594db8068` |
| 76 | black_channel | `ui_black_channel.png` | 1430806 | `b705caa2a356b195d7094861df58044f6054a9963b002bcc836b1328f93fc5b2` |
| 77 | living_chorus | `ui_living_chorus.png` | 1458397 | `b0a21c63ea76dcb89a5341290605d4770196ffe56544c183d860711d523ad530` |
| 78 | open_circuit | `ui_open_circuit.png` | 1453985 | `ea9b1f6379f4993d4a4c0d35f4215de28861ad94e73c11564c81e18862b81b08` |
| 79 | trace_active | `ui_trace_active.png` | 1393202 | `9802a1fdf939bac66a9cc5b51f43831893882f7507df694b9438c8e52b27e551` |
| 80 | equipment | `ui_equipment.png` | 1270896 | `b9639722617e360e0ece607171e460c19da2997db8779ad6231ecfc14e03f277` |
| 81 | weapon_profile | `ui_weapon_profile.png` | 1448665 | `b34f3130bdf95a8160c1a7ac6b77587beb3a725405632eb20fde87d0edcf0154` |
| 82 | class_target | `ui_class_target.png` | 1310870 | `e2947a90a0aeb7e8b787561e042b56ec4eaa06e522a85b782cb123da44b199f9` |
| 83 | breach | `ui_breach.png` | 1449371 | `03afca022aff60cf3689684384d12e3d2f048e6f7391783445b23e22728d2493` |
| 84 | debrief | `ui_debrief.png` | 195735 | `4990a8de76dd2694877dc7f0d9172e37f9425361ce161e710240f33ed9762015` |
| 85 | trauma_debrief | `ui_trauma_debrief.png` | 202757 | `741df737bd0bc2c157e5e2a1398d3198a8220400f5a3ef7d23f7a62bcef45510` |
