//! Durable goal execution, continuation prompts and answer-review holds.
//!
//! Implement [Journal] to resolve session journals. [current] uses a shared
//! metadata-keyed fold cache; [record_usage], [set_status] and [next_message]
//! append existing governance events. [answer_review] retains user-only hold
//! operations. [compose] and [turn_instructions] render the existing contracts.
mod account;
mod adopt;
pub mod answer_review;
mod auto_goal;
mod boundary;
mod continuation;
mod journal;
mod load;
mod prompt;
mod resume;
pub mod state_cache;
mod system_prompt;
mod transition;
mod transition_checked;
pub mod turn_instructions;
pub use account::record_usage;
pub use adopt::adopt_prompt;
pub use auto_goal::{enabled as auto_goal_enabled, set_enabled as set_auto_goal};
pub use boundary::ensure_running;
pub use continuation::next_message;
pub use journal::Journal;
pub use load::current;
pub use resume::prompt as resume_prompt;
pub use system_prompt::{compose, from_log as compose_from_log};
pub use transition::set_status;
pub use transition_checked::set_status_if_current;
