use super::support::draw;
use crate::tui::app::state::App;

#[test]
fn activity_events_visible_without_reply_text() {
    let mut app = App::default();
    app.state.processing = true;
    app.state
        .activity_events
        .record("Thinking update received".into());
    app.state
        .activity_events
        .record("Thinking update received".into());
    let text = draw(&mut app, 120, 35);
    assert!(text.contains("Recent events"), "{text}");
    assert!(text.contains("Thinking update received ×2"), "{text}");
    assert!(app.state.messages.is_empty());
    app.state
        .activity_events
        .record("Tool started: bash".into());
    app.state.message_cache.invalidate_streaming();
    let text = draw(&mut app, 120, 35);
    assert!(text.contains("Tool started: bash"), "{text}");
}

#[test]
fn activity_events_retain_only_four_groups() {
    let mut app = App::default();
    for index in 0..10 {
        app.state.activity_events.record(format!("Event {index}"));
    }
    let text = draw(&mut app, 120, 35);
    assert!(!text.contains("Event 0"));
    assert!(text.contains("Event 9"), "{text}");
    assert_eq!(app.state.activity_events.entries.len(), 4);
}
