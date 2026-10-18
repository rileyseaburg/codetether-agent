//! Original repeat-call guard regression case.
use crate::RepeatGuard;
use serde_json::json;

#[test]
fn non_edit_tools_are_not_tracked() {
    let mut g = RepeatGuard::default();
    assert!(g.check("read", &json!({"path": "a.rs"})).is_none());
    assert!(g.check("read", &json!({"path": "a.rs"})).is_none());
}
