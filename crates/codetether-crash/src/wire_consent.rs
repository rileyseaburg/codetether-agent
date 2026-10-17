// Module declarations and local exports for crash consent.
mod maybe_prompt_for_consent;
pub use maybe_prompt_for_consent::maybe_prompt_for_consent;
mod prompt_yes_no;
use prompt_yes_no::prompt_yes_no;
mod persist_consent_choice;
use persist_consent_choice::persist_consent_choice;
mod env_non_empty;
use env_non_empty::env_non_empty;
