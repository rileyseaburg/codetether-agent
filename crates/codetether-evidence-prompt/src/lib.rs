//! Evidence and prior-context instruction rendering without storage access.
//!
//! [assembly::PromptSections] carries caller-supplied memory and prefetch text.
//! [assembly::render] preserves section ordering. The remaining renderers supply
//! validation, runtime, workflow and permission-sensitive guidance.
pub mod assembly;
pub mod capabilities;
mod capability;
pub mod claim_note;
pub mod failure_attribution;
pub mod guardrails;
pub mod memory_protocol;
pub mod trapdoor;
pub mod workflow_templates;
pub mod writeback;
