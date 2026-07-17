use layered_config::{Config, merge};

#[test]
fn inherits_absent_values_and_deduplicates_labels() {
    let base = Config {
        timeout: Some(30),
        retries: Some(2),
        labels: vec!["api".into(), "stable".into()],
    };
    let overrides = Config {
        timeout: None,
        retries: Some(5),
        labels: vec!["stable".into(), "fast".into()],
    };
    let merged = merge(base, overrides);
    assert_eq!(merged.timeout, Some(30));
    assert_eq!(merged.retries, Some(5));
    assert_eq!(merged.labels, ["api", "stable", "fast"]);
}
