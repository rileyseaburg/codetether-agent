//! Append one content block using the summarizer transcript format.
use codetether_provider_core::ContentPart;

pub(super) fn append(out: &mut String, part: &ContentPart) {
    match part {
        ContentPart::Text { text } => {
            out.push_str(text);
            out.push('\n');
        }
        ContentPart::Thinking { text, .. } => {
            if !text.trim().is_empty() {
                out.push_str("[Thinking]\n");
                out.push_str(text);
                out.push('\n');
            }
        }
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            ..
        } => {
            out.push_str(&format!(
                "[ToolCall id={id} name={name}]\nargs: {arguments}\n"
            ));
        }
        ContentPart::ToolResult {
            tool_call_id,
            content,
        } => {
            out.push_str(&format!("[ToolResult id={tool_call_id}]\n"));
            out.push_str(content);
            out.push('\n');
        }
        ContentPart::Image { mime_type, url } => {
            out.push_str(&format!(
                "[Image mime_type={} url_len={}]\n",
                mime_type.clone().unwrap_or_else(|| "unknown".to_string()),
                url.len()
            ));
        }
        ContentPart::File { path, mime_type } => {
            out.push_str(&format!(
                "[File path={} mime_type={}]\n",
                path,
                mime_type.clone().unwrap_or_else(|| "unknown".to_string())
            ));
        }
    }
}
