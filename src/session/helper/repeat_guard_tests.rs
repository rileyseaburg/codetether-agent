use super::repeat_guard::*;
use serde_json::json;

#[test]
fn non_edit_tools_are_not_tracked() {
    let mut g = RepeatGuard::default();
    assert!(g.check("read", &json!({"path": "a.rs"})).is_none());
    assert!(g.check("read", &json!({"path": "a.rs"})).is_none());
}

#[test]
fn allows_up_to_threshold_repeats() {
    let mut g = RepeatGuard::default();
    let inp = json!({"path": "a.rs", "old_string": "x", "new_string": "y"});
    assert!(g.check("edit", &inp).is_none());
    assert!(g.check("edit", &inp).is_none());
    assert!(g.check("edit", &inp).is_none());
}

#[test]
fn blocks_after_threshold() {
    let mut g = RepeatGuard::default();
    let inp = json!({"path": "a.rs", "old_string": "x", "new_string": "y"});
    for _ in 0..3 {
        g.check("edit", &inp);
    }
    let blocked = g.check("edit", &inp);
    assert!(blocked.is_some());
    assert!(blocked.unwrap().contains("already attempted"));
}

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
