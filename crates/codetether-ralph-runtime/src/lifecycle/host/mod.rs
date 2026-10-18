//! Runtime state and scheduling bindings for the shared run lifecycle.
//!
//! The run crate owns lifecycle order; these implementations lend live state
//! and call the existing policy-aware checkout and scheduling services.
use super::*;
mod execution;
mod state;
