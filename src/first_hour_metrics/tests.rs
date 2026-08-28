use super::*;

#[test]
fn first_occurrence_timings_are_stable() {
    let mut metrics = FirstHourMetrics::default();
    metrics.tick(1.25);
    metrics.city_moved();
    metrics.tick(0.75);
    metrics.city_moved();
    metrics.city_interacted();

    assert_eq!(metrics.first_city_move_millis, Some(1_250));
    assert_eq!(metrics.first_city_interaction_millis, Some(2_000));
}

#[test]
fn operation_duration_and_rounds_are_recorded() {
    let mut metrics = FirstHourMetrics::default();
    metrics.tick(2.0);
    metrics.operation_started();
    metrics.tick(65.5);
    metrics.operation_resolved(1, 4);

    assert_eq!(metrics.operation_one_duration_millis, Some(65_500));
    assert_eq!(metrics.operation_one_rounds, Some(4));
}

#[test]
fn missing_serialized_metrics_use_safe_defaults() {
    let metrics: FirstHourMetrics = serde_json::from_str("{}").unwrap();
    assert_eq!(metrics, FirstHourMetrics::default());
}
