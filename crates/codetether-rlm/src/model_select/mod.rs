//! RLM model selection policy.
//!
//! Hosts provide providers and score signals; this crate owns the order
//! of precedence for choosing which RLM model reference to try.

mod configured;
mod defaults;
mod resolve;
#[cfg(test)]
mod tests;
mod types;

pub use defaults::RLM_MODEL_ENV;
pub use resolve::{select_rlm_model, select_rlm_model_with_env};
pub use types::{RlmModelChoice, RlmModelPurpose, RlmModelSource};
