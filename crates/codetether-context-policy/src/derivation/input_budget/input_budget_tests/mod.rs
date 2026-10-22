//! Existing input_budget_tests regressions.
//! Regression tests for model-relative input budgets.

use super::{calculate, resolve, usable};
mod fn_calculation_scales_with_context_window;
mod fn_explicit_budget_is_preserved_or_capped;
mod fn_reserves_the_actual_output_limit_even_above_a_quarter_window;
mod fn_zero_uses_selected_models_runtime_capacity;
