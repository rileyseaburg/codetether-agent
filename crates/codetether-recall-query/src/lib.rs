//! Local recall search independent of native session loading.
//!
//! [run] dispatches a [SearchOptions] query to a direct session or a streamed
//! workspace scan. [Host] binds native tail loading and backfill scheduling;
//! projections and ranking come from the shared recall store and index crates.
mod direct;
mod host;
mod migrate;
mod options;
mod search;
/// Recall-tool search adapter over the shared query engine.
pub mod tool;
mod workspace;
pub use host::Host;
pub use options::SearchOptions;
pub use search::run;
