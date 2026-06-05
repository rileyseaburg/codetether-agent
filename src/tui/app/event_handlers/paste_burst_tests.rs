//! Tests for fallback paste-burst detection.

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use crate::tui::app::event_handlers::paste_burst::{
        current_line_len, enter_is_likely, enter_is_likely_paste_newline,
    };
    use crate::tui::app::state::App;
    use crate::tui::models::ViewMode;

    fn app_with_input(input: &str, age_ms: u64) -> App {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.input = input.to_string();
        app.state.input_cursor = input.chars().count();
        app.state.last_key_at = Some(Instant::now() - Duration::from_millis(age_ms));
        app
    }

    #[test]
    fn delayed_linux_first_line_enter_submits() {
        let app = app_with_input("ab", 180);

        assert!(!enter_is_likely_paste_newline(&app));
    }

    #[test]
    fn delayed_multiline_followup_enter_stays_in_paste() {
        let app = app_with_input("a\nb", 180);

        assert!(enter_is_likely_paste_newline(&app));
    }

    #[test]
    fn slow_terminal_first_line_enter_stays_in_paste() {
        let app = app_with_input("ab", 180);

        assert!(enter_is_likely(&app, true));
    }

    #[test]
    fn single_char_enter_still_submits() {
        let app = app_with_input("x", 180);

        assert!(!enter_is_likely_paste_newline(&app));
    }

    #[test]
    fn current_line_len_stops_at_newline() {
        assert_eq!(current_line_len("one\ntwo", 7), 3);
    }
}
