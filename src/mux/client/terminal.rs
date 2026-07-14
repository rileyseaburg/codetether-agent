//! Local terminal state guard while proxying a remote PTY.

use std::io;

use anyhow::Result;
use crossterm::{
    cursor::Show,
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

pub(super) struct ProxyTerminal;

impl ProxyTerminal {
    pub(super) fn enter() -> Result<Self> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for ProxyTerminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
    }
}
