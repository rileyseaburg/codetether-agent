//! Active-goal accounting, terminal transitions, and continuation prompts.

mod account;
mod adopt;
pub mod answer_review;
mod continuation;
#[doc(hidden)]
pub mod journal;
mod load;
mod resume;
mod system_prompt;
mod transition;
mod transition_checked;
pub use transition_checked::set_status_if_current;

pub use account::record_usage;
pub use adopt::adopt_prompt;
pub use codetether_goal_runtime::{auto_goal_enabled, set_auto_goal};
pub use continuation::next_message;
pub use load::current;
pub use resume::prompt as resume_prompt;
pub use system_prompt::compose;
pub use transition::set_status;
