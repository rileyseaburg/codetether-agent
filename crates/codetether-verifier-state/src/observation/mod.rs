//! Harness-owned metadata for the most recently started verification in this process.

mod store;
mod ticket;
mod types;
pub use store::{ObservationStore, shared_observations};
pub use ticket::Ticket;
pub use types::{Observation, RunState};

#[cfg(test)]
mod tests;
