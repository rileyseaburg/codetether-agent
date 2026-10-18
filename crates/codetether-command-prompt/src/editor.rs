//! Keyboard event loop for the mux control prompt.

use std::path::Path;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};

use super::editor_helpers::{complete, done};
use super::state::State;

pub(super) fn read(workspace: &Path) -> Result<Option<String>> {
    let _terminal = super::terminal::Guard::enter()?;
    let mut state = State::new();
    super::render::prompt(&state)?;
    loop {
        let Some(key) = super::input_event::read(&mut state)? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                return done(None);
            }
            KeyCode::Char('d')
                if key.modifiers.contains(KeyModifiers::CONTROL) && state.line().is_empty() =>
            {
                return done(None);
            }
            KeyCode::Char(value) => state.insert(value),
            KeyCode::Backspace => state.backspace(),
            KeyCode::Delete => state.delete(),
            KeyCode::Left => state.left(),
            KeyCode::Right => state.right(),
            KeyCode::Home => state.home(),
            KeyCode::End => state.end(),
            KeyCode::Tab => complete(&mut state, workspace)?,
            KeyCode::Enter => return done(Some(state.line())),
            _ => {}
        }
        super::render::prompt(&state)?;
    }
}
