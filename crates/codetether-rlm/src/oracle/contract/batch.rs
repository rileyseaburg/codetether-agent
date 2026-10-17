//! Consensus and batch bucket contracts.
use super::*;
#[test]
fn consensus_and_batch_snapshot() {
    let results = [
        analysis("explain", "same"),
        analysis("explain", "same"),
        analysis("explain", "other"),
    ];
    let accepted = capture(
        TraceValidator::new()
            .with_consensus_threshold(0.6)
            .validate_with_consensus(&results, "", Some("main.rs"), Some("rev"), None),
    );
    assert_eq!(accepted["kind"], "consensus");
    let rejected =
        capture(TraceValidator::new().validate_with_consensus(&results, "", None, None, None));
    assert_eq!(rejected["kind"], "unverified");
    println!("CONSENSUS={}", serde_json::json!([accepted, rejected]));
    let batch = TraceValidator::new().batch_validate(vec![
        (
            analysis("count async functions", "Found 1 async functions"),
            "async fn f() {}",
            None,
        ),
        (analysis("explain", "text"), "source", None),
        (analysis("explain", "{broken"), "source", None),
    ]);
    assert_eq!(batch.total(), 3);
    assert_eq!(batch.golden.len(), 1);
    assert_eq!(batch.unverified.len(), 1);
    assert_eq!(batch.failed.len(), 1);
}
