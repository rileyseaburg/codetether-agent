//! Owner reply queue and its acknowledgement/expiry lifecycle.
//!
//! Queue validation and lifecycle transitions extend the shared runtime;
//! HTTP handlers use those methods without owning queue state.
mod lifecycle;
mod queue;
