//! Original model and message conversion regression tests.
use super::*;
mod convert_messages_serializes_tool_arguments_as_json_string;
mod convert_messages_wraps_invalid_tool_arguments;
mod normalize_model_aliases_glm5_to_fp8;
mod normalize_model_preserves_other_variants;
mod normalize_model_strips_prefix;
