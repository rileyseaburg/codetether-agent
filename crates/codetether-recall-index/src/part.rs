//! Bounded content-part rendering that omits model thinking.
use codetether_provider_core::ContentPart;
const MAX_FIELD_BYTES: usize = 2048;

/// Render a single content part for recall, skipping thinking and
/// truncating large tool results.
/// # Arguments
/// * `part` - One provider content block.
/// # Returns
/// Bounded text with provenance markers, or empty text for thinking blocks.
/// # Examples
/// ```
/// let part = codetether_provider_core::ContentPart::Text { text: "hello".into() };
/// assert_eq!(codetether_recall_index::part::render_part(&part), "hello\n");
/// ```
pub fn render_part(part: &ContentPart) -> String {
    match part {
        ContentPart::Text { text } => format!("{text}\n"),
        ContentPart::Thinking { .. } => String::new(),
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            ..
        } => {
            let args = super::truncate::truncate_bytes(arguments, MAX_FIELD_BYTES);
            format!("[ToolCall id={id} name={name}]\nargs: {args}\n")
        }
        ContentPart::ToolResult {
            tool_call_id,
            content,
        } => super::part_result::render(tool_call_id, content),
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
