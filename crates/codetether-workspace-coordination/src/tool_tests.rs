//! Native coordinator-result regression.

#[test]
fn timeout_message_tells_the_agent_not_to_wait_manually() {
    let conflict = codetether_worktree_lease::WorktreeLease {
        owner: "turn-a".into(),
        agent: "build".into(),
        workspace: "/work".into(),
        path: "src".into(),
        expires_at_ms: 90_000,
    };
    let (output, success, metadata) =
        super::gate_error::conflict("apply_patch", &[conflict], 60_000, 30_000);
    assert!(!success);
    assert!(output.contains("already waited 60s"));
    assert!(output.contains("Do not call wait_agent"));
    assert!(output.contains("ask another agent"));
    assert_eq!(
        metadata.unwrap()["error_code"],
        "WORKTREE_LEASE_WAIT_TIMEOUT"
    );
}
