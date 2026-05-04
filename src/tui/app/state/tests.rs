//! State module tests covering cursor movement, scrolling, model picker, and timing.

use super::AppState;
use std::time::{Duration, Instant};

#[test]
fn utf8_cursor_moves_by_character_not_byte() {
    let mut state = AppState {
        input: "aé🙂z".to_string(),
        ..Default::default()
    };
    state.move_cursor_end();
    assert_eq!(state.input_cursor, 4);
    state.move_cursor_left();
    assert_eq!(state.input_cursor, 3);
    state.move_cursor_left();
    assert_eq!(state.input_cursor, 2);
    state.move_cursor_right();
    assert_eq!(state.input_cursor, 3);
}

#[test]
fn utf8_backspace_removes_full_character() {
    let mut state = AppState {
        input: "aé🙂z".to_string(),
        ..Default::default()
    };
    state.move_cursor_end();
    state.move_cursor_left();
    state.delete_backspace();
    assert_eq!(state.input, "aéz");
    assert_eq!(state.input_cursor, 2);
}

#[test]
fn utf8_delete_forward_removes_full_character() {
    let mut state = AppState {
        input: "aé🙂z".to_string(),
        input_cursor: 1,
        ..Default::default()
    };
    state.delete_forward();
    assert_eq!(state.input, "a🙂z");
    assert_eq!(state.input_cursor, 1);
}

#[test]
fn utf8_insert_respects_character_cursor() {
    let mut state = AppState {
        input: "a🙂z".to_string(),
        input_cursor: 2,
        ..Default::default()
    };
    state.insert_char('é');
    assert_eq!(state.input, "a🙂éz");
    assert_eq!(state.input_cursor, 3);
}

#[test]
fn model_filter_limits_visible_models() {
    let mut state = AppState {
        available_models: vec![
            "zai/glm-5".to_string(),
            "openai/gpt-4o".to_string(),
            "openrouter/qwen/qwen3-coder".to_string(),
        ],
        ..Default::default()
    };
    state.model_filter_push('g');
    state.model_filter_push('p');
    state.model_filter_push('t');
    assert_eq!(state.filtered_models(), vec!["openai/gpt-4o"]);
    assert_eq!(state.selected_model(), Some("openai/gpt-4o"));
}

#[test]
fn scroll_up_from_follow_latest_enters_manual_scroll_mode() {
    let mut state = AppState::default();
    state.set_chat_max_scroll(25);
    state.scroll_to_bottom();
    state.scroll_up(1);
    assert_eq!(state.chat_scroll, 24);
}

#[test]
fn scroll_down_returns_to_follow_latest_at_bottom() {
    let mut state = AppState::default();
    state.set_chat_max_scroll(25);
    state.chat_scroll = 24;
    state.scroll_down(1);
    assert_eq!(state.chat_scroll, 1_000_000);
}

#[test]
fn tool_preview_scroll_auto_follow_and_clamp() {
    let mut state = AppState::default();
    // Setting max_scroll while in auto-follow (default from reset) snaps to bottom.
    state.reset_tool_preview_scroll();
    state.set_tool_preview_max_scroll(4);
    assert_eq!(state.tool_preview_scroll, 4);
    // Scrolling down past max re-enables auto-follow sentinel.
    state.tool_preview_scroll = 3;
    state.scroll_tool_preview_down(10);
    assert_eq!(state.tool_preview_scroll, super::scroll::TOOL_PREVIEW_FOLLOW);
    // Scrolling up cancels auto-follow, starts from current max_scroll.
    state.scroll_tool_preview_up(2);
    assert_eq!(state.tool_preview_scroll, 2);
    // Reducing max_scroll clamps.
    state.set_tool_preview_max_scroll(1);
    assert_eq!(state.tool_preview_scroll, 1);
}

#[test]
fn request_timing_captures_first_and_last_token() {
    let mut state = AppState::default();
    state.processing_started_at = Some(
        Instant::now()
            .checked_sub(Duration::from_millis(12))
            .unwrap(),
    );
    state.note_text_token();
    let first = state
        .current_request_first_token_ms
        .expect("first token should be recorded");
    assert_eq!(state.current_request_last_token_ms, Some(first));
    state.processing_started_at = Some(
        Instant::now()
            .checked_sub(Duration::from_millis(34))
            .unwrap(),
    );
    state.note_text_token();
    let last = state
        .current_request_last_token_ms
        .expect("last token should be recorded");
    assert!(last >= first);
    state.complete_request_timing();
    assert_eq!(state.last_request_first_token_ms, Some(first));
    assert_eq!(state.last_request_last_token_ms, Some(last));
    assert!(state.processing_started_at.is_none());
    assert!(state.current_request_first_token_ms.is_none());
    assert!(state.current_request_last_token_ms.is_none());
}
