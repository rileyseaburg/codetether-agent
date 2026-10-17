//! Sequential benchmark scheduling, scoring and publication.
//!
//! Implement BenchmarkBackend to supply model loading and one-PRD execution,
//! then call run_suite. The backend retains application-specific integration.
mod backend;
mod finish;
mod model;
mod prds;
mod pricing;
mod record;
mod run;
pub use backend::BenchmarkBackend;
pub use run::run_suite;
