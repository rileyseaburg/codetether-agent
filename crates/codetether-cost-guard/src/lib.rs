//! Cost budget enforcement shared by agent loops and display adapters.
//!
//! [enforce_cost_budget] checks current spend before a request and emits a
//! one-time warning before reaching the hard limit. [cost_guard_level] reports
//! severity without consuming that warning. Thresholds use shared configuration.
mod check;
mod enforce;
mod level;
mod status;
pub use enforce::enforce_cost_budget;
pub use level::cost_guard_level;
pub use status::{CostGuardLevel, CostGuardStatus};
