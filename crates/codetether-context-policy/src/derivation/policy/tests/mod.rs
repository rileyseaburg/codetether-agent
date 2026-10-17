//! Persisted derivation policy selectors tests.

use super::*;
mod fn_default_is_incremental;
use fn_default_is_incremental::default_is_incremental;
mod fn_reset_carries_threshold;
use fn_reset_carries_threshold::reset_carries_threshold;
mod fn_kind_is_snake_case_and_distinct;
use fn_kind_is_snake_case_and_distinct::kind_is_snake_case_and_distinct;
mod fn_incremental_carries_budget;
use fn_incremental_carries_budget::incremental_carries_budget;
mod fn_policy_round_trips_through_serde;
use fn_policy_round_trips_through_serde::policy_round_trips_through_serde;
