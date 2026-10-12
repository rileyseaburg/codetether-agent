//! State retained between chunks of a single SSE response.
use crate::{Usage, stream_output::OutputSeen, zai_stream_state::ZaiStreamToolState};
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct State {
    pub(crate) buffer: String,
    pub(crate) tool_states: HashMap<usize, ZaiStreamToolState>,
    pub(crate) next_tool_index: usize,
    pub(crate) last_tool_index: Option<usize>,
    pub(crate) seen: OutputSeen,
    pub(crate) frames: usize,
    pub(crate) last_finish: Option<String>,
    pub(crate) final_usage: Option<Usage>,
}
