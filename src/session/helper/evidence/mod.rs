//! Validation evidence guardrails injected into agent prompts.

mod assembly;
mod belief_keywords;
mod belief_recall;
mod capabilities;
mod capability;
mod claim;
mod core_memory;
pub(super) mod digest;
mod guardrails;
mod level;
mod memory_protocol;
mod prompt;
mod recovery;
mod scope;
mod trapdoor;
mod workflow;

pub(crate) use prompt::{append_guardrails, append_guardrails_for_cwd};

#[cfg(test)]
mod prompt_tests;
