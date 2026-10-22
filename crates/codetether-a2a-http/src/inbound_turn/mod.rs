//! Inbound-turn routing and interactive task liveness.
//!
//! [execute_turn] prefers the shared interactive inbox when attached. Otherwise
//! it polls the headless future supplied by the application. Interactive waits
//! refresh task timestamps every twenty seconds while awaiting the answer.
use anyhow::Result;
use codetether_a2a_core::Task;
use codetether_a2a_inbox as live_inbox;
use dashmap::DashMap;
use std::{future::Future, time::Duration};
mod execute;
mod live;
mod reply_path;
pub use execute::execute_turn;
use live::live;
pub use reply_path::{PendingReply, begin_reply, is_waiting, prompt};

/// How often a live turn refreshes its task while the human's session works,
/// so a polling peer sees progress instead of an idle timeout.
const LIVE_HEARTBEAT: Duration = Duration::from_secs(20);
