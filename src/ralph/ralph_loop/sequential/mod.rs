//! Native bindings for shared sequential Ralph execution.
//!
//! Story lifecycle policy lives in codetether-ralph-sequential; these modules
//! supply the existing runtime state, prompts, reporting and effect services.
use super::*;
mod driver;
mod host;
mod scheduling_sequential;
