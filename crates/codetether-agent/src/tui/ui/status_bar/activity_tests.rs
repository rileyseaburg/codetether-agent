use super::*;
use std::time::Duration;

#[test]
fn activity_age_formats_and_resets() {
    let start = SystemTime::UNIX_EPOCH;
    for (seconds, expected) in [(0, "0s"), (59, "59s"), (61, "1m 01s"), (3661, "1h 01m 01s")] {
        let now = start + Duration::from_secs(seconds);
        assert_eq!(label(start, now), format!("Last activity: {expected}"));
        assert_eq!(label(now, now), "Last activity: 0s");
    }
    assert_eq!(
        label(start + Duration::from_secs(1), start),
        "Last activity: 0s"
    );
}

#[test]
fn activity_age_resets_on_a_new_message() {
    let mut app = App::default();
    app.state.last_activity_at = SystemTime::UNIX_EPOCH;
    app.state
        .messages
        .push(codetether_chat_model::ChatMessage::new(
            codetether_chat_model::MessageType::User,
            "hello",
        ));
    assert_eq!(badge(&app).content, "Last activity: 0s (message)");
    app.state.watchdog.last_event_at = Some(std::time::Instant::now());
    app.state.messages.clear();
    assert_ne!(badge(&app).content, "Last activity: 0s (startup)");
}
