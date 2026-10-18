//! Relay planning requests and response handling.
//!
//! Reuse an initialized [ModelRegistry] with [plan_okr_draft_with_registry],
//! [plan_relay_profiles_with_registry] or [decide_dynamic_spawn_with_registry].
//! Provider failures and unusable planning responses preserve the optional fallback.
mod okr;
mod okr_request;
mod profiles;
mod profiles_request;
mod spawn;
mod spawn_request;
mod text;
pub use codetether_autochat_core::model_rotation::ModelRegistry;
pub use okr::plan_okr_draft_with_registry;
pub use profiles::plan_relay_profiles_with_registry;
pub use spawn::decide_dynamic_spawn_with_registry;
