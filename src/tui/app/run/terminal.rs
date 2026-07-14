use crossterm::{
    event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::enable_raw_mode,
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::tui::app::panic_cleanup::{PanicHookGuard, install_panic_cleanup_hook};
use crate::tui::app::terminal_state::{TerminalGuard, enter_display_mode, restore_terminal_state};

pub(super) struct Runtime {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    _terminal_guard: TerminalGuard,
    _panic_guard: PanicHookGuard,
}

pub(super) fn enter() -> anyhow::Result<Runtime> {
    restore_terminal_state();
    enable_raw_mode()?;
    crate::worktree::set_tui_active(true);
    let terminal_guard = TerminalGuard;
    let panic_guard = install_panic_cleanup_hook();
    let mut stdout = std::io::stdout();
    enter_display_mode(&mut stdout)?;
    let _ = execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    );
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    if let Err(error) = terminal.clear() {
        tracing::debug!(%error, "TUI startup clear skipped");
    }
    Ok(Runtime {
        terminal,
        _terminal_guard: terminal_guard,
        _panic_guard: panic_guard,
    })
}
