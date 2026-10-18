//! Concurrent model catalog discovery and selection variants.
//!
//! Implement [Registry] using configured provider instances, then call
//! [load_available_models] for sorted, deduplicated model identifiers and a
//! [ModelRefreshSummary]. [ModelRefreshEvent] carries results to frontends.
mod event;
mod extend_models;
mod load;
mod outcome;
mod reasoning_refs;
mod registry;
mod selection_policy;
mod summary;
mod summary_status;
pub use event::ModelRefreshEvent;
pub use load::load_available_models;
pub use registry::Registry;
pub use summary::ModelRefreshSummary;
