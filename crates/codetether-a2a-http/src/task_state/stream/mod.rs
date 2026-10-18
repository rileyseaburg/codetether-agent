//! Task records and terminal outcomes for message/stream requests.
//!
//! [open] and [text] prepare accepted requests. [complete], [failure] and
//! [start_failure] update the shared task table and record telemetry; the
//! application owns session construction and prompt execution.
use codetether_a2a_core::*;
use codetether_telemetry::a2a_recording::record_a2a_message_telemetry;
use dashmap::DashMap;
use std::time::Instant;
use uuid::Uuid;
mod artifact;
/// Successful terminal task updates and completion observations.
pub mod complete;
mod completion_events;
/// Failed execution updates and observations.
pub mod failure;
/// Initial working-task construction.
pub mod open;
mod reply;
/// Failed session-construction updates and observations.
pub mod start_failure;
/// Text extraction from stream requests.
pub mod text;

/// Task activity and bus publication for application-mapped stream events.
pub mod events;

/// Ordered execution and settlement over application session hooks.
pub mod driver;

/// Provider initialization and task-state failure handling.
pub mod registry;
