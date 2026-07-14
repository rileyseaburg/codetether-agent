//! Keyboard event loop for the mux control prompt.

use std::path::Path;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use super::state::State;

pub(super) fn read(workspace: &Path) -> Result<Option<String>> {
    let _terminal = super::terminal::Guard::enter()?;
    let mut state = State::new();
    super::render::prompt(&state)?;
    loop {
        let Event::Key(key) = event::read()? else {
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

fn complete(state: &mut State, workspace: &Path) -> Result<()> {
    let result = super::completion::directory(&state.line(), workspace)?;
    state.replace(result.line);
    super::render::matches(&result.matches)
}

fn done(value: Option<String>) -> Result<Option<String>> {
    print!("\r\n");
    Ok(value)
}
