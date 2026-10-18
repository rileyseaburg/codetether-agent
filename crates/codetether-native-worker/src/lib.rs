//! Native worker registration and terminal bridge composition.
//!
//! [register_worker] submits the shared A2A payload using cached native provider
//! discovery. [bridge::TuiWorkerBridge] uses that same registration function
//! when the terminal starts a worker.
mod registration;
pub use registration::register_worker;
pub mod bridge;

/// Shared default models and temperature policy for worker execution.
pub mod model_policy;
