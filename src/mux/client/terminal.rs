//! Local terminal state guard while proxying a remote PTY.

mod cleanup;
mod synchronized;

use std::io;

use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, enable_raw_mode},
};

use crate::mux::pty::terminal_mode::TerminalMode;

use synchronized::SynchronizedOutput;

pub(super) struct ProxyTerminal {
    mode: TerminalMode,
    clear_on_drop: bool,
    synchronized: SynchronizedOutput,
}

impl ProxyTerminal {
    pub(super) fn enter(alternate_screen: bool, replaying: bool) -> Result<Self> {
        enable_raw_mode()?;
        if alternate_screen {
            execute!(io::stdout(), EnterAlternateScreen)?;
        }
        let synchronized = SynchronizedOutput::begin(replaying)?;
        if replaying {
            execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        }
        Ok(Self {
            mode: TerminalMode::new(alternate_screen),
            clear_on_drop: false,
            synchronized,
        })
    }

    pub(super) fn observe_output(&mut self, data: &[u8]) {
        self.mode.observe(data);
    }

    pub(super) fn finish_replay(&mut self) -> Result<()> {
        self.synchronized.finish()
    }

    pub(super) fn clear_after_detach(&mut self) {
        self.clear_on_drop = true;
    }
}
