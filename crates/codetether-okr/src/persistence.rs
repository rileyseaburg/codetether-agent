//! File-backed repository for objectives and execution runs.
//!
//! [`OkrRepository`] caches deserialized records and provides CRUD, query, and
//! statistics operations. Construct it with a storage root or use
//! [`OkrRepository::from_config`] to resolve the default data directory.

mod cache;
mod initialize;
mod okr_create;
mod okr_delete;
mod okr_get;
mod okr_list;
mod okr_queries;
mod okr_update;
mod paths;
mod run_create;
mod run_delete;
mod run_get;
mod run_links;
mod run_list;
mod run_queries;
mod run_update;
mod state;
mod statistics;
mod stats;
pub use state::OkrRepository;
pub use stats::OkrRepositoryStats;
#[cfg(test)]
mod tests;
