//! A task at the start of the retained window needs no prefix summary.
use super::*;
#[test]
fn returns_uncompressed_when_the_prefix_is_empty() {
    let ResetPreparation::Ready(context) = prepare(vec![message(Role::User, "task")], 1, 0, |_| 1)
    else {
        panic!("ready context expected")
    };
    assert_eq!(context.provenance, ["reset_without_prefix"]);
    assert!(!context.compressed);
    assert_eq!(context.messages.len(), 1);
}
