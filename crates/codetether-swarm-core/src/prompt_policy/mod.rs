//! Shared composition of delegated task prompts and sourced constraints.
//!
//! [system_prompt] combines [SystemPromptInput], repository policy, capability,
//! quality and verification requirements. The host supplies session instructions.
use crate::deliverable::VerificationContract;
mod capability_prompt;
mod constraint_entry;
mod constraint_extract;
mod constraint_ledger;
mod constraint_mode;
mod project_quality;
mod prompt_input;
mod prompt_sections;
mod quality_contract;
mod source_metric_contract;
pub use prompt_input::SystemPromptInput;
mod prompts;
pub use prompts::system_prompt;
