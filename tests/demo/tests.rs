use super::*;

#[test]
fn full_build_never_applies_the_demo_cutoff() {
    if !is_demo_build() {
        assert!(!campaign_is_complete(MISSION_LIMIT));
        assert!(!campaign_is_complete(u32::MAX));
    }
}

#[test]
fn demo_build_ends_after_exactly_three_operations() {
    if is_demo_build() {
        assert!(!campaign_is_complete(MISSION_LIMIT - 1));
        assert!(campaign_is_complete(MISSION_LIMIT));
        assert!(campaign_is_complete(MISSION_LIMIT + 1));
    }
}
