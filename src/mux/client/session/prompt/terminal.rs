//! Raw-terminal lifetime for one mux control prompt read.

use anyhow::Result;

pub(super) struct Guard;

impl Guard {
    pub(super) fn enter() -> Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
    }
}
