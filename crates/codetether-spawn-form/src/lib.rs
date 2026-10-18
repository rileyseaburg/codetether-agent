//! Editable agent-spawn form and terminal overlay.
//!
//! Create a SpawnFormState, pass keys to its handle_key method, then handle
//! returned FormAction values in the host. Render the optional state using
//! render_spawn_form_if_needed and obtain command arguments using to_args.
mod action;
mod arguments;
mod edit;
mod field;
mod focus;
pub mod key_dispatch;
mod keys;
mod render;
mod row;
mod state;
mod widget;
pub use action::FormAction;
pub use field::SpawnField;
pub use render::render_spawn_form_if_needed;
pub use state::SpawnFormState;
#[cfg(test)]
mod tests;

pub mod lifecycle;
pub mod submission;
