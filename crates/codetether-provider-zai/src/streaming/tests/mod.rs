//! Application stream contracts and usage accounting tests.
use crate::zai_stream_assembly::{append_stream_tool_call_chunks, finish_stream_tool_call_chunks};
use crate::zai_stream_state::ZaiStreamToolState;
use crate::zai_stream_types::{self, ZaiStreamResponse};
use crate::{StreamChunk, Usage};
use std::collections::HashMap;
mod assertions;
mod stream_response_parses_usage_block;
mod stream_response_without_usage_or_choices_is_ok;
mod stream_tool_chunks_keep_same_call_id_when_followup_delta_omits_id;
mod usage;
