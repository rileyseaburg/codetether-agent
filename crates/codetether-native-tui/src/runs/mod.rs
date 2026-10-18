//! Background run services for terminal swarm and forage commands.
//!
//! [Swarm] attaches shared control, events and the global agent bus. [Forage]
//! selects the shared quiet-mode and forage engine. The frontend supplies its
//! concrete backends while retaining presentation state and view updates.
mod forage;
mod swarm;
pub use forage::Forage;
pub use swarm::Swarm;
