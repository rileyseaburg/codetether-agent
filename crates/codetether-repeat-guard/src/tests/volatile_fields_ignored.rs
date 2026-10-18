//! Original repeat-call guard regression case.
use crate::RepeatGuard;
use serde_json::json;

#[test]
fn volatile_fields_ignored() {
    let mut g = RepeatGuard::default();
    let base = json!({"path": "a.rs", "new_string": "y"});
    for _ in 0..3 {
        g.check("edit", &base);
    }
    let with_id = json!({
        "path": "a.rs", "new_string": "y",
        "tool_call_id": "call_abc123"
    });
    assert!(g.check("edit", &with_id).is_some());
}
