//! System-wide, append-only audit trail.
//!
//! [`AuditEntry`] describes an event; [`AuditLog`] retains a bounded in-memory
//! history and optionally appends JSONL to disk. Persistence is best effort.
//! Initialize the optional process-wide log with [`init_audit_log`].
//!
//! # Examples
//! ```
//! use codetether_audit::{AuditCategory, AuditLog, AuditOutcome};
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! let log = AuditLog::default();
//! log.log(AuditCategory::Api, "GET /health", AuditOutcome::Success, None, None).await;
//! assert_eq!(log.count().await, 1);
//! # });
//! ```
//!
//! # Architecture
//! Event types define the wire format. The log module owns retention, recording,
//! and queries; the sink handles JSONL I/O; the global module owns initialization.

mod category;
mod entry;
mod global;
mod log;
mod outcome;
mod sink;

pub use category::AuditCategory;
pub use entry::AuditEntry;
pub use global::{audit_log, init_audit_log, try_audit_log};
pub use log::AuditLog;
pub use outcome::AuditOutcome;

#[cfg(test)]
mod tests;
