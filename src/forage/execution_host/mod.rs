//! Existing application services for shared forage execution.
mod forage;
mod runtime;
use crate::swarm::SwarmExecutor as NativeSwarm;
pub(super) struct Native;
