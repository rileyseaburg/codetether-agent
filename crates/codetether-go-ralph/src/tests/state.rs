//! Original terminal Ralph state used in outcome assertions.

use codetether_ralph_core::{Prd, RalphState, RalphStatus};
use std::path::PathBuf;
pub(super) fn create(prd: &Prd) -> RalphState {
    RalphState {
        prd: prd.clone(),
        current_iteration: 3,
        max_iterations: 10,
        status: RalphStatus::MaxIterations,
        progress_log: vec![],
        prd_path: PathBuf::from("test.json"),
        working_dir: PathBuf::from("."),
    }
}
