//! Modifier shortcut selection and status updates.
//!
//! [dispatch] gives scroll and approval handlers priority, then selects UI
//! effects through [Host]. Native code binds [Action] to concrete operations.
mod action;
mod binding;
mod host;
mod run;
pub use action::Action;
pub use host::{Effects, Host, Routing, State};
pub use run::dispatch;
/// Chat-only scroll shortcut selection.
pub mod scroll;
