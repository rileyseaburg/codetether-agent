//! Complete or fail an inbound task and record its outcome.
//!
//! [Settle] groups the task table, response context and timing. Its completion
//! methods update the table before recording best-effort telemetry.
use super::{complete, fail};
use codetether_a2a_core::{Message, Task};
use codetether_telemetry::a2a_recording::record_a2a_message_telemetry;
use dashmap::DashMap;
use std::time::Duration;
mod outcome;
mod record;
mod telemetry;
pub use record::Settle;
