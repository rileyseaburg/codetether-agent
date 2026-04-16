use super::text::role_label;
use crate::provider::{ContentPart, Message};

pub fn messages_to_rlm_context(messages: &[Message]) -> String {
    let mut out = String::new();
    for (idx, m) in messages.iter().enumerate() {
        out.push_str(&format!("[{} {}]\n", idx, role_label(m.role)));

        for part in &m.content {
            match part {
                ContentPart::Text { text } => {
                    out.push_str(text);
                    out.push('\n');
                }
                ContentPart::Thinking { text } => {
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

        out.push_str("\n---\n\n");
    }
    out
}

pub fn is_prompt_too_long_error(err: &anyhow::Error) -> bool {
    let msg = err.to_string().to_ascii_lowercase();
    msg.contains("prompt is too long")
        || msg.contains("context length")
        || msg.contains("maximum context")
        || (msg.contains("tokens") && msg.contains("maximum") && msg.contains("prompt"))
}

pub fn is_retryable_upstream_error(err: &anyhow::Error) -> bool {
    let msg = err.to_string().to_ascii_lowercase();
    [
        " 500 ",
        " 504 ",
        " 502 ",
        " 429 ",
        "status code 500",
        "status code 504",
        "status code 502",
        "status code 429",
        "internal server error",
        "gateway timeout",
        "upstream request timeout",
        "server error: 500",
        "server error: 504",
        "network error",
        "connection reset",
        "connection refused",
        "timed out",
        "timeout",
        "broken pipe",
        "unexpected eof",
    ]
    .iter()
    .any(|needle| msg.contains(needle))
}
