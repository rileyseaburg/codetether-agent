//! Active-goal accounting, terminal transitions, and continuation prompts.

mod account;
mod continuation;
mod load;
mod prompt;
mod resume;
mod system_prompt;
mod transition;

pub(crate) use account::record_usage;
pub(crate) use continuation::next_message;
pub(crate) use load::current;
pub(crate) use resume::prompt as resume_prompt;
pub(crate) use system_prompt::compose;
pub(crate) use transition::set_status;
