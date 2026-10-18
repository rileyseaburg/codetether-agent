//! Provider-step request preparation independent of native session ownership.
//!
//! [build_provider_step_state] derives registry and request settings through
//! [Host]. [ProviderStepState] retains those fields for loop refreshes.
//! [system_prompt_for] composes the shared prompt and policy layers.
mod apply_to;
mod build;
mod capabilities;
mod host;
mod prompt;
mod state;
pub use build::build_provider_step_state;
pub use capabilities::model_supports_tools;
pub use host::Host;
pub use prompt::system_prompt_for;
pub use state::ProviderStepState;
