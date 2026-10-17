//! delegation::tests_query::tests_rank contracts and policy operations.

use crate::delegation::{DelegationConfig, DelegationState};

use crate::relevance::{Bucket, Dependency, Difficulty, ToolUse};
mod fn_bucket;
use fn_bucket::bucket;
mod fn_rank_candidates_is_none_for_empty_input;
mod fn_rank_candidates_picks_first_on_cold_start;
mod fn_rank_candidates_prefers_best_scoring_once_warm;
