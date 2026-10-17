//! delegation::lambda::tests contracts and policy operations.

use super::*;
mod fn_clamps_below_min_to_min;
mod fn_nan_falls_back_to_default;
mod fn_one_passes_through_above_one_falls_back;
mod fn_passes_through_in_range;
