//! Application reviewer tool-surface guarantees.
use crate::review::read_only_tools;

#[test]
fn reviewer_tools_are_all_read_only() {
    let tools = read_only_tools();
    for denied in [
        "write",
        "edit",
        "multiedit",
        "apply_patch",
        "patch",
        "bash",
        "exec_command",
    ] {
        assert!(
            tools.get(denied).is_none(),
            "{denied} leaked into the reviewer"
        );
    }
    for allowed in ["read", "rg", "grep", "glob", "lsp", "list"] {
        assert!(
            tools.get(allowed).is_some(),
            "{allowed} missing from the reviewer"
        );
    }
}
