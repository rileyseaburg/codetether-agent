//! Ralph execution events and story summaries shared with presentation layers.
//!
//! Producers send RalphEvent values. Consumers display RalphStoryInfo and its
//! RalphStoryStatus without introducing a dependency on terminal rendering.
use codetether_agent_core::events::{AgentMessageEntry, AgentToolCallDetail};
mod event;
mod status;
mod story;
pub use event::RalphEvent;
pub use status::RalphStoryStatus;
pub use story::RalphStoryInfo;

mod from_swarm;
mod initial_stories;
pub use from_swarm::from_swarm_event;
pub use initial_stories::initial_stories;
