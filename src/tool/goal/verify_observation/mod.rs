//! Harness-owned metadata for the most recently started verification in this process.

mod store;
mod ticket;
mod types;
pub(crate) use store::{ObservationStore, shared_observations};
pub(crate) use ticket::Ticket;
pub(crate) use types::{Observation, RunState};

#[cfg(test)]
mod tests;
