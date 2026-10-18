//! Agent dashboard rows and transcript presentation.
//!
//! [managed] and [tool_row] format child identities; [swarm] formats swarm
//! workers. [metadata], [messages] and [live] compose detail-pane rows from
//! existing shared snapshots, without reading native application state.
pub mod live;
pub mod managed;
mod message_label;
pub mod messages;
pub mod metadata;
pub mod swarm;
pub mod swarm_row;
pub mod tool_row;

mod child;
pub use child::Child;
/// Unified dashboard row composition.
pub mod dashboard;
/// Managed child transcript composition.
pub mod managed_detail;
mod managed_section;
/// Dashboard and detail panel widgets.
pub mod render;

/// Focused-child detail selection.
pub mod detail;
pub mod selection;
/// Parent-scoped tool-child transcript assembly.
pub mod tool_detail;
/// Dashboard and detail-panel selection.
pub mod view;

/// Display projection of generic spawned-agent metadata.
pub mod spawned;

/// Unified main, local, tool and swarm header tabs.
pub mod bar;

/// Dashboard Enter-key navigation and stale-selection policy.
pub mod interaction;
