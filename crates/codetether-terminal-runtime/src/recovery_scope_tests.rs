use super::{active, enter};

#[test]
fn recovery_scope_is_reset_by_guard() {
    assert!(!active());
    {
        let _guard = enter();
        assert!(active());
    }
    assert!(!active());
}
