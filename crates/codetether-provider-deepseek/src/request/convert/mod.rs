//! Message ordering and role dispatch.
use codetether_provider_core::{ContentPart, Message, Role};
use serde_json::{Value, json};
use super::convert_helpers;
mod tool_msg;
mod assistant_msg;
mod text_msg;
use tool_msg::tool_msg;
use assistant_msg::assistant_msg;
use text_msg::text_msg;

pub(crate) fn messages(msgs: &[Message]) -> Vec<Value> {
    msgs.iter()
        .map(|m| match m.role {
            Role::Tool => tool_msg(m),
            Role::Assistant => assistant_msg(m),
            _ => text_msg(m),
        })
        .collect()
}
