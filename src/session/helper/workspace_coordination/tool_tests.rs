//! Coverage for indirect and dynamically targeted workspace writes.

use serde_json::json;
use std::path::PathBuf;

#[test]
fn interactive_input_and_indirect_tools_claim_paths() {
    let stdin = super::paths::mutation_paths("write_stdin", &json!({ "chars": "rm a" }));
    assert_eq!(stdin.unwrap(), vec![PathBuf::new()]);
    assert!(super::paths::mutation_paths("write_stdin", &json!({ "chars": "" })).is_none());

    let screenshot = json!({ "action": "screenshot", "path": "proof.png" });
    assert_eq!(
        super::paths::mutation_paths("browserctl", &screenshot).unwrap(),
        vec![PathBuf::from("proof.png")]
    );
    let plugin = super::paths::mutation_paths("tetherscript_plugin", &json!({}));
    assert_eq!(plugin.unwrap(), vec![PathBuf::new()]);
}

#[test]
fn shell_output_options_are_not_treated_as_read_only() {
    assert!(!super::shell::read_only("find . -fprint files.txt"));
    assert!(!super::shell::read_only("git diff --output=changes.diff"));
    assert!(!super::shell::read_only("sort -o sorted.txt input.txt"));
    assert!(!super::shell::read_only("rg needle src & rm src/lib.rs"));
}

#[test]
fn timeout_message_tells_the_agent_not_to_wait_manually() {
    let conflict = crate::mux::lease::WorktreeLease {
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
