//! Swarm execution view state and event reduction.
//!
//! [SwarmViewState] owns cached subtasks, selection, the event receiver and
//! shared control handle. Attach a run, drain events, then read progress or
//! navigate its task details. Events and tasks use the shared swarm schema.
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
