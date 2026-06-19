//! Runtime control handle for a live swarm: cancel and pause/resume.
//!
//! A [`SwarmControl`] is cloneable and shared between the running executor and
//! whoever launched it (TUI, CLI, agent). Cancellation propagates to detached
//! sub-agents because the executor checks the token at stage boundaries and in
//! its `tokio::select!` loop; pause holds new stages from starting.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::swarm::SwarmControl;
//!
//! let ctl = SwarmControl::new();
//! assert!(!ctl.is_cancelled());
//! ctl.pause();
//! assert!(ctl.is_paused());
//! ctl.resume();
//! assert!(!ctl.is_paused());
//! ctl.cancel();
//! assert!(ctl.is_cancelled());
//! ```
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio_util::sync::CancellationToken;

/// Shared cancel + pause control for a running swarm.
#[derive(Clone, Debug, Default)]
pub struct SwarmControl {
    token: CancellationToken,
    paused: Arc<AtomicBool>,
}

impl SwarmControl {
    /// Create a fresh, un-cancelled, un-paused control handle.
    pub fn new() -> Self {
        Self::default()
    }

    /// Request cancellation of the whole swarm.
    pub fn cancel(&self) {
        self.token.cancel();
    }

    /// Whether cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.token.is_cancelled()
    }

    /// Future that resolves once cancellation is requested.
    pub async fn cancelled(&self) {
        self.token.cancelled().await;
    }

    /// Hold new stages from starting.
    pub fn pause(&self) {
        self.paused.store(true, Ordering::SeqCst);
    }

    /// Resume a paused swarm.
    pub fn resume(&self) {
        self.paused.store(false, Ordering::SeqCst);
    }

    /// Whether the swarm is currently paused.
    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::SeqCst)
    }
}
