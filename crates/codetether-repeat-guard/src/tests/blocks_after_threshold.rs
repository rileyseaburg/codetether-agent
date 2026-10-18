//! Original repeat-call guard regression case.
use crate::RepeatGuard;
use serde_json::json;

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
