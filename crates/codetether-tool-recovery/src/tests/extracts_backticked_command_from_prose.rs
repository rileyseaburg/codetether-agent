//! Original prose recovery regression case.
use super::support::bash_tool;
use crate::extract_tool_call_from_prose;

#[test]
fn extracts_backticked_command_from_prose() {
    let tools = bash_tool();
    let (_id, name, args) =
        extract_tool_call_from_prose("I'll use bash to run `ls -la` now.", &tools)
            .expect("should extract a synthetic bash call");
    assert_eq!(name, "bash");
    assert_eq!(args["command"], "ls -la");
}
