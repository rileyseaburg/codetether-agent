//! Lifecycle status for swarm actors.

/// Status of an actor in the swarm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorStatus {
    /// Actor is initializing
    Initializing,
    /// Actor is ready to process messages
    Ready,
    /// Actor is currently processing
    Busy,
    /// Actor is paused
    Paused,
    /// Actor is shutting down
    ShuttingDown,
    /// Actor has stopped
    Stopped,
}

impl std::fmt::Display for ActorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorStatus::Initializing => write!(f, "initializing"),
            ActorStatus::Ready => write!(f, "ready"),
            ActorStatus::Busy => write!(f, "busy"),
            ActorStatus::Paused => write!(f, "paused"),
            ActorStatus::ShuttingDown => write!(f, "shutting_down"),
            ActorStatus::Stopped => write!(f, "stopped"),
        }
    }
}
