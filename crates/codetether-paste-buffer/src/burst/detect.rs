use super::View;
use super::lines::{current_line_len, has_prior_newline};
use std::time::Duration;

const FAST_ENTER_GAP: Duration = Duration::from_millis(80);

const SLOW_ENTER_GAP: Duration = Duration::from_millis(300);

const MIN_LINE_CHARS: usize = 1;
/// Detect whether Enter belongs to a recent multiline paste.
/// # Arguments
/// * `app` - Borrowed chat input and key timing.
/// # Returns
/// Whether to insert a newline instead of submitting the input.
/// # Examples
/// ```text
/// burst::enter_is_likely_paste_newline(&view);
/// ```

pub fn enter_is_likely_paste_newline(app: &View<'_>) -> bool {
    enter_is_likely(app, slow_terminal())
}
/// Detect whether Enter belongs to a recent multiline paste.
/// # Arguments
/// * `app` - Borrowed chat input and key timing.
/// * `allow_slow_first_line` - Whether a slow first line may count as paste.
/// # Returns
/// Whether to insert a newline instead of submitting the input.
/// # Examples
/// ```text
/// burst::enter_is_likely(&view, false);
/// ```

pub fn enter_is_likely(app: &View<'_>, allow_slow_first_line: bool) -> bool {
    app.in_chat
        && recent_key(app, allow_slow_first_line)
        && current_line_len(app.input, app.cursor) >= MIN_LINE_CHARS
}

fn recent_key(app: &View<'_>, allow_slow_first_line: bool) -> bool {
    let Some(elapsed) = app.last_key_at.map(|t| t.elapsed()) else {
        return false;
    };
    elapsed <= FAST_ENTER_GAP
        || (elapsed <= SLOW_ENTER_GAP && slow_paste_context(app, allow_slow_first_line))
}

fn slow_paste_context(app: &View<'_>, allow_slow_first_line: bool) -> bool {
    allow_slow_first_line || has_prior_newline(app.input, app.cursor)
}

fn slow_terminal() -> bool {
    cfg!(windows) || std::env::var_os("WT_SESSION").is_some()
}
