//! Evidence prompt orchestration, project memory integration and final gating.
//!
//! [append_guardrails_for_cwd] loads permitted workspace context and assembles
//! one idempotent prompt. [gate_final_answer] checks final claims and persists
//! ledgers and proven beliefs using the original runtime ordering.
mod belief_keywords;
mod belief_recall;
mod core_memory;
pub mod digest;
mod final_gate;
mod palace_belief;
mod palace_sync;
mod prefetch;
mod prompt;
mod sections;
pub(crate) use codetether_evidence_core::{gate_mode, verification_gate};
pub(crate) use codetether_evidence_ledger::{
    final_note, ledger, ledger_persist, scope_item, writeback_persist, writeback_read,
};
pub(crate) use codetether_evidence_prompt::{
    assembly, capabilities, claim_note, failure_attribution, guardrails, memory_protocol, trapdoor,
    workflow_templates, writeback,
};
pub use final_gate::gate as gate_final_answer;
pub use prompt::append_guardrails_for_cwd;
#[cfg(test)]
mod failure_attribution_tests;
#[cfg(test)]
mod prompt_tests;
