//! Durable prior-context policy and enforcement without native Session ownership.
//!
//! [allowed] resolves inherited, turn and persistent state. [trusted] and
//! [delegated] apply authoritative or restricted updates to shared metadata.
//! [runtime_context] and [serialized_policy] reject prohibited tool calls.
mod blocking;
mod call;
mod delegation;
mod prepare;
mod registry;
mod state;
mod update;
pub use blocking::{for_messages, runtime_context, serialized_messages, serialized_policy};
pub use call::{PRIOR_CONTEXT_TOOLS, requests_prior_context};
pub use registry::{available, remove_tools};
pub use state::allowed;
pub use update::{delegated, trusted};

/// Application-local message ingress and policy access.
pub mod session_binding;
