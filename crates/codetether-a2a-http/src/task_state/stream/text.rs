//! Collect textual prompt parts without executing the request.
use super::*;
/// Join the request’s text parts with newline separators.
///
/// # Arguments
///
/// * `params` - Message request whose text parts are collected.
///
/// # Returns
///
/// The text parts in request order, separated by newlines.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_http::task_state::stream::text::text;
/// let _operation = text;
/// ```
pub fn text(params: &MessageSendParams) -> String {
    params
        .message
        .parts
        .iter()
        .filter_map(|p| match p {
            Part::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
