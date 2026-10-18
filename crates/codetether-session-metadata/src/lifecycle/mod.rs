//! Initial session identity and provenance lifecycle operations.
//!
//! [InitialSession] creates a UUID, timestamps and initial metadata. Mutation
//! helpers keep the session persona and existing execution provenance aligned
//! when agents, worker tasks and accepted claims are attached.
mod initial;
mod provenance;
pub use initial::InitialSession;
pub use provenance::{attach_claim, attach_worker_task, set_agent_name};

mod hydrate;
pub use hydrate::hydrate;
