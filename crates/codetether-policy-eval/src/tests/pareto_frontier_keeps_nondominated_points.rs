use super::fixtures::sample;
use crate::pareto_frontier;

#[test]
fn pareto_frontier_keeps_nondominated_points() {
    let results = vec![
        sample("legacy", 24_000, 12, 4, 0.50),
        sample("reset", 6_500, 5, 2, 0.62),
        sample("dominated", 30_000, 20, 10, 0.10),
    ];
    let frontier = pareto_frontier(&results);
    let labels: Vec<&str> = frontier.iter().map(|r| r.policy).collect();
    assert!(labels.contains(&"reset"));
    assert!(!labels.contains(&"dominated"));
}
