//! # Embedded editor view
//!
//! Provides an in-TUI text editing surface. The editing engine is abstracted
//! behind the [`EditorBackend`] trait so the renderer never depends on a
//! concrete editor implementation.
//!
//! Two backends are available:
//!
//! * [`builtin::BuiltinBackend`] — a dependency-free rope-less buffer used by
//!   default. Always compiled.
//! * `helix` backend — embeds [`helix-core`](https://github.com/helix-editor/helix)
//!   (rope + tree-sitter syntax + transactions). Compiled only when the
//!   `helix` cargo feature is enabled, because it raises the crate MSRV to
//!   rustc 1.90 and pulls MPL-2.0 dependencies.
//!
//! The split keeps the heavy, license- and toolchain-sensitive Helix
//! dependency fully optional while the ratatui renderer and view plumbing
//! stay on the default build.

/// Dependency-free editor backend used when `helix` is not enabled.
pub mod builtin;
/// The backend abstraction consumed by the renderer.
pub mod backend;

pub use backend::{EditorBackend, EditorCell, EditorLine};
