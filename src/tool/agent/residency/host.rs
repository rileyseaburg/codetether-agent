//! Bind shared residency policy to application stores and session overlays.
mod activation;
mod storage;
/// Application operations used by shared residency policy.
pub struct ApplicationHost;
/// Prepared native manifest and session, opaque outside application bindings.
pub struct Prepared(super::super::persistence::ClosedAgent);
