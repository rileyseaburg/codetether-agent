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
