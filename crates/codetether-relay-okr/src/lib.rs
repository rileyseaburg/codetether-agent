//! Relay OKR templates, model drafting and response normalization.
//!
//! draft uses a caller-selected provider; okr_from_planned_draft validates its
//! schema values and falls back to default_relay_okr_template when needed.
mod convert;
mod draft;
mod parse;
mod planned;
mod prompt;
mod request;
mod template;
pub use convert::okr_from_planned_draft;
pub use draft::draft;
pub use planned::PlannedOkrDraft;
pub use template::default_relay_okr_template;

mod command;
mod pending;
mod pending_new;
mod pending_prompt;
mod rotation;
pub use command::is_easy_go_command;
pub use pending::PendingOkrApproval;
pub use rotation::next_go_model;
/// GLM model used by the relay rotation.
pub const GO_SWAP_MODEL_GLM: &str = "zai/glm-5";
/// MiniMax model used by the relay rotation.
pub const GO_SWAP_MODEL_MINIMAX: &str = "minimax/MiniMax-M3";

/// CLI relay progress persistence and final outcome recording.
pub mod progress;

/// CLI objective approval display and persistence.
pub mod cli_approval;
/// Interactive command preparation and objective approval drafting.
pub mod interactive;

#[cfg(test)]
mod legacy_tui_tests;

mod application_binding;

/// Types used by frontend goal-preparation declarations.
#[doc(hidden)]
pub mod __private {
    pub use codetether_okr::OkrRepository;
    pub use codetether_provider_core::Provider;
}
