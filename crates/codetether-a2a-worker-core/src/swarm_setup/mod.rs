//! Swarm task settings independent of application sessions.
//!
//! [build_swarm_setup] parses worker metadata and combines it with a supplied
//! model future. [SwarmSetup] carries the resulting config and strategy.
use crate::task_config::{metadata_bool, metadata_u64, metadata_usize};
use codetether_swarm_core::SwarmConfig;
mod build;
mod record;
mod strategy;
pub use build::build_swarm_setup;
pub use record::SwarmSetup;
pub use strategy::parse_swarm_strategy;
