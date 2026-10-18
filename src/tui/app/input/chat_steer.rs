//! Bind shared active-turn steering to native UI and runtime state.
mod host;
use crate::tui::app::{session_runtime::TuiSessionHandle, state::App};
pub(super) fn submit(app: &mut App, runtime: &TuiSessionHandle, prompt: &str) {
    codetether_chat_submit::steering::submit(&mut host::Native { app, runtime }, prompt);
}
