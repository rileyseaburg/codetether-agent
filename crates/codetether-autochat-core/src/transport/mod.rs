//! Receive a text handoff for a specific relay correlation ID.
//!
//! Register each recipient with [attach_handoff_receiver], then await
//! [consume_handoff_by_correlation] using the published correlation ID.
use anyhow::{Context, Result, anyhow};
use codetether_a2a_core::Part;
use codetether_bus_core::{AgentBus, BusHandle, BusMessage};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{Duration, timeout};

const HANDOFF_TIMEOUT_SECS: u64 = 5;
mod attach_handoff_receiver;
pub use attach_handoff_receiver::attach_handoff_receiver;
mod consume_handoff_by_correlation;
pub use consume_handoff_by_correlation::consume_handoff_by_correlation;
mod first_text_part;
use first_text_part::first_text_part;
