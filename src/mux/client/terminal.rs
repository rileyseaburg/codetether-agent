//! Local terminal state guard while proxying a remote PTY.

use std::io;

use anyhow::Result;
use crossterm::{
    cursor::{MoveTo, Show},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

use crate::mux::pty::terminal_mode::TerminalMode;

pub(super) struct ProxyTerminal {
    mode: TerminalMode,
    clear_on_drop: bool,
}

impl ProxyTerminal {
    pub(super) fn enter(alternate_screen: bool) -> Result<Self> {
        enable_raw_mode()?;
        if alternate_screen {
            execute!(io::stdout(), EnterAlternateScreen)?;
        }
        Ok(Self {
            mode: TerminalMode::new(alternate_screen),
            clear_on_drop: false,
        })
    }

    pub(super) fn observe_output(&mut self, data: &[u8]) {
        self.mode.observe(data);
    }

    pub(super) fn clear_after_detach(&mut self) {
        self.clear_on_drop = true;
    }
}

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
