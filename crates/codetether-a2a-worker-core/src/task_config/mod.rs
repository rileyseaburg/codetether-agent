//! Task envelope readers and worker execution configuration.
//!
//! Metadata parsers retain nested lookup precedence. Timeout and step-budget
//! policy consume the same parsers; heartbeat configuration reads environment
//! overrides without starting runtime work.
use crate::model_ref_to_provider_model;
mod metadata_lookup;
pub use metadata_lookup::metadata_lookup;
mod metadata_flags;
pub use metadata_flags::metadata_bool;
mod metadata_numbers_metadata_usize;
pub use metadata_numbers_metadata_usize::metadata_usize;
mod metadata_numbers_metadata_u64;
pub use metadata_numbers_metadata_u64::metadata_u64;
mod metadata_numbers_metadata_f64;
pub use metadata_numbers_metadata_f64::metadata_f64;
mod metadata_numbers_parse_keys;
use metadata_numbers_parse_keys::parse_keys;
mod metadata_strings_metadata_str;
pub use metadata_strings_metadata_str::metadata_str;
mod metadata_strings_metadata_string_list;
pub use metadata_strings_metadata_string_list::metadata_string_list;
mod metadata_strings_parse_value;
use metadata_strings_parse_value::parse_value;
mod task_data;
pub use task_data::{task_metadata, task_str, task_value};
mod task_timeout;
pub use task_timeout::task_timeout_secs;
mod persistent_heartbeat;
pub use persistent_heartbeat::{persistent_worker_enabled, persistent_worker_lease_seconds};
mod session_step_budget;
pub use session_step_budget::{
    DEFAULT_WORKER_MAX_STEPS, WORKER_MAX_STEPS_ENV, resolve_step_budget,
};
mod task_context_helpers;
pub use task_context_helpers::{context_selected_model, context_workspace_id, tier_from_metadata};

mod task_context;
mod task_context_struct;
pub use task_context::build_task_context;
pub use task_context_struct::TaskContext;
