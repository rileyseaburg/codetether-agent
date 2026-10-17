//! Persona identity, lifecycle requests and resource-policy contracts.
//!
//! [PersonaIdentity] records lineage, [PersonaPolicy] supplies limits, and
//! [PersonaRuntimeState] records counters without owning runtime tasks.
mod identity;
mod policy;
mod request;
mod state;
pub use identity::{PersonaIdentity, PersonaStatus};
pub use policy::PersonaPolicy;
pub use request::{CreatePersonaRequest, ReapPersonaRequest, SpawnPersonaRequest};
pub use state::PersonaRuntimeState;

/// Spawn-depth constraints.
pub mod depth;
/// Parent inheritance and branching constraints.
pub mod lineage;
/// Descendant collection for cascading reaps.
pub mod reap_targets;

/// Shared asynchronous budget accounting.
pub mod accounting;
/// Per-tick resource and idle policy.
pub mod scheduling;

/// Sorted lineage graph projection.
pub mod graph;
/// Default seed persona request.
pub mod seed;

/// Progress timestamps and allowed-tool lookup.
pub mod progress;

/// Store mutations and event construction for persona lifecycle operations.
pub mod lifecycle;
