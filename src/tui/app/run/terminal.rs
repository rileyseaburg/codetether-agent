use crossterm::{
    event::{EnableBracketedPaste, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::tui::app::panic_cleanup::{PanicHookGuard, install_panic_cleanup_hook};
use crate::tui::app::terminal_state::{TerminalGuard, restore_terminal_state};

pub(super) struct Runtime {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    _terminal_guard: TerminalGuard,
    _panic_guard: PanicHookGuard,
}

pub(super) fn enter() -> anyhow::Result<Runtime> {
    restore_terminal_state();
    enable_raw_mode()?;
    let terminal_guard = TerminalGuard;
    let panic_guard = install_panic_cleanup_hook();
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableBracketedPaste)?;
    let _ = execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    );
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    terminal.clear()?;
    Ok(Runtime {
        terminal,
        _terminal_guard: terminal_guard,
        _panic_guard: panic_guard,
    })
}
