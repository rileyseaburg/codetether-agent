//! Original repeat-call guard regression case.
use crate::RepeatGuard;
use serde_json::json;

#[test]
fn allows_up_to_threshold_repeats() {
    let mut g = RepeatGuard::default();
    let inp = json!({"path": "a.rs", "old_string": "x", "new_string": "y"});
    assert!(g.check("edit", &inp).is_none());
    assert!(g.check("edit", &inp).is_none());
    assert!(g.check("edit", &inp).is_none());
}
