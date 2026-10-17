//! HTTP discovery of explicitly configured peer seeds.
//!
//! Polling uses ordered candidate probes and endpoint-only deduplication.
//! The public loop takes application-selected seeds and a ledger-path resolver.
use super::*;
use crate::peer_probe::try_fetch_agent_card;
use codetether_a2a_core::AgentCard;
use codetether_a2a_discovery::endpoint::peer_candidates;
use std::time::Duration;
mod probe;
mod register;
mod run;
use probe::probe;
use register::register;
pub use run::discovery_loop;
