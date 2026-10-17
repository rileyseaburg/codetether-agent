//! Canonical live-agent storage and owner-scoped lookup.
//!
//! [AgentEntry] carries metadata and an application-provided [Session].
//! [Registry] owns the synchronized map, name matching, lineage, and scoped
//! resolution. Callers supply tree lookup and residency notifications so the
//! registry does not depend on application session persistence or execution.
mod crud;
mod entry;
mod names;
mod registry;
mod resolve;
mod scope;
pub use entry::{AgentEntry, Session};
pub use registry::Registry;
