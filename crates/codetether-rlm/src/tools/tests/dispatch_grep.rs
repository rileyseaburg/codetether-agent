//! Existing dispatch_grep contract check.
use super::*;

#[test]
fn dispatch_grep() {
    let ctx = "error: fail\ninfo: ok\nerror: boom".to_string();
    let mut repl = RlmRepl::new(ctx, ReplRuntime::Rust);
    let result = dispatch_tool_call("rlm_grep", r#"{"pattern": "error"}"#, &mut repl);
    match result {
        Some(RlmToolResult::Output(s)) => {
            assert!(s.contains("error: fail"));
            assert!(s.contains("error: boom"));
        }
        _ => panic!("expected Output"),
    }
}
