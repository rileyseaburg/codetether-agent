//! Original prose recovery regression case.
use super::support::bash_tool;
use crate::extract_tool_call_from_prose;

#[test]
fn extracts_command_after_run_verb_without_backticks() {
    let tools = bash_tool();
    let (_id, name, args) = extract_tool_call_from_prose("Let me use bash and run ls -la", &tools)
        .expect("should extract a synthetic bash call");
    assert_eq!(name, "bash");
    assert_eq!(args["command"], "ls -la");
}
