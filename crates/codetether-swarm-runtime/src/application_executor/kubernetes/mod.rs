//! Kubernetes stage execution using the shared application executor.
//!
//! [execute] forwards the executor services and the current planning identity
//! to the existing Kubernetes backend.
mod host;
mod run;
pub use run::execute;
