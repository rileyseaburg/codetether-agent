//! Event forwarding for streamed task activity.
//!
//! [spawn] drains an application-owned event channel, applies the supplied
//! JSON mapping, records task activity and publishes accepted events to the bus.
use crate::task_state::activity;
use codetether_a2a_core::{Task, TaskState};
use codetether_bus_core::{AgentBus, BusMessage};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
mod forward;
mod spawn;
use forward::forward;
pub use spawn::spawn;
