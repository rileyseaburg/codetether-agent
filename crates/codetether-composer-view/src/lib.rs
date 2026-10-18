//! Chat composer textbox, cursor, attachment labels and autocomplete display.
//!
//! Hosts provide a borrowed [ComposerView] to [render_input]. Editing and
//! command selection are coordinated through [interactive].
mod attachment;
mod cursor;
mod input_area;
mod input_title;
mod suggestion_hint;
mod suggestion_style;
mod suggestions;
mod view;
pub use attachment::attachment_suffix;
pub use cursor::place_cursor;
pub use input_area::render_input;
pub use input_title::build as build_title;
pub use suggestion_style::highlight_style;
pub use suggestions::render_suggestions;
pub use view::ComposerView;

/// Unicode-aware composer cursor indexing.
pub mod cursor_index;
/// Character and endpoint cursor movement.
pub mod cursor_move;
/// Horizontal cursor visibility.
pub mod cursor_visibility;
/// Word movement to the left.
pub mod cursor_word_left;
/// Word movement to the right.
pub mod cursor_word_right;

mod edit_clear;
mod edit_delete;
mod edit_insert;
mod edit_replace;
mod input_edit;
pub use input_edit::InputEdit;

mod command_history;
mod history_next;
mod history_previous;
mod history_push;
pub use command_history::{CommandHistory, HistoryRecall};

/// Coordinated edit, history and completion state transitions.
pub mod interactive;
