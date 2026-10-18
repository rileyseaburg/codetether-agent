use super::estimate;

#[test]
fn opaque_signature_counts_toward_context_budget() {
    let without = estimate("summary", &None);
    let with = estimate("summary", &Some("opaque".repeat(100)));

    assert!(with > without);
}
