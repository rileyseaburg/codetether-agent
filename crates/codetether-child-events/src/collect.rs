//! Streaming event collection for sub-agent replies.
//!
//! This module reads session events until completion or timeout and updates the
//! event-loop state incrementally.
//!
//! # Examples
//!
//! ```ignore
//! let state = collect_events(&mut rx).await;
//! ```

use super::state::EventLoopState;
use crate::Host;
use std::time::Duration;
use tokio::sync::mpsc;

const IDLE_TIMEOUT: Duration = Duration::from_secs(300);

/// Collects streamed session events until completion or timeout.
///
/// # Arguments
///
/// * `rx` - Channel carrying streamed sub-agent session events.
///
/// # Returns
///
/// The accumulated event-loop state after completion or inactivity.
///
/// # Examples
///
/// ```ignore
/// let state = collect_events("reviewer", &mut rx).await;
/// ```
pub async fn collect_events<H: Host>(
    agent_name: &str,
    rx: &mut mpsc::Receiver<H::SourceEvent>,
) -> EventLoopState {
    collect_events_with_idle_timeout::<H>(agent_name, rx, IDLE_TIMEOUT).await
}

/// Collect events while enforcing a caller-selected inactivity limit.
///
/// # Arguments
///
/// * `rx` - Channel carrying streamed sub-agent session events.
/// * `idle_timeout` - Maximum duration allowed between consecutive events.
///
/// # Returns
///
/// The accumulated event-loop state, marked timed out after inactivity.
///
/// # Examples
///
/// ```ignore
/// let state = collect_events_with_idle_timeout(
///     "reviewer", &mut rx, Duration::from_secs(30)
/// ).await;
/// ```
pub async fn collect_events_with_idle_timeout<H: Host>(
    agent_name: &str,
    rx: &mut mpsc::Receiver<H::SourceEvent>,
    idle_timeout: Duration,
) -> EventLoopState {
    let mut state = EventLoopState::default();
    while !state.done {
        match tokio::time::timeout(idle_timeout, rx.recv()).await {
            Ok(result) => observe_and_apply::<H>(agent_name, result, &mut state),
            Err(_) => mark_timed_out(&mut state),
        }
    }
    state
}

fn observe_and_apply<H: Host>(
    agent_name: &str,
    result: Option<H::SourceEvent>,
    state: &mut EventLoopState,
) {
    if let Some(event) = result.as_ref() {
        H::observe(agent_name, event);
    }
    super::handle::recv_result::<H>(result.map(H::project), state);
}

fn mark_timed_out(state: &mut EventLoopState) {
    state.error = Some("Agent timed out after 5 minutes of inactivity".into());
    state.done = true;
    state.timed_out = true;
}
