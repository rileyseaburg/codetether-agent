//! Restores local terminal state when a proxy session ends.

use std::io;

use crossterm::{
    cursor::{MoveTo, Show},
    execute,
    terminal::{Clear, ClearType, LeaveAlternateScreen, disable_raw_mode},
};

use super::ProxyTerminal;

impl Drop for ProxyTerminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        if self.mode.active() {
            let _ = execute!(io::stdout(), LeaveAlternateScreen);
        }
        if self.clear_on_drop {
            let _ = execute!(
                io::stdout(),
                Clear(ClearType::Purge),
                Clear(ClearType::All),
                MoveTo(0, 0)
            );
        }
        let _ = execute!(io::stdout(), Show);
    }
}
