//! Spawn-to-watch transition tests.

use super::*;

#[test]
fn opens_the_new_child_transcript_immediately() {
    let mut app = App::default();

    open(&mut app, "observer");

    assert_eq!(app.state.view_mode, ViewMode::Subagents);
    assert_eq!(app.state.active_spawned_agent.as_deref(), Some("observer"));
    assert!(app.state.subagent_detail_mode);
    assert_eq!(app.state.subagent_detail_scroll, 0);
}
