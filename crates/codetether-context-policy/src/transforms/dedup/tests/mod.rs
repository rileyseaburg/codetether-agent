//! dedup::tests transform rules.

use super::*;
mod tool_msg;
use tool_msg::tool_msg;
mod padding_msg;
use padding_msg::padding_msg;
mod distinct_outputs_are_preserved;
mod recent_identical_tool_result_is_not_deduplicated;
mod short_outputs_are_not_deduplicated;
mod three_way_dedup_references_first_sighting;
