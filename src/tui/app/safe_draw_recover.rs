//! Panic recovery wrapper for terminal rendering.
//!
//! Long-running TUI sessions accumulate diverse message content (tool outputs,
//! compressed context markers, multi-byte text). A single unexpected edge case
//! inside the render closure would panic and kill the process. This module
//! catches that panic, logs a diagnostic, and returns `Ok(())` so the event
//! loop keeps running — the next frame can retry with fresh state.

use std::panic::{AssertUnwindSafe, catch_unwind};

use ratatui::{Frame, Terminal, backend::CrosstermBackend};

use crate::tui::app::session_runtime::SessionView;
use crate::tui::app::state::App;

/// Draw a frame, catching any panic inside the render closure.
///
/// Returns `Ok(())` on success or when a panic was caught (logged at `error`
/// level). Returns `Err` only for non-panic I/O failures from the terminal
/// backend, which the caller may also downgrade to non-fatal.
pub(super) fn draw_or_recover(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    session: &SessionView,
) -> anyhow::Result<()> {
    let result: Result<Result<(), std::io::Error>, Box<dyn std::any::Any + Send>> =
        catch_unwind(AssertUnwindSafe(|| {
            match terminal.draw(|f: &mut Frame| ui_render(f, app, session)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }));
    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(io_err)) => Err(anyhow::anyhow!(io_err)),
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic_payload.downcast_ref::<&'static str>().copied())
                .unwrap_or("(non-string panic)");
            tracing::error!(panic = %msg, "render panic caught — frame skipped, TUI continues");
            Ok(())
        }
    }
}

/// Trampoline to the real UI renderer. Separate function so the closure passed
/// to `terminal.draw` is a simple function pointer — easier for the compiler
/// to optimise and keeps the unwind boundary explicit.
fn ui_render(f: &mut Frame, app: &mut App, session: &SessionView) {
    crate::tui::ui::main::ui(f, app, session);
}
