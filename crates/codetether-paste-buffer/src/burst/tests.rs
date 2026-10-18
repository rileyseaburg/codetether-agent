//! Tests for fallback paste-burst detection.

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::super::{View, current_line_len, enter_is_likely, enter_is_likely_paste_newline};

    fn app_with_input(input: &str, age_ms: u64) -> View<'_> {
        View {
            in_chat: true,
            input,
            cursor: input.chars().count(),
            last_key_at: Some(Instant::now() - Duration::from_millis(age_ms)),
        }
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
