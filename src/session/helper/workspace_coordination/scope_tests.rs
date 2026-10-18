//! Native coordinator-result regression.

#[test]
fn coordinator_rejects_workspace_ownership() {
    let (output, success, metadata) = super::scope_error::result("exec_command");
    assert!(!success);
    assert!(output.contains("cannot own a workspace"));
    assert_eq!(metadata.unwrap()["error_code"], "WORKSPACE_LEASE_FORBIDDEN");
}
