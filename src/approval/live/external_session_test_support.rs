//! Session-grant cleanup for external approval tests.

pub(super) struct Reset;

impl Drop for Reset {
    fn drop(&mut self) {
        crate::approval::session_grants::reset();
    }
}

pub(crate) fn assert_session_scope() {
    assert!(crate::approval::session_grants::allowed_scoped(
        "generic_mutator",
        "execute",
        "resource",
        Some("session-a"),
    ));
    assert!(!crate::approval::session_grants::allowed_scoped(
        "generic_mutator",
        "execute",
        "resource",
        Some("session-b"),
    ));
}
