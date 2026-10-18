//! Size-checked terminal rendering with one recovery attempt.
use crate::draw_recover::draw_or_recover;
use crate::draw_size::safe_size;
use ratatui::{Frame, Terminal, backend::CrosstermBackend};

/// Draw one TUI frame, recovering from transient failures.
///
/// The function reads the terminal size, rejects unsafe dimensions, and
/// delegates to the panic-guarded renderer. Size-query errors, draw errors,
/// and render-closure panics are all logged and swallowed so the event loop
/// continues after clearing and retrying the interrupted frame once.
///
/// # Returns
///
/// `Ok(true)` only when the initial draw or recovery draw rendered a frame;
/// `Ok(false)` when a frame was skipped or recovery failed.
///
/// # Side Effects
///
/// Writes a frame to stdout on success. Emits `tracing` warnings/errors on
/// any recoverable failure.
///
/// # Arguments
/// * `terminal` — Output terminal with retained rendering state.
/// * `render` — Callback that draws the initial frame and any recovery frame.
///
/// # Errors
/// Ordinary size/draw failures are logged and represented by Ok(false).
///
/// # Examples
/// ```no_run
/// # fn example(terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>) {
/// let _rendered = codetether_terminal_runtime::draw_ui(terminal, |_frame| {}).unwrap();
/// # }
/// ```
pub fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    mut render: impl FnMut(&mut Frame),
) -> anyhow::Result<bool> {
    let size = match terminal.size() {
        Ok(s) => s,
        Err(err) => {
            tracing::warn!(%err, "terminal size query failed — skipping frame");
            return Ok(false);
        }
    };
    if !safe_size(size) {
        crate::draw_log::log_skipped_size(size);
        return Ok(false);
    }
    let rendered = match draw_or_recover(terminal, &mut render) {
        Ok(()) => true,
        Err(err) => crate::draw_resync::clear_and_retry(terminal, &mut render, err),
    };
    Ok(rendered)
}
