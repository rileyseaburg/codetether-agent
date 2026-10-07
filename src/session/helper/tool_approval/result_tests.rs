use super::result::denied;

#[test]
fn denied_without_reason_uses_default_message() {
    let (output, success, _meta) = denied("bash", "approval-1", None);
    assert!(!success);
    assert!(output.contains("denied by the user"));
    assert!(!output.contains("denied by the user:"));
}

#[test]
fn denied_with_reason_forwards_reason_to_agent() {
    let (output, success, _meta) = denied("bash", "approval-1", Some("use ripgrep instead"));
    assert!(!success);
    assert!(output.contains("use ripgrep instead"));
}

#[test]
fn denied_blank_reason_and_retry_guidance() {
    let (output, _, meta) = denied("bash", "approval-1", Some("   "));
    assert!(!output.contains("denied by the user:"));
    assert!(output.contains("resubmit the same call unchanged"));
    assert_eq!(meta.unwrap()["retry_same_call"], false);
}
