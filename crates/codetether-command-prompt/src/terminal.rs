//! Raw-terminal lifetime for one mux control prompt read.

use anyhow::Result;
use crossterm::{event::EnableBracketedPaste, execute};
use std::io;

pub(super) struct Guard;

impl Guard {
    pub(super) fn enter() -> Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnableBracketedPaste)?;
        Ok(Self)
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), crossterm::event::DisableBracketedPaste);
        let _ = crossterm::terminal::disable_raw_mode();
    }
}
