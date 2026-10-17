//! OpenAI-compatible provider regressions.

use super::{OpenAIProvider, Provider};
use serde_json::json;
mod detects_minimax_chat_setting_error_variants;
mod parses_openai_compatible_models_payload;
mod parses_string_only_models_payload;
mod supports_openai_compatible_provider_without_api_key;
