//! Swarm execution state, event reduction and terminal presentation.
//!
//! [SwarmViewState] owns cached subtasks, selection, the event receiver and
//! shared control handle. Attach a run, drain events, then read progress or
//! navigate its task details. Events and tasks use the shared swarm schema.
//! [render_swarm_view] draws the overview or selected-agent details.
pub use codetether_swarm_core::{SubTaskInfo, SwarmEvent};
mod agent_activity;
mod agent_complete;
mod agent_results;
mod agent_start;
mod attach;
mod detail_scroll;
mod event_rx;
mod events;
mod lifecycle;
mod navigation;
mod progress;
mod run_progress;
mod run_start;
mod state;
pub use state::SwarmViewState;

mod view;
pub use codetether_agent_core::events::{AgentMessageEntry, AgentToolCallDetail};
pub use view::{render_swarm_view, swarm_view_fmt};

mod transcript;
mod transcript_role;
pub use transcript::active_messages;

/// Interactive swarm commands with application-provided execution and UI.
pub mod interactive;
