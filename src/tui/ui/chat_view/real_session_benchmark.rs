//! Manual renderer benchmark against an exported real session.

use std::time::Instant;

use super::super::lines::build_chat_lines;
use crate::provider::Message;
use crate::tui::app::{message_text::provider_messages_to_chat_messages, state::App};
use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;

#[test]
#[ignore = "set CODETETHER_TUI_PROFILE_SESSION to a saved session JSON"]
fn profiles_real_saved_session() {
    let path = std::env::var("CODETETHER_TUI_PROFILE_SESSION").expect("session path");
    let raw = std::fs::read(path).expect("read session");
    let value: serde_json::Value = serde_json::from_slice(&raw).expect("parse session");
    let messages: Vec<Message> = serde_json::from_value(value["messages"].clone()).unwrap();
    let mut app = App::default();
    app.state.messages = provider_messages_to_chat_messages(&messages);
    let formatter = MessageFormatter::new(176);
    let palette = ColorPalette::marketing();

    let cold = timed(&mut app, &formatter, &palette);
    let cached = timed(&mut app, &formatter, &palette);
    app.state.start_pending_tool("benchmark".into());
    let pending = timed(&mut app, &formatter, &palette);
    println!(
        "provider={} chat={} cold={:?} cached={:?} pending={:?}",
        messages.len(),
        app.state.messages.len(),
        cold,
        cached,
        pending
    );
}

fn timed(
    app: &mut App,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
) -> std::time::Duration {
    let started = Instant::now();
    let drawn = build_chat_lines(app, 180, 180, formatter, palette);
    let elapsed = started.elapsed();
    drawn.restore(app, 180);
    elapsed
}
