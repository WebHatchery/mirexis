use super::*;

#[test]
fn attack_button_stays_inside_the_forecast_card() {
    let panel = Rect::new(10.0, 74.0, 900.0, 608.0);
    let card = attack_card_bounds(panel);
    let button = attack_button_bounds(card);

    assert!(button.x >= card.x);
    assert!(button.y >= card.y);
    assert!(button.right() <= card.right());
    assert!(button.bottom() <= card.bottom());
}
