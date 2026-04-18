use std::io;

use crossterm::{
    cursor::Show,
    event::{DisableBracketedPaste, DisableMouseCapture},
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};

pub(super) fn restore_terminal_state() {
    let _ = disable_raw_mode();
    let mut stdout = io::stdout();
    let _ = execute!(
        stdout,
        Show,
        LeaveAlternateScreen,
        DisableMouseCapture,
        DisableBracketedPaste
    );
}

pub(super) struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        restore_terminal_state();
    }
}
