use ratatui::text::Line;

use super::super::AppState;

#[test]
fn idle_cache_is_reusable() {
    let mut state = AppState::default();
    state.store_message_lines(vec![Line::from("cached")], 80);

    assert!(state.is_message_cache_valid(80));
}

#[test]
fn streaming_replacement_invalidates_cached_suffix() {
    let mut state = AppState::default();
    state.processing = true;
    state.replace_streaming_text("first".to_string());
    state.store_message_lines(vec![Line::from("first")], 80);
    assert!(state.is_message_cache_valid(80));

    state.replace_streaming_text("other".to_string());
    assert!(!state.is_message_cache_valid(80));
}
