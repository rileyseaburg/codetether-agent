//! Validation evidence guardrails injected into agent prompts.

mod assembly;
mod belief_keywords;
mod belief_recall;
mod capabilities;
mod capability;
mod claim;
mod claim_note;
mod core_memory;
pub(super) mod digest;
mod extract;
mod extract_artifact;
mod extract_db;
mod extract_platform;
mod extract_runtime;
mod final_gate;
mod final_note;
mod gate_mode;
mod guardrails;
mod ledger;
mod ledger_path;
mod ledger_persist;
mod level;
mod memory_protocol;
mod message_text;
mod prefetch;
mod prompt;
mod record;
mod recovery;
mod scope;
mod scope_extract;
mod scope_item;
mod sections;
mod trapdoor;
mod workflow;
mod workflow_templates;
mod writeback;
mod writeback_path;
mod writeback_persist;

pub(crate) use final_gate::gate as gate_final_answer;
pub(crate) use prompt::append_guardrails_for_cwd;

#[cfg(test)]
mod prompt_tests;
