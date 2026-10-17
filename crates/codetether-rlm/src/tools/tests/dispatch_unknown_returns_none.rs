//! Existing dispatch_unknown_returns_none contract check.
use super::*;

#[test]
fn dispatch_unknown_returns_none() {
    let ctx = "data".to_string();
    let mut repl = RlmRepl::new(ctx, ReplRuntime::Rust);
    assert!(dispatch_tool_call("unknown_tool", "{}", &mut repl).is_none());
}
