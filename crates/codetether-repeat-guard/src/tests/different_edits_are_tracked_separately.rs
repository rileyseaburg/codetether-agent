//! Original repeat-call guard regression case.
use crate::RepeatGuard;
use serde_json::json;

#[test]
fn different_edits_are_tracked_separately() {
    let mut g = RepeatGuard::default();
    let a = json!({"path": "a.rs", "new_string": "aa"});
    let b = json!({"path": "b.rs", "new_string": "bb"});
    for _ in 0..3 {
        g.check("write", &a);
    }
    assert!(g.check("write", &b).is_none());
}
