//! Derive the per-step LLM context from an append-only chat history.
//!
//! See [`derive_context`] and [`derive_with_policy`] for entry points.

mod compress_step;
mod derive;
mod helpers;
mod policy;
mod reset;
mod reset_helpers;
mod reset_rebuild;
mod reset_summary;

pub use self::derive::derive_context;
pub use self::helpers::DerivedContext;
pub use self::policy::derive_with_policy;
