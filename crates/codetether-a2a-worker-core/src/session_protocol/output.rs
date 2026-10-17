//! Append assistant text and notify the worker output callback.
use codetether_provider_core::ContentPart;
use std::sync::Arc;
/// Append nonempty text parts, preserving callback and newline order.
///
/// # Arguments
/// * `parts` - Provider content to scan.
/// * `out` - Accumulated text buffer.
/// * `cb` - Optional observer called for each nonempty text part.
/// # Returns
/// Unit after appending the text parts.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::session_protocol::append_text_output;
/// use codetether_provider_core::ContentPart;
/// let mut text = String::new();
/// append_text_output(&[ContentPart::Text { text: "done".into() }], &mut text, &None);
/// assert_eq!(text, "done\n");
/// ```

pub fn append_text_output(
    parts: &[ContentPart],
    out: &mut String,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) {
    for part in parts {
        if let ContentPart::Text { text } = part
            && !text.is_empty()
        {
            out.push_str(text);
            out.push('\n');
            if let Some(cb) = cb {
                cb(text.clone());
            }
        }
    }
}
