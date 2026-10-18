//! Spawn-to-watch transition tests.

use super::*;

#[test]
fn opens_the_new_child_transcript_immediately() {
    let mut app = App::default();

    open(&mut app, "observer");

    assert_eq!(app.state.view.mode, ViewMode::Subagents);
    assert_eq!(app.state.agent_tree.active.as_deref(), Some("observer"));
    assert!(app.state.agent_tree.detail_mode);
    assert_eq!(app.state.agent_tree.detail_scroll, 0);
}
