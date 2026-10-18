//! tool content conversion.
use crate::*;
pub(super) fn push(msg: &Message, _system_blocks: &mut Vec<Value>, api_messages: &mut Vec<Value>) {
    let tool_results = image_content::tool::parts(msg);
    if !tool_results.is_empty() {
        api_messages.push(json!({
            "role": "user",
            "content": tool_results
        }));
    }
}
