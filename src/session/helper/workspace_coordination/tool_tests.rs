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
