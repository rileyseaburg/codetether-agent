//! Ordered key-event admission, modal interception and shortcut dispatch.
//!
//! [dispatch] preserves the priority of native [Layers], [Review] and [Dispatch]
//! handlers. [dispatchable] filters key releases and unwanted repeated keys.
mod binding;
mod host;
mod repeat;
mod run;
pub use host::{Dispatch, Host, Layers, Review};
pub use repeat::dispatchable;
pub use run::dispatch;
/// Interrupt and modal handler priority.
pub mod priority;

/// Active-turn interrupt and idle-exit key policy.
pub mod interrupt;
