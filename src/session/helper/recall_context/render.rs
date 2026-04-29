//! Part rendering for recall context (budget-aware).

use crate::provider::ContentPart;

/// Maximum characters retained from a single `ToolResult` or `ToolCall` args.
const MAX_FIELD_CHARS: usize = 2048;

/// Render a single content part for recall, skipping thinking and
/// truncating large tool results.
pub fn render_part(part: &ContentPart) -> String {
    match part {
        ContentPart::Text { text } => {
            let mut s = text.clone();
            s.push('\n');
            s
        }
        // Skip thinking — low recall value, high token cost
        ContentPart::Thinking { .. } => String::new(),
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            ..
        } => {
            let args_preview = truncate_str(arguments, MAX_FIELD_CHARS);
            format!("[ToolCall id={id} name={name}]\nargs: {args_preview}\n")
        }
        ContentPart::ToolResult {
            tool_call_id,
            content,
        } => {
            let preview = truncate_str(content, MAX_FIELD_CHARS);
            let ellipsis = if content.len() > MAX_FIELD_CHARS {
                " [...truncated]"
            } else {
                ""
            };
            format!("[ToolResult id={tool_call_id}]\n{preview}{ellipsis}\n")
        }
        ContentPart::Image { mime_type, url } => {
            let mime = mime_type.clone().unwrap_or_else(|| "unknown".into());
            format!("[Image mime_type={mime} url_len={}]\n", url.len())
        }
        ContentPart::File { path, mime_type } => {
            let mime = mime_type.clone().unwrap_or_else(|| "unknown".into());
            format!("[File path={path} mime_type={mime}]\n")
        }
    }
}

/// Truncate to `max_len` bytes, respecting char boundaries.
fn truncate_str(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        return s;
    }
    let boundary = s.floor_char_boundary(max_len);
    &s[..boundary]
}

#[cfg(test)]
mod render_tests {
    use super::*;

    #[test]
    fn skips_thinking() {
        use crate::provider::ContentPart;
        let part = ContentPart::Thinking {
            text: "reasoning".into(),
        };
        assert!(render_part(&part).is_empty());
    }

    #[test]
    fn truncates_tool_result() {
        use crate::provider::ContentPart;
        let big = "x".repeat(5000);
        let part = ContentPart::ToolResult {
            tool_call_id: "c1".into(),
            content: big,
        };
        let rendered = render_part(&part);
        assert!(rendered.contains("[...truncated]"));
    }

    #[test]
    fn truncate_str_char_boundary() {
        let s = "hello 🌍 world";
        let t = truncate_str(s, 10);
        assert!(t.len() <= 10);
    }
}
