//! Fallback detection for terminals that replay paste as key events.

use std::time::Duration;

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

const FAST_ENTER_GAP: Duration = Duration::from_millis(80);
const SLOW_ENTER_GAP: Duration = Duration::from_millis(300);
const MIN_LINE_CHARS: usize = 1;

pub(super) fn enter_is_likely_paste_newline(app: &App) -> bool {
    enter_is_likely(app, slow_terminal())
}

pub(super) fn enter_is_likely(app: &App, allow_slow_first_line: bool) -> bool {
    app.state.view_mode == ViewMode::Chat
        && recent_key(app, allow_slow_first_line)
        && current_line_len(&app.state.input, app.state.input_cursor) >= MIN_LINE_CHARS
}

fn recent_key(app: &App, allow_slow_first_line: bool) -> bool {
    let Some(elapsed) = app.state.last_key_at.map(|t| t.elapsed()) else {
        return false;
    };
    elapsed <= FAST_ENTER_GAP
        || (elapsed <= SLOW_ENTER_GAP && slow_paste_context(app, allow_slow_first_line))
}

fn slow_paste_context(app: &App, allow_slow_first_line: bool) -> bool {
    allow_slow_first_line || has_prior_newline(&app.state.input, app.state.input_cursor)
}

fn slow_terminal() -> bool {
    cfg!(windows) || std::env::var_os("WT_SESSION").is_some()
}

fn has_prior_newline(input: &str, cursor: usize) -> bool {
    input.chars().take(cursor).any(|c| c == '\n')
}

pub(super) fn current_line_len(input: &str, cursor: usize) -> usize {
    let mut len = 0;
    for c in input.chars().take(cursor) {
        if c == '\n' {
            len = 0;
        } else {
            len += 1;
        }
    }
    len
}
