//! Enter interactive terminal mode and install cleanup guards.
use crate::clear::clear_screen;
use crate::{
    Runtime, TerminalGuard, enter_display_mode, install_panic_cleanup_hook, restore_terminal_state,
};
use crossterm::{
    event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::enable_raw_mode,
};
use ratatui::{Terminal, backend::CrosstermBackend};

/// Configure the terminal and install lifetime guards.
///
/// # Returns
/// An owned terminal runtime to retain while drawing the interactive UI.
///
/// # Errors
/// Returns terminal I/O errors from raw mode, display setup or backend creation.
///
/// # Examples
/// ```no_run
/// let _runtime = codetether_terminal_runtime::enter().expect("terminal setup");
/// ```
pub fn enter() -> anyhow::Result<Runtime> {
    restore_terminal_state();
    enable_raw_mode()?;
    codetether_worktree::set_tui_active(true);
    let terminal_guard = TerminalGuard;
    let panic_guard = install_panic_cleanup_hook();
    let mut stdout = std::io::stdout();
    enter_display_mode(&mut stdout)?;
    clear_screen(&mut stdout)?;
    let _ = execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    );
    let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    Ok(Runtime {
        terminal,
        _terminal_guard: terminal_guard,
        _panic_guard: panic_guard,
    })
}
