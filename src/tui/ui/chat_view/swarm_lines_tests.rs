use super::super::lines::build_chat_lines;
use crate::tui::app::{navigation::cycle_agent_focus, state::App};
use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;

#[path = "swarm_lines_test_fixture.rs"]
mod fixture;

#[test]
fn tab_focus_loads_swarm_transcript_into_chat() {
    let mut app = App::default();
    app.state.swarm.subtasks.push(fixture::task());

    cycle_agent_focus(&mut app);

    let formatter = MessageFormatter::new(76);
    let palette = ColorPalette::marketing();
    let drawn = build_chat_lines(&mut app, 80, 80, &formatter, &palette);
    let text = drawn
        .as_slice()
        .iter()
        .flat_map(|line| line.spans.iter())
        .map(|span| span.content.as_ref())
        .collect::<String>();
    assert_eq!(
        app.state.active_spawned_agent.as_deref(),
        Some("worker-one")
    );
    assert!(text.contains("worker transcript"));
    assert!(text.contains("Session: task-one"));
}
