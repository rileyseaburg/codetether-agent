//! Provider error classification and normalization.

#[path = "error_detection/normalization.rs"]
mod normalization;
#[path = "error_detection/overload.rs"]
mod overload;
#[path = "error_detection/retryable.rs"]
mod retryable;
pub use normalization::{normalize_provider_alias, smart_switch_model_key};
pub use overload::{codex_overload, codex_overload_cooldown};

pub use retryable::is_retryable_provider_error;
