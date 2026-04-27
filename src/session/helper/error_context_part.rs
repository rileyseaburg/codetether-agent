//! Content-part formatting for RLM context serialization.

use crate::provider::ContentPart;

pub fn write_part(out: &mut String, part: &ContentPart) {
    match part {
        ContentPart::Text { text } => line(out, text),
        ContentPart::Thinking { text } => thinking(out, text),
        ContentPart::ToolCall { id, name, arguments, .. } => tool_call(out, id, name, arguments),
        ContentPart::ToolResult { tool_call_id, content } => result(out, tool_call_id, content),
        ContentPart::Image { mime_type, url } => image(out, mime_type, url),
        ContentPart::File { path, mime_type } => file(out, path, mime_type),
    }
}

fn tool_call(out: &mut String, id: &str, name: &str, arguments: &str) {
    line(out, &format!("[ToolCall id={id} name={name}]\nargs: {arguments}"));
}

fn result(out: &mut String, id: &str, content: &str) {
    line(out, &format!("[ToolResult id={id}]\n{content}"));
}

fn image(out: &mut String, mime_type: &Option<String>, url: &str) {
    line(out, &format!("[Image mime_type={} url_len={}]", mime(mime_type), url.len()));
}

fn file(out: &mut String, path: &str, mime_type: &Option<String>) {
    line(out, &format!("[File path={} mime_type={}]", path, mime(mime_type)));
}

fn thinking(out: &mut String, text: &str) {
    if text.trim().is_empty() {
        return;
    }
    line(out, "[Thinking]");
    line(out, text);
}

fn line(out: &mut String, text: &str) {
    out.push_str(text);
    out.push('\n');
}

fn mime(mime_type: &Option<String>) -> String {
    mime_type.clone().unwrap_or_else(|| "unknown".to_string())
}
