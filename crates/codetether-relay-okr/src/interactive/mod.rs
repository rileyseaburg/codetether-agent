//! Interactive goal command preparation and approval drafting.
//!
//! Implement [`DraftHost`] for provider lookup and [`CommandHost`] for session
//! and view bindings. [`execute`] rotates the model and stages a proposed OKR.
mod command;
mod draft;
mod host;
mod propose;
mod repository;
pub use command::execute;
pub use host::{CommandHost, DraftHost};
pub use propose::propose;
pub use repository::ensure_repository;

/// Pending-objective approval and relay startup workflow.
pub mod approval;
