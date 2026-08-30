use super::*;

#[test]
fn outer_route_consequence_names_the_second_operation_result() {
    assert_eq!(outer_route_label(Some(true)), "OUTER ROUTES HARDENED");
    assert_eq!(outer_route_label(Some(false)), "OUTER ROUTE BREACHED");
    assert_eq!(outer_route_label(None), "OUTER ROUTE UNRESOLVED");
    assert!(outer_route_is_breached(Some(false)));
    assert!(!outer_route_is_breached(Some(true)));
}
