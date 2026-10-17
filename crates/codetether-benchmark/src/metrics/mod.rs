//! Benchmark aggregation and ranking policy.
//! [compute_aggregate] summarizes PRDs; [compute_summary] ranks models.
use crate::types::*;
mod aggregate;
mod best_pass;
mod cheapest;
mod fastest;
mod limits;
mod rank;
mod summary;
pub use aggregate::compute_aggregate;
use best_pass::best_pass;
use cheapest::cheapest;
use fastest::fastest;
use limits::limits;
use rank::rank_models;
pub use summary::compute_summary;
