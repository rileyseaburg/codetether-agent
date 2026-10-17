//! Searchable turn files rendered from the canonical transcript.
//!
//! [materialize_history] writes turn text at stable paths and removes stale
//! regular files from the selected session directory. [render_turn] and
//! [format_turn_path] also support bounded history browsing.
mod materialize;
mod paths;
mod prune;
mod render;
mod role;
mod write;
pub use materialize::materialize_history;
pub use paths::{format_turn_path, history_dir};
pub use render::render_turn;
pub use role::role_label;
