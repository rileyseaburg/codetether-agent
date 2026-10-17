//! Existing dispatch_final contract check.
use super::*;

#[test]
fn dispatch_final() {
    let ctx = "whatever".to_string();
    let mut repl = RlmRepl::new(ctx, ReplRuntime::Rust);
    let result = dispatch_tool_call("rlm_final", r#"{"answer": "The answer is 42"}"#, &mut repl);
    match result {
        Some(RlmToolResult::Final(s)) => assert_eq!(s, "The answer is 42"),
        _ => panic!("expected Final"),
    }
}
