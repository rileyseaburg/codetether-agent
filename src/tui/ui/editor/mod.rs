//! # Embedded editor view
//!
//! Provides an in-TUI text editing surface. The editing engine is abstracted
//! behind the [`EditorBackend`] trait so the renderer never depends on a
//! concrete editor implementation.
//!
//! Two backends are available:
//!
//! * [`builtin::BuiltinBackend`] — a dependency-free rope-less line buffer.
//! * `helix` backend — embeds [`helix-core`](https://github.com/helix-editor/helix)
//!   (rope + tree-sitter syntax + transactions).
//!
//! The [`EditorBackend`] trait is the seam between the two: the ratatui
//! renderer consumes the trait and never references `helix-core` types
//! directly.

/// The backend abstraction consumed by the renderer.
pub mod backend;
/// Dependency-free editor backend.
pub mod builtin;

pub use backend::{EditorBackend, EditorCell, EditorLine};