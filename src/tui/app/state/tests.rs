//! State module tests covering cursor movement, scrolling, model picker, and timing.

use super::AppState;

#[test]
fn utf8_cursor_moves_by_character_not_byte() {
    let mut state = AppState::default();
    state.composer.input = "aé🙂z".to_string();
    state.composer.move_cursor_end();
    assert_eq!(state.composer.cursor, 4);
    state.composer.move_cursor_left();
    assert_eq!(state.composer.cursor, 3);
    state.composer.move_cursor_left();
    assert_eq!(state.composer.cursor, 2);
    state.composer.move_cursor_right();
    assert_eq!(state.composer.cursor, 3);
}

#[test]
fn utf8_backspace_removes_full_character() {
    let mut state = AppState::default();
    state.composer.input = "aé🙂z".to_string();
    state.composer.move_cursor_end();
    state.composer.move_cursor_left();
    state.composer.delete_backspace();
    assert_eq!(state.composer.input, "aéz");
    assert_eq!(state.composer.cursor, 2);
}

#[test]
fn utf8_delete_forward_removes_full_character() {
    let mut state = AppState::default();
    state.composer.input = "aé🙂z".to_string();
    state.composer.cursor = 1;
    state.composer.delete_forward();
    assert_eq!(state.composer.input, "a🙂z");
    assert_eq!(state.composer.cursor, 1);
}

#[test]
fn utf8_insert_respects_character_cursor() {
    let mut state = AppState::default();
    state.composer.input = "a🙂z".to_string();
    state.composer.cursor = 2;
    state.composer.insert_char('é');
    assert_eq!(state.composer.input, "a🙂éz");
    assert_eq!(state.composer.cursor, 3);
}

#[test]
fn model_filter_limits_visible_models() {
    let mut state = AppState::default();
    state.model_picker.available_models = vec![
        "zai/glm-5".to_string(),
        "openai/gpt-4o".to_string(),
        "openrouter/qwen/qwen3-coder".to_string(),
    ];
    state.model_picker.model_filter_push('g');
    state.model_picker.model_filter_push('p');
    state.model_picker.model_filter_push('t');
    assert_eq!(state.model_picker.filtered_models(), vec!["openai/gpt-4o"]);
    assert_eq!(state.model_picker.selected_model(), Some("openai/gpt-4o"));
}

#[test]
fn scroll_up_from_follow_latest_enters_manual_scroll_mode() {
    let mut state = AppState::default();
    state.chat_viewport.set_chat_max_scroll(25);
    state.chat_viewport.scroll_to_bottom();
    crate::tui::app::state::scroll::view(&mut state).scroll_up(1);
    assert_eq!(state.chat_viewport.chat_scroll, 24);
}

#[test]
fn scroll_down_returns_to_follow_latest_at_bottom() {
    let mut state = AppState::default();
    state.chat_viewport.set_chat_max_scroll(25);
    state.chat_viewport.chat_scroll = 24;
    state.chat_viewport.scroll_down(1);
    assert_eq!(state.chat_viewport.chat_scroll, 1_000_000);
}
