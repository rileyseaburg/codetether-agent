//! Relay team preparation, session turns, event forwarding and convergence.
//!
//! The group owns its application execution flow; shared policy and scheduling
//! remain in ralph-core. Runtime resources retain their existing lifetimes.
use super::*;
mod relay_announce;
mod relay_coordinator;
mod relay_events;
mod relay_finish;
mod relay_prepare;
mod relay_profiles;
mod relay_progress;
mod relay_rounds;
mod relay_sender;
mod relay_session;
mod relay_text;
mod relay_turn;
mod relay_types;
