//! Persisted derivation policy selectors tests.

use super::*;
mod fn_default_is_incremental;
mod fn_reset_carries_threshold;
mod fn_kind_is_snake_case_and_distinct;
mod fn_incremental_carries_budget;
mod fn_policy_round_trips_through_serde;
