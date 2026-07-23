//! Remote-tab transcript projection tests.

use super::*;
#[test]
fn selected_mdns_agent_replaces_the_main_chat_transcript() {
    let suffix = uuid::Uuid::new_v4();
    let name = format!("peer-{suffix}");
    let parent = format!("parent-{suffix}");
    crate::tool::agent::bridge::record_remote_turn(
        &name,
        &parent,
        "inspect backend",
        "backend is healthy",
    );
    let mut app = App::default();
    app.state.session_id = Some(parent);
    app.state.active_spawned_agent = Some(name);
    app.state
        .messages
        .push(crate::tui::chat::message::ChatMessage::new(
            crate::tui::chat::message::MessageType::User,
            "main-only message",
        ));

    let formatter = crate::tui::message_formatter::MessageFormatter::new(76);
    let palette = crate::tui::color_palette::ColorPalette::marketing();
    let drawn = super::super::lines::build_chat_lines(&mut app, 80, 80, &formatter, &palette);
    let text = drawn
        .as_slice()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(text.contains("A2A REQUEST"));
    assert!(text.contains("A2A RESPONSE"));
    assert!(text.contains("inspect backend"));
    assert!(text.contains("backend is healthy"));
    assert!(!text.contains("main-only message"));
}