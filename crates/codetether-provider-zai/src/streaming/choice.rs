//! Streaming choice dispatch for reasoning, text and tool-call deltas.
use crate::zai_stream_assembly::{append_stream_tool_call_chunks, finish_stream_tool_call_chunks};
use crate::{StreamChunk, streaming::state::State, zai_stream_types::ZaiStreamChoice};

pub(crate) fn process(
    state: &mut State,
    choice: &ZaiStreamChoice,
    chunks: &mut Vec<StreamChunk>,
    text_buf: &mut String,
) {
    if let Some(reasoning) = choice.delta.reasoning_content.as_deref()
        && !reasoning.is_empty()
    {
        state.seen.text = true;
        crate::delta::push_reasoning(chunks, text_buf, reasoning);
    }
    if let Some(content) = choice.delta.content.as_deref()
        && !content.is_empty()
    {
        state.seen.text = true;
        text_buf.push_str(content);
    }
    if let Some(tool_calls) = choice.delta.tool_calls.as_ref() {
        flush(chunks, text_buf);
        state.seen.tool_calls = true;
        append_stream_tool_call_chunks(
            chunks,
            tool_calls,
            &mut state.tool_states,
            &mut state.next_tool_index,
            &mut state.last_tool_index,
        );
    }
    if let Some(reason) = choice.finish_reason.as_ref() {
        state.last_finish = Some(reason.clone());
        flush(chunks, text_buf);
        if reason == "tool_calls" {
            finish_stream_tool_call_chunks(chunks, &mut state.tool_states);
        }
    }
}
fn flush(chunks: &mut Vec<StreamChunk>, text_buf: &mut String) {
    if !text_buf.is_empty() {
        chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
    }
}
