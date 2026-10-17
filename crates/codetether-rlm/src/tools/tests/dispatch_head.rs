//! Existing dispatch_head contract check.
use super::*;

#[test]
fn dispatch_head() {
    let ctx = "line 1\nline 2\nline 3\nline 4\nline 5".to_string();
    let mut repl = RlmRepl::new(ctx, ReplRuntime::Rust);
    let result = dispatch_tool_call("rlm_head", r#"{"n": 2}"#, &mut repl);
    match result {
        Some(RlmToolResult::Output(s)) => assert_eq!(s, "line 1\nline 2"),
        _ => panic!("expected Output"),
    }
}
