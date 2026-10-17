//! Per-tick work selection, budget windows and idle detection.
//!
//! [select_work] advances active personas within their resource limits.
//! [is_idle] excludes budget-paused personas from idle reaping.
mod idle;
mod tick_budget;
mod tick_window;
pub use idle::{children_of, is_idle, mark_reaped};
pub use tick_budget::select_work;
pub use tick_window::{budget_paused_event, over_budget, reset_window_if_elapsed};

#[cfg(test)]
mod tick_budget_tests;
#[cfg(test)]
mod tick_budget_tests_support;

mod idle_reap;
/// Reap expired idle personas and direct children.
pub use idle_reap::reap_idle;
