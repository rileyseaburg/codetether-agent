//! Active-goal accounting, terminal transitions, and continuation prompts.

mod account;
mod adopt;
mod auto_goal;
mod continuation;
mod load;
mod prompt;
mod resume;
mod system_prompt;
mod transition;

pub(crate) use account::record_usage;
pub(crate) use adopt::adopt_prompt;
pub(crate) use auto_goal::{enabled as auto_goal_enabled, set_enabled as set_auto_goal};
pub(crate) use continuation::next_message;
pub(crate) use load::current;
pub(crate) use resume::prompt as resume_prompt;
pub(crate) use system_prompt::compose;
pub(crate) use transition::set_status;
