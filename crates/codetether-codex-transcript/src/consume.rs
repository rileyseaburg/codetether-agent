//! Apply one canonical archive record to the transcript accumulator.
use crate::{response::parse_response_item, state::State, usage::parse_event_msg_usage};
use anyhow::Result;
use codetether_codex_records::{
    CodexJsonlRecord, CodexTurnContextPayload, extract_cwd_from_env_context,
};
pub(crate) fn consume(state: &mut State, record: CodexJsonlRecord) -> Result<()> {
    match record.kind.as_str() {
        "session_meta" => state.session_meta = Some(serde_json::from_value(record.payload)?),
        "turn_context" => {
            let ctx: CodexTurnContextPayload = serde_json::from_value(record.payload)?;
            if let Some(next_model) = ctx.model.filter(|value| !value.is_empty()) {
                state.model = Some(next_model);
            }
        }
        "response_item" => {
            // Recover cwd from legacy <environment_context> before consuming the payload.
            if let Some(meta) = state.session_meta.as_mut()
                && meta.cwd.is_empty()
                && let Some(cwd) = extract_cwd_from_env_context(&record.payload)
            {
                meta.cwd = cwd;
            }
            if let Some(message) = parse_response_item(record.payload, &mut state.first_user_text)?
            {
                state.messages.push(message);
            }
        }
        "event_msg" => {
            if let Some(next_usage) = parse_event_msg_usage(record.payload)? {
                state.usage = next_usage;
            }
        }
        _ => {}
    }
    Ok(())
}
