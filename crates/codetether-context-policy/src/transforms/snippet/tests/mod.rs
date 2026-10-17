//! snippet::tests transform rules.

use super::*;
mod tool_msg;
use tool_msg::tool_msg;
mod user_msg;
use user_msg::user_msg;
mod already_snipped_outputs_are_not_resnipped;
mod recent_tool_outputs_are_protected;
mod short_histories_are_untouched;
mod utf8_boundaries_are_respected;
