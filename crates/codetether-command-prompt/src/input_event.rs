//! Terminal event translation for the mux control prompt.

use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent};

use super::state::State;

pub(super) fn read(state: &mut State) -> Result<Option<KeyEvent>> {
    match event::read()? {
        Event::Paste(text) => {
            insert_paste(state, &text)?;
            Ok(None)
        }
        Event::Key(key) => Ok(Some(key)),
        _ => Ok(None),
    }
}

fn insert_paste(state: &mut State, text: &str) -> Result<()> {
    for value in super::paste::single_line(text).chars() {
        state.insert(value);
    }
    super::render::prompt(state)
}
