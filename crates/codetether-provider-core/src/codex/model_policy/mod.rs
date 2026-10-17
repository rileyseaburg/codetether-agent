//! Codex selector metadata, backend eligibility, and model-option policy.
//!
//! Runtime configuration stays with callers. [resolve_model_and_reasoning_effort_and_service_tier]
//! accepts a lazy callback; metadata and filters share the existing discovery cache.
use super::{
    CodexServiceTier, ThinkingLevel, model_catalog, model_discovery, parse_model_thinking_level,
    parse_service_tier_model_alias, reasoning_catalog,
};
use crate::ModelInfo;
mod api_key_hidden_model;
mod astra_model;
mod available_models;
mod discovered_model_info;
mod model_info;
mod model_is_supported_by_backend;
mod model_supports_reasoning_effort;
mod resolve;
pub mod transport_catalog;
pub use api_key_hidden_model::api_key_hidden_model;
pub use available_models::available_models;
pub use discovered_model_info::discovered_model_info;
pub use model_info::model_info;
pub use model_is_supported_by_backend::model_is_supported_by_backend;
pub use model_supports_reasoning_effort::model_supports_reasoning_effort;
pub use resolve::resolve_model_and_reasoning_effort_and_service_tier;

pub use astra_model::info as astra_info;
