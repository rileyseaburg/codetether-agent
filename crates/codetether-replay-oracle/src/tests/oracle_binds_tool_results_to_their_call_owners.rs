use super::fixtures::*;
use crate::replay_oracle;
use codetether_provider_core::Role;

#[test]
fn oracle_binds_tool_results_to_their_call_owners() {
    let msgs = vec![
        text(Role::User, "do the thing"),
        tool_call("call-1", "Shell"),
        tool_result("call-1", "ok"),
    ];
    let report = replay_oracle(&msgs, 5);
    // Turn 2 (ToolResult) should reference turn 1 (its ToolCall).
    assert!(report.demand[2].contains(&1));
    assert!(report.reference_count() >= 1);
}
