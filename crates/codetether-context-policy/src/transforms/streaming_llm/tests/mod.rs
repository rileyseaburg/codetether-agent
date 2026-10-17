//! streaming llm::tests transform rules.

use super::*;
mod u;
use u::u;
mod a;
use a::a;
mod does_not_split_tool_call_pair;
mod long_history_preserves_sinks_and_recency;
mod short_history_is_noop;
