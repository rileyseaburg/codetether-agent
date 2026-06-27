//! Interactive spawn form: a modal overlay for creating sub-agents.
//!
//! Opened via `/spawn` (no arguments). Submitting the form
//! reuses the existing [`crate::tui::app::spawn_agent::handle_spawn_command`]
//! path so spawn logic is never duplicated.

pub mod keys;
pub mod render;
pub mod state;
#[cfg(test)]
#[path = "state_tests.rs"]
mod state_tests;
pub mod submit;

pub use keys::handle_spawn_form_key;
pub use render::render_spawn_form_if_needed;
pub use state::{SpawnField, SpawnFormState};
pub use submit::submit_spawn_form;

/// Open the spawn form modal, resetting all fields.
pub fn open_spawn_form(app: &mut crate::tui::app::state::App) {
    app.state.spawn_form = Some(SpawnFormState::default());
    app.state.status = "Spawn agent form — Tab: next field, Enter: submit, Esc: cancel".to_string();
}

/// Close the spawn form modal without creating an agent.
pub fn close_spawn_form(app: &mut crate::tui::app::state::App) {
    app.state.spawn_form = None;
    app.state.status = "Spawn form cancelled".to_string();
}
