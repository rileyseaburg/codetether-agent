//! toc lists user turns with base offset.
use super::*;
#[test]
fn toc_lists_user_turns_with_base_offset() {
    let prefix = vec![
        text(Role::User, "implement auth middleware"),
        text(Role::Assistant, "done"),
        text(Role::User, "now fix the failing tests"),
    ];
    let toc = render_toc(&prefix, 5);
    assert!(toc.contains("[DROPPED-RANGE INDEX] turns 5-7"));
    assert!(toc.contains("[turn 5] implement auth middleware"));
    assert!(toc.contains("[turn 7] now fix the failing tests"));
    assert!(toc.contains("context_browse"));
    assert!(toc.contains("When the user allows history access"));
}
