//! delegation::tests_state::tests contracts and policy operations.

use crate::delegation::{DelegationConfig, DelegationState};

use crate::relevance::{Bucket, Dependency, Difficulty, ToolUse};
mod fn_bucket;
use fn_bucket::bucket;
mod fn_delegate_to_respects_margin;
mod fn_delegation_state_update_seeds_and_records;
mod fn_shrink_cold_start_pulls_neighbour_mass;
