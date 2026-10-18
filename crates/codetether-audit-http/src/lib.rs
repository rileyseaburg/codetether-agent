//! HTTP request auditing and audit-history query behavior.
//!
//! Call audit_middleware with a shared log around a request handler.
//! list_audit_entries provides bounded history and category filtering.
pub mod http;
mod middleware;
mod query;
mod types;
pub use middleware::audit_middleware;
pub use query::list_audit_entries;
pub use types::AuditQuery;
