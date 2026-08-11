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
| 1 | title | `ui_title.png` | 478110 | `b644b0b3ec2d7f4e1491d7ee7d8b17eaabe826a4adfee79ad42f9ee26f35c66e` |
| 2 | title_controller | `ui_title_controller.png` | 480998 | `6ab179d703e9bf93d9c8a19f8c8e3e239f33c2b275ef5a052f9ffeabeeb45ced` |
| 3 | title_hover | `ui_title_hover.png` | 480945 | `d66cd1d879fae41613a25c248ac7dc0e2a05d83d87cb07d670769bb6af8f6da9` |
| 4 | colony | `ui_colony.png` | 375918 | `1f76ea5fd3a6f237b512724246f640c293fcb25a3b879d7a824adc6b273aa805` |
| 5 | contact | `ui_contact.png` | 393731 | `9aa343ff6b0bcce2c33e8ddac18b57791cba70fc36bc7eceeb50047f9cb518b7` |
| 6 | contact_gear | `ui_contact_gear.png` | 248722 | `990c8326bc96356c8f86f99ccc76a6268c9dbdf90be8531bb7388fa4b046e47d` |
| 7 | contact_event | `ui_contact_event.png` | 378684 | `33eddda376cb5415770ed5b4c51caeb74c4234a7d4a4f5d54edd5b350f059833` |
| 8 | adaptation | `ui_adaptation.png` | 354501 | `da4cfc0d69a2e63ea0879887aea6d7f40d3463a6cab9b3d24d3fe292137d378a` |
| 9 | gene_lab | `ui_gene_lab.png` | 202223 | `ceb9723401a6ede0350646876fc84b77e58955313c73c99f53b193fd70719ad7` |
| 10 | evolution | `ui_evolution.png` | 247490 | `5c4da7e697c08ae4e9d3ec65286ed4cceacbaf4b95d425023d9484cd43696479` |
| 11 | mara_evolution | `ui_mara_evolution.png` | 201923 | `d9065e16ca1e21521c8a2f76c2f40e23010624746141e1c7bed46e700a9f1f02` |
| 12 | ilya_evolution | `ui_ilya_evolution.png` | 200185 | `e2856d8f9725bf9ddc5cb8a76e3cfa3562db32b17cc62f8cca53390ff7af111d` |
| 13 | sol_evolution | `ui_sol_evolution.png` | 194576 | `86342e97f330470ef2b1d8a64e2eabc1304ecf85ada6a49d4d75ff0e204f5bd2` |
| 14 | nadi_evolution | `ui_nadi_evolution.png` | 206565 | `084de54f8d78be117e267da6290dbd76209fdebb37725b4b925c2d55b8a6fc5c` |
| 15 | escalation | `ui_escalation.png` | 363563 | `b4e7f1f603ca21b418d5910c0c080c61ec4120e4acda733bcfa68ebcf4d50e68` |
| 16 | escalation_operation | `ui_escalation_operation.png` | 217901 | `0f9d3d0d85a6e23845aa43b88881d6d70e70822a451ce877070fcaae164c073c` |
| 17 | escalation_response | `ui_escalation_response.png` | 404159 | `8767c363971dab410df7566c2db0ece0e55e208b941737fd73db934a3c1392db` |
| 18 | mirexis | `ui_mirexis.png` | 409797 | `41869028d76e2890e25fc6de503fe73c1920fd9deeaf384b22818d72d3b3201f` |
| 19 | mirexis_path | `ui_mirexis_path.png` | 364408 | `f9fa3afb2cd7aa2f8d42678430800a89e1de838d2e917352013014e90a20c82f` |
| 20 | redoubt_end | `ui_redoubt_end.png` | 404609 | `2cec8b1d70043501246d9519d51f7930371e9b02346422243fd30a8eec16193e` |
| 21 | commonwealth_end | `ui_commonwealth_end.png` | 417342 | `884449cdcf39d269620b4eb2038b1414f84323bbe7ea27d0a17922e40c5db892` |
| 22 | threshold_end | `ui_threshold_end.png` | 392650 | `3e38f9a3517054af834a8c818ec1b15d02a2e0ce7ba1a7168336b8b8f5bad275` |
| 23 | finale_debrief | `ui_finale_debrief.png` | 199479 | `4bb1c851e27ff5f650250b149a6b632eac74b00c6a3f07d9a18101d14c8bf593` |
| 24 | adaptation_operation | `ui_adaptation_operation.png` | 201132 | `b519520582403c87ccb2422bef54b623498fbbcc7b4135364039e2c3dde26e0e` |
| 25 | glass_nerve | `ui_glass_nerve.png` | 597309 | `abd2caac2883b187a248c8dfbd60b324234a303900cabe3368abe48b142abdb2` |
| 26 | three_knives | `ui_three_knives.png` | 599084 | `0b10b98a461e16e051e1331e8bc876bceaa58b307c4b18e665769785e4da924c` |
| 27 | reinforcement_warning | `ui_reinforcement_warning.png` | 599415 | `5ddeb4d3603d3a0ca631c6011eee268f06b727fe6a697c192abd842349aaca95` |
| 28 | line_formation | `ui_line_formation.png` | 600662 | `6dcba5911b38154cd937a5b6138bca14cd0a830a7124ab33325e322d9d7efc99` |
| 29 | thin_shelter | `ui_thin_shelter.png` | 602903 | `1c3e4f512a54b3e8ae110d98a02abff3b0530a8c3065361792d962525a44a09a` |
| 30 | breakwater | `ui_breakwater.png` | 592829 | `28f03141a5ef79783504de4f50c703bcd2b189df27c88a650ecf5d01bd112a96` |
| 31 | false_heart | `ui_false_heart.png` | 601040 | `87695cdb4a1034fe29df5cee7bd4a68cb2d7748c5bb60446700e4708cc9c0c01` |
| 32 | live_wire | `ui_live_wire.png` | 605204 | `4245d89af7e56655b8888a7a4307f8cf60eaffaf27ada21d931fc97e5d796285` |
| 33 | last_wall | `ui_last_wall.png` | 587763 | `2938ef940ffce3db45e2ddb2b8f0578729585c933a9232e6ae3397cb43fda507` |
| 34 | root_choir | `ui_root_choir.png` | 607408 | `26a00bb195bb396f13c4ac6aaf014b2258578cf032ead9ad3681bde6db0ecc78` |
| 35 | door_of_light | `ui_door_of_light.png` | 598794 | `cdc9956bb52395bb4d79277c06093c6d4a0d3498096c0d1d25115b7eb18e1157` |
| 36 | damage | `ui_damage.png` | 376715 | `249fa6dd976b225c240d02a20f4a8e56ca84841683539924780768f0d7fd480b` |
| 37 | repair | `ui_repair.png` | 383310 | `437fa7900cc6463e62f002d5cbd29f46399ec988156bd851e849a5c1d074ec32` |
| 38 | power | `ui_power.png` | 374429 | `d619c182aaf865d079d38123a39717f1f80ed26381204fb76766287cfe9c5668` |
| 39 | construction | `ui_construction.png` | 379937 | `698ec14fb6f8ba3e4f283aeb296bffabda01610d3cc1d18df6a26ec8823026d7` |
| 40 | research | `ui_research.png` | 365311 | `54f3fedb09dffb498863fb0f09626ca28bfc608139b278cef746c16d848a2e68` |
| 41 | roster | `ui_roster.png` | 247464 | `f9813569de87219e5bb3af1bf9f3c995111abd15840722ec207eb20e2b2ce532` |
| 42 | advanced_roster | `ui_advanced_roster.png` | 246352 | `feb2141b52a9b54ce25dded0ccdd351e45d1ad1978ffeb13629d0b17f654856f` |
| 43 | relationships | `ui_relationships.png` | 251059 | `1e6ba755a8176509c296eb703e640a470e01ddb650a135835a6d5b2f5e3241d0` |
| 44 | trauma | `ui_trauma.png` | 251365 | `3a9dfc5143feba1b0775665416848c24674b55212470be7b1f034c2664f612c7` |
| 45 | bonded_briefing | `ui_bonded_briefing.png` | 207245 | `e1198436c056b8a0ac6fbe70734c1a72af5d6e32eda6c9a42ae0ee1756ae9978` |
| 46 | legacy | `ui_legacy.png` | 249689 | `ff35a231a988ba8e93aacb41da8694b8825a804971b4fff3121b5e4132e49233` |
| 47 | briefing | `ui_briefing.png` | 205455 | `48a037cde4ccc475fa922b274ef961ad872ade892e869555295739441370ed64` |
| 48 | threat_briefing | `ui_threat_briefing.png` | 199810 | `0dc4caa7bf522c5ab13cc7bf8b1cac18d362276a90f3d72eaa4d870d9bc002a7` |
| 49 | loadout_briefing | `ui_loadout_briefing.png` | 206161 | `c8a2b431ccc78e20094f854d9c72d6dbf471186bfe8220c941906e64b0bd8ca5` |
| 50 | pressure | `ui_pressure.png` | 214478 | `5984586ffd3141e0031790dc1f086b4257702ed60a5ba564a67fc157bee5de9c` |
| 51 | gameplay | `ui_gameplay.png` | 598798 | `16a734b06258b9a6b46ed484c0d870e547787b6e08dcaccc83a420065caa4266` |
| 52 | overwatch | `ui_overwatch.png` | 603496 | `fa1f4bb27b08c4b31367b9aac47a7a4667aee88c5d2b46e7af59678ded53ba5d` |
| 53 | brood_ability | `ui_brood_ability.png` | 695146 | `c48ac3d000b4cbe8d0a939ed3408877bcf2641418a035b2e9f3c75530d2d08f9` |
| 54 | directorate_ability | `ui_directorate_ability.png` | 690251 | `2ae8519ea78f0564bc34ec1be30f2a6ad555e75193e2043983c482bc01c7a483` |
| 55 | ascendant_ability | `ui_ascendant_ability.png` | 693004 | `10381777c63cdd21d0ece738dad8a86cf4c636b0d86f1fce7ec766ab8d756be8` |
| 56 | hazard | `ui_hazard.png` | 702745 | `2881184d6b0edf435b3c9e88c9cb3448e5c98736e86ac3a161e7b39eb4a5283d` |
| 57 | intent | `ui_intent.png` | 614919 | `3326bee591da5e462af4d3b7670a038da10782582eb2f51ea42ea5b900e6e055` |
| 58 | action_preview | `ui_action_preview.png` | 660936 | `1fbba5027023f8a53a88a66e12b0ea844d48db283b97bde24dc446ffebb7464d` |
| 59 | movement_route | `ui_movement_route.png` | 586582 | `0397fd823ccb59cf0c53b0b78a6a5602b16b47f8def6d3c51c2f334e342da939` |
| 60 | cover_edges | `ui_cover_edges.png` | 601286 | `2d95857c8859d6ca6251991152b61462a33dbfbdeaca2b9841be7106e6aa67af` |
| 61 | invalid_command | `ui_invalid_command.png` | 690574 | `f5f2910e8cca7c74ebad47227c551b8074bfbd40823fac15492b2db29a0ade53` |
| 62 | valid_shot | `ui_valid_shot.png` | 656950 | `3dfae9d8f7d22c7c99616cfe664ef20bec053258ff56898918089e22492caca8` |
| 63 | threat_range | `ui_threat_range.png` | 683453 | `2b55932ddd56fe828bf998575a38a65bc18189986ae38dbdfa9545a7e227dc21` |
| 64 | danger_reach | `ui_danger_reach.png` | 674431 | `011be7ac677aa4fbf682a6a9653dbfaa623fab94bc4bf614dcd195a49efa3673` |
| 65 | help | `ui_help.png` | 331010 | `b3039abf421309b38c56daf841921f4b69eb15c6cb2dd5841c1ff2851b16d484` |
| 66 | battle_log | `ui_battle_log.png` | 378054 | `038b2967bc02d6f32b15277b6f4f797973301925823012d1964a22acead9204b` |
| 67 | combat_feedback | `ui_combat_feedback.png` | 607487 | `680dc8679e4948cd9d8269392e769aeac19a7259c7d897e1ac248ca5ae0b3dbb` |
| 68 | phase_replay | `ui_phase_replay.png` | 571132 | `63023b6a6c9c23315d14e854ed070637de7685ba60c8c608aec9b1af16baa56c` |
| 69 | end_phase_guard | `ui_end_phase_guard.png` | 599012 | `6a17dfe2c56e3700c57e7187cc8c104fe77127148e184c418ca88ce80659af56` |
| 70 | readiness_markers | `ui_readiness_markers.png` | 602529 | `799be5118af82a3085487f61f34b5497d23e6f64413812a27ab10d5bc684d60c` |
| 71 | vitality_markers | `ui_vitality_markers.png` | 602937 | `2e0fe08a8a815f9f41154ffb0817bb217c76a36910a59c7e44cf1a6b2a8fff26` |
| 72 | extraction | `ui_extraction.png` | 727138 | `a5e7d7e4b27e1df59001674180a737520ac1a5c7841b92d00ee1d57ee01a32e0` |
| 73 | variant | `ui_variant.png` | 727271 | `ee65cd1cf69184c2042e1a5a963f27f8850d5fd52066fec3d93cd98cdd3b1f52` |
| 74 | sporefield | `ui_sporefield.png` | 604163 | `f79576b38122078d68a7102aa1f8c42929a6dd16c945cf9103ba48ee0e6b6c2c` |
| 75 | vault | `ui_vault.png` | 599260 | `726a12ba8c03aeb9e2c299c93813a428498b4b89f46c4b44fa839f7050984de3` |
| 76 | black_channel | `ui_black_channel.png` | 605792 | `fb5b17ea89d22df2ed524ebc7d121de8b8d1f843c3e2bf1f0f647bc36bd5870d` |
| 77 | living_chorus | `ui_living_chorus.png` | 616644 | `6f64c3c5a289a07c1763a4b79a85a95d3f60ed32485c20fe3aaea881655063e4` |
| 78 | open_circuit | `ui_open_circuit.png` | 612054 | `9988efe0414106c384eb6047da87c1c632e070262161a08355f098b17aeae42c` |
| 79 | trace_active | `ui_trace_active.png` | 596070 | `8ece361e7f1ae4a5971078a3d5cab72c995e912b233bb6514caddb5ccddad109` |
| 80 | equipment | `ui_equipment.png` | 542728 | `9cda23dfb7cafcf93665c65b6f9b39ca507d90af5bb51209f3c599915de5c279` |
| 81 | weapon_profile | `ui_weapon_profile.png` | 598865 | `6e6ba2af92b0a0d70ae693d6c37dbf358e457db57297bf80f241dc890a00dbcd` |
| 82 | class_target | `ui_class_target.png` | 553981 | `3882838df8d4214ce63c2e0899b3b22bc58b9123a40c8931a1ac4aea083eac12` |
| 83 | breach | `ui_breach.png` | 685941 | `441df66f23a1a3a8b5fee7c884e951eeead7f4771aa5322ef67cd7631af4f847` |
| 84 | debrief | `ui_debrief.png` | 195991 | `09b030be92a25fbb21910627814c1e999f5512b345f90b21c67ffafd20ab1e09` |
| 85 | trauma_debrief | `ui_trauma_debrief.png` | 203021 | `e10788bd1ebad45bd57c5457c05cf3191ec3ba6b4d49554c5e5be5449359f723` |
