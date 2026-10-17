//! Ralph learning, handoff and progress messages on a supplied shared bus.
//!
//! [collect] drains project learning context; [publish] emits story results.
//! The caller selects and retains the bus and decides when these operations run.
use crate::{RalphState, UserStory};
use codetether_bus_core::{AgentBus, BusMessage};
use std::sync::Arc;
mod collect;
mod publish;
pub use collect::collect;
pub use publish::publish;
