//! Shared execution control without frontend, provider, or tool dependencies.
//!
//! [`ControlPlane`] owns run reservations, executor claims, pause, cancellation, and
//! generation-checked cleanup. [`ActiveTurn`] retains cancellation across
//! frontend handoff. [`Inboxes`] accepts session-addressed input and atomically
//! closes an idle inbox at a completion boundary. Callers adapt their own
//! input types, durable goal gates, and transport resources.
//!
//! # Examples
//! ```
//! use codetether_session_runtime::{ActiveTurn, Inboxes};
//! let mut turn = ActiveTurn::default();
//! assert!(turn.prepare("session"));
//! let inboxes = Inboxes::default();
//! inboxes.open("session");
//! assert!(inboxes.push("session", "inspect the cloned repositories").is_ok());
//! assert_eq!(inboxes.take("session"), vec!["inspect the cloned repositories"]);
//! ```

mod control;
mod control_cleanup;
mod control_handoff;
mod control_pause;
mod control_query;
mod control_start;
mod control_stop;
mod control_subscribe;
mod execution;
mod inboxes;
mod inboxes_drain;
mod lease;
mod turn;
mod turn_cancel;

pub use control::ControlPlane;
pub use execution::Execution;
pub use inboxes::Inboxes;
pub use lease::Lease;
pub use turn::ActiveTurn;

#[cfg(test)]
mod control_cleanup_tests;
#[cfg(test)]
mod control_observer_tests;
#[cfg(test)]
mod control_tests;
#[cfg(test)]
mod inboxes_tests;
#[cfg(test)]
mod turn_tests;

/// Session checkout/restore ownership with retained frontend views.
pub mod slot;

/// Owned execution requests and runtime command transport data.
pub mod request;

/// Owned command submission, cancellation and active-turn steering handles.
pub mod handle;
