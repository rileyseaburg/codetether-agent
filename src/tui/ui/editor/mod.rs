//! # Embedded editor view
//!
//! Provides an in-TUI text editing surface. The editing engine is abstracted
//! behind the [`EditorBackend`] trait so the renderer never depends on a
//! concrete editor implementation.
//!
//! Two backends are available:
//!
//! * [`builtin::BuiltinBackend`] — a dependency-free rope-less line buffer.
//! * [`helix_backend::HelixBackend`] — a [`ropey`] rope buffer (the same rope
//!   type Helix uses) with editing via the [`edit::EditorEdit`] trait.
//!
//! [`EditorBackend`] is the render seam (read); [`EditorEdit`] is the mutation
//! seam (write). Keeping them separate lets a backend be a pure viewer or a
//! full editor.

/// Applies editor actions to a file buffer.
pub mod apply;
/// The backend abstraction consumed by the renderer.
pub mod backend;
/// Dependency-free editor backend.
pub mod builtin;
/// Terminal cursor position computation.
pub mod cursor_pos;
/// Tests for cursor ↔ screen-cell mapping.
#[cfg(test)]
mod cursor_pos_tests;
/// Draws the editor view to a frame.
pub mod draw;
/// Mutating operations layered on the render backend.
pub mod edit;
/// File-backed editor document (open/save/dirty).
pub mod file_buffer;
/// Cursor accessors for the file buffer (LSP navigation).
mod file_buffer_cursor;
/// Helix-core (rope) powered editor backend.
pub mod helix_backend;
/// Cursor/char-index helpers for the rope backend.
mod helix_cursor;
/// `EditorEdit` implementation for the rope backend.
mod helix_edit;
/// Behavior tests for rope backend edit/navigation ops.
#[cfg(test)]
mod helix_edit_tests;
/// Rope-line to colored-cell conversion.
mod helix_render;
/// Rust syntax highlighting.
pub mod highlight;
/// Renders the LSP hover/JSDoc popup over the editor.
pub mod hover_popup;
/// Key-event to editor-action mapping.
pub mod input;
/// Tests for editing/navigation key mappings.
#[cfg(test)]
mod input_edit_tests;
/// Tests for the key-event to editor-action mapping.
#[cfg(test)]
mod input_tests;
/// Editor-side LSP state (shared manager + hover popup).
pub mod lsp_state;
/// Renders editor content to ratatui lines.
pub mod render;
/// Cursor-follow scroll offset computation.
pub mod scroll;

pub use backend::{EditorBackend, EditorCell, EditorLine};
pub use edit::{EditorEdit, Move};
pub use file_buffer::FileBuffer;
pub use input::{EditorInput, map_key};
pub use render::editor_lines;
