//! Validation evidence guardrails injected into agent prompts.

mod claim;
mod level;
mod prompt;
mod recovery;

pub(crate) use prompt::append_guardrails;
