use super::*;

#[test]
fn evolution_button_labels_disclose_biomass_gate() {
    assert_eq!(evolution_button_label(true, 20), "EVOLVE // 20 BIOMASS");
    assert_eq!(evolution_button_label(false, 20), "NEEDS 20 BIOMASS");
}

#[test]
fn full_recruited_roster_stays_inside_gene_lab_list() {
    let (default_step, default_height) = character_list_row_layout(5);
    assert_eq!((default_step, default_height), (88.0, 72.0));

    let (step, height) = character_list_row_layout(7);
    let last_row_bottom = 154.0 + 6.0 * step + height;
    assert!(step < default_step);
    assert!(last_row_bottom <= 568.0);
    assert!(height >= 50.0);
}

#[test]
fn evolution_options_clear_the_anatomy_scan_and_return_control() {
    let bio_scan_baseline = 328.0;
    let first_option = evolution_option_y(0);
    let second_option = evolution_option_y(1);
    let return_button_top = 622.0;

    assert!(first_option > bio_scan_baseline);
    assert!(second_option + 100.0 < return_button_top);
}
