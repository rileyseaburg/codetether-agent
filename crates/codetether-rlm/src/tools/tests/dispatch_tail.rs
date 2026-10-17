//! Existing dispatch_tail contract check.
use super::*;

#[test]
fn dispatch_tail() {
    let ctx = "line 1\nline 2\nline 3\nline 4\nline 5".to_string();
    let mut repl = RlmRepl::new(ctx, ReplRuntime::Rust);
    let result = dispatch_tool_call("rlm_tail", r#"{"n": 2}"#, &mut repl);
    match result {
        Some(RlmToolResult::Output(s)) => assert_eq!(s, "line 4\nline 5"),
        _ => panic!("expected Output"),
    }
}
