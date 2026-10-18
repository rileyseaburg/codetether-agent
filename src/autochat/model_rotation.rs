//! Application-bound auto-chat model selection.
pub use codetether_application_providers::rotation::{
    RelayModelRotation, build_round_robin_model_rotation, resolve_provider_for_model_autochat,
};
