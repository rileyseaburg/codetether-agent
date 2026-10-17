//! SSE data-frame parsing and terminal event handling.
use crate::{StreamChunk, streaming::state::State, zai_stream_types::ZaiStreamResponse};

pub(crate) fn process(
    state: &mut State,
    line: &str,
    chunks: &mut Vec<StreamChunk>,
    text_buf: &mut String,
) {
    if line == "data: [DONE]" {
        crate::stream_done::finish(
            chunks,
            text_buf,
            &mut state.seen,
            &mut state.tool_states,
            state.frames,
            state.last_finish.as_deref(),
            state.final_usage.take(),
        );
        return;
    }
    if let Some(data) = line.strip_prefix("data: ")
        && let Ok(parsed) = serde_json::from_str::<ZaiStreamResponse>(data)
    {
        state.frames += 1;
        if let Some(usage) = parsed.usage.as_ref() {
            state.final_usage = Some(if state.total_prompt_usage {
                super::usage::total_prompt(usage)
            } else {
                crate::usage::convert(usage)
            });
        }
        if let Some(choice) = parsed.choices.first() {
            crate::streaming::choice::process(state, choice, chunks, text_buf);
        }
    }
}
