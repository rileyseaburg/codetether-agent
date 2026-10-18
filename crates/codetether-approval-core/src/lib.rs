//! Approval records and their append-only authorization ledger.
//!
//! [`ApprovalStore`] records requests and decisions, then verifies that a
//! receipt authorizes the exact tool, action, and resource requested. Shared
//! process-local registries own pending decisions and session grants; callers
//! deliver requests to interactive sessions.
//!
//! # Examples
//! ```
//! use codetether_approval_core::ApprovalStore;
//! let directory = tempfile::tempdir().unwrap();
//! let store = ApprovalStore::open(directory.path()).unwrap();
//! let request = store.create_request("write", "update", "notes.md", "save notes").unwrap();
//! store.approve(&request.id, "user", "reviewed").unwrap();
//! assert!(store.verify(&request.id, "write", "update", "notes.md").is_ok());
//! assert!(store.verify(&request.id, "write", "update", "other.md").is_err());
//! ```

mod decision;
mod event;
mod receipt;
mod request;
mod status;
mod store;
mod store_create;
mod store_decide;
mod store_events;
mod store_lookup;
mod store_verify;

pub use decision::ApprovalDecision;
pub use event::ApprovalEvent;
pub use receipt::ApprovalReceipt;
pub use request::ApprovalRequest;
pub use status::ApprovalStatus;
pub use store::ApprovalStore;

#[cfg(test)]
mod tests;

/// Advisory review contracts and prompt/output conversion.
pub mod review;

/// Shared records for interactive approval clients and execution adapters.
pub mod interaction;

/// Process-local grants for explicitly approved command prefixes.
pub mod session_command_grants;
/// Process-local grants for exact tool/action/resource approval tuples.
pub mod session_grants;

/// Explicit approval command parsing, recording and result text.
pub mod command;
