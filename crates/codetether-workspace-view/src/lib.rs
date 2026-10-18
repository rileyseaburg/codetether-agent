//! Terminal workspace snapshot panel.
//!
//! [render] displays captured entries and Git metadata without rescanning the
//! filesystem. Pass an existing [codetether_workspace_snapshot::WorkspaceSnapshot]
//! and the rectangle reserved for the panel.
mod panel;
pub use panel::render;
