//! Configuration policy for native and RLM session model selection.
//!
//! [apply_config] copies configured RLM options, seeds an unset model and
//! resets/resolves the optional subcall provider. [ModelResolver] keeps
//! provider-registry ownership outside the policy crate.
mod apply;
mod default_model;
#[cfg(test)]
mod default_model_tests;
mod resolver;
mod subcall;
pub use apply::apply_config;
pub use default_model::seed_default_model;
pub use resolver::ModelResolver;
pub use subcall::{apply_subcall_model, resolve_subcall_provider};
