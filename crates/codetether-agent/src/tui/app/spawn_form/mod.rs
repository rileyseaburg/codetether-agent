//! Interactive spawn form: a modal overlay for creating sub-agents.
//!
//! Opened via `/spawn` (no arguments). Submitting the form
//! reuses the existing [`crate::tui::app::spawn_agent::handle_spawn_command`]
//! path so managed-agent dispatch is never duplicated.

pub mod keys;
pub mod render;
pub use codetether_spawn_form as state;
pub mod submit;

pub(crate) use keys::handle_spawn_form_key;
pub use render::render_spawn_form_if_needed;
pub use state::{SpawnField, SpawnFormState};
pub use submit::submit_spawn_form;

/// Open the spawn form modal, resetting all fields.
pub fn open_spawn_form(app: &mut crate::tui::app::state::App) {
    codetether_spawn_form::lifecycle::open(&mut app.state.spawn_form, &mut app.state.status);
}

/// Close the spawn form modal without creating an agent.
pub fn close_spawn_form(app: &mut crate::tui::app::state::App) {
    codetether_spawn_form::lifecycle::close(&mut app.state.spawn_form, &mut app.state.status);
}
