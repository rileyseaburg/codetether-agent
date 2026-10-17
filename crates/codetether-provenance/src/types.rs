//! Execution identity, origin, construction, and lifecycle updates.

mod construct;
mod identity;
mod origin;
mod record;
mod update;
pub use identity::AgentIdentity;
pub use origin::ExecutionOrigin;
pub use record::ExecutionProvenance;
