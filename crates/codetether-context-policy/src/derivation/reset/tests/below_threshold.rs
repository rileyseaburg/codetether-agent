//! Full-history pass-through preserves origin and provenance.
use super::*;
#[test]
fn returns_full_history_below_threshold() {
    let source = vec![message(Role::User, "task")];
    let ResetPreparation::Ready(context) = prepare(source, 1, 10, |_| 1) else {
        panic!("ready context expected")
    };
    assert_eq!(context.messages.len(), 1);
    assert_eq!(context.origin_len, 1);
    assert!(!context.compressed);
    assert_eq!(context.provenance, ["reset_below_threshold"]);
}
