//! Nested mutation batches retain atomic, sorted scope aggregation.
use serde_json::json;

#[test]
fn batch_aggregates_nested_mutations_atomically() {
    let input = json!({ "calls": [
        { "tool": "read", "args": { "path": "ignored" } },
        { "tool": "write", "args": { "path": "a.rs" } },
        { "name": "multiedit", "arguments": { "edits": [{ "file": "b.rs" }] } }
    ] });
    let paths = super::paths::mutation_paths("batch", &input).unwrap();
    assert_eq!(
        paths,
        vec![
            std::path::PathBuf::from("a.rs"),
            std::path::PathBuf::from("b.rs")
        ]
    );
}
