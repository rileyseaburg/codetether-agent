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

/// The backend abstraction consumed by the renderer.
pub mod backend;
/// Mutating operations layered on the render backend.
pub mod edit;
/// Dependency-free editor backend.
pub mod builtin;
/// Helix-core (rope) powered editor backend.
pub mod helix_backend;
/// Cursor/char-index helpers for the rope backend.
mod helix_cursor;
/// `EditorEdit` implementation for the rope backend.
mod helix_edit;

pub use backend::{EditorBackend, EditorCell, EditorLine};
pub use edit::{EditorEdit, Move};