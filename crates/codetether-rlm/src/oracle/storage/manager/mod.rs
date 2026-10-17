//! Spool-first trace persistence with an optional remote backend.

mod initialize;
mod internals;
mod persist;
mod state;
#[cfg(test)]
mod testing;
pub use state::OracleTraceStorage;

pub(in super::super) const DEFAULT_MAX_SPOOL_BYTES: u64 = 500 * 1024 * 1024;
