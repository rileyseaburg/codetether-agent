//! Completion and submission actions for the terminal editor.
use super::state::State;
use anyhow::Result;
use std::path::Path;

pub(super) fn complete(state: &mut State, workspace: &Path) -> Result<()> {
    let result = super::completion::directory(&state.line(), workspace)?;
    state.replace(result.line);
    super::render::matches(&result.matches)
}

pub(super) fn done(value: Option<String>) -> Result<Option<String>> {
    print!("\r\n");
    Ok(value)
}
