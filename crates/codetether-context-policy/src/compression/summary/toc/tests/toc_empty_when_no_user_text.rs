//! toc empty when no user text.
use super::*;
#[test]
fn toc_empty_when_no_user_text() {
    let prefix = vec![text(Role::Assistant, "only assistant output")];
    assert!(render_toc(&prefix, 0).is_empty());
    assert!(render_toc(&[], 0).is_empty());
}
