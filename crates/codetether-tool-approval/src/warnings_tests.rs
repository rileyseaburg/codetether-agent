//! Verification warnings preserve write success and retain structured diagnostics.

use super::annotate;
use serde_json::json;

#[test]
fn successful_write_exposes_unavailable_verification() {
    let result = annotate(
        ("Modified 1 files".into(), true, None),
        vec!["deadline exceeded".into()],
    );
    assert!(result.1);
    assert!(result.0.starts_with("Modified 1 files"));
    assert!(result.0.contains("LSP_PREAPPROVAL_UNAVAILABLE"));
    let metadata = result.2.unwrap();
    assert_eq!(metadata["lsp_preapproval"]["status"], "unavailable");
    assert_eq!(
        metadata["lsp_preapproval"]["warnings"],
        json!(["deadline exceeded"])
    );
}

#[test]
fn warnings_never_turn_a_failure_into_success() {
    let result = annotate(
        ("denied".into(), false, None),
        vec!["server unavailable".into()],
    );
    assert!(!result.1);
    assert!(result.0.starts_with("denied"));
}

#[test]
fn healthy_verification_preserves_the_exact_result() {
    let original = ("Modified 1 files".into(), true, None);
    assert_eq!(annotate(original.clone(), Vec::new()), original);
}
