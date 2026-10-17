//! Migrated application stream regression.
use super::*;
#[test]
fn stream_tool_chunks_keep_same_call_id_when_followup_delta_omits_id() {
    let mut chunks = Vec::new();
    let mut tool_states = HashMap::new();
    let mut next_fallback_tool_index = 0usize;
    let mut last_seen_tool_index = None;

    append_stream_tool_call_chunks(
        &mut chunks,
        &[zai_stream_types::ZaiStreamToolCall {
            index: Some(0),
            id: Some("call_1".to_string()),
            function: Some(zai_stream_types::ZaiStreamFunction {
                name: Some("bash".to_string()),
                arguments: Some(Value::String("{\"".to_string())),
            }),
        }],
        &mut tool_states,
        &mut next_fallback_tool_index,
        &mut last_seen_tool_index,
    );

    append_stream_tool_call_chunks(
        &mut chunks,
        &[zai_stream_types::ZaiStreamToolCall {
            index: Some(0),
            id: None,
            function: Some(zai_stream_types::ZaiStreamFunction {
                name: None,
                arguments: Some(Value::String("command\":\"pwd\"}".to_string())),
            }),
        }],
        &mut tool_states,
        &mut next_fallback_tool_index,
        &mut last_seen_tool_index,
    );

    finish_stream_tool_call_chunks(&mut chunks, &mut tool_states);

    super::assertions::tool_chunks(&chunks);
}
