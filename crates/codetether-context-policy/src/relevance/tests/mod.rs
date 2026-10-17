//! relevance::tests contracts and policy operations.

use super::*;

use codetether_provider_core::{ContentPart, Role};
mod fn_text;
use fn_text::text;
mod fn_tool_call;
use fn_tool_call::tool_call;
mod fn_tool_result;
use fn_tool_result::tool_result;
mod fn_bucket_for_messages_merges_recent_window;
mod fn_extract_captures_tool_names_from_tool_calls;
mod fn_extract_picks_up_paths_and_dedupes;
mod fn_extract_recognises_source_extensions_without_slash;
mod fn_extract_tags_error_markers_from_tool_results;
mod fn_project_bucket_escalates_difficulty_with_error_count;
mod fn_project_bucket_maps_axes_correctly;
