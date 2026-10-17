//! trim middle.
use super::*;

/// Apply sink + recent middle-drop to `messages`.
///
/// Returns a stats struct whose `total_bytes_saved` reflects the bytes
/// removed from the buffer.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{ContentPart, Message, Role};
/// use codetether_context_policy::transforms::streaming_llm::{
///     trim_middle, STREAM_MIN_MESSAGES,
/// };
///
/// let user = |t: &str| Message {
///     role: Role::User,
///     content: vec![ContentPart::Text { text: t.into() }],
/// };
/// let asst = |t: &str| Message {
///     role: Role::Assistant,
///     content: vec![ContentPart::Text { text: t.into() }],
/// };
///
/// let mut msgs: Vec<Message> = Vec::new();
/// msgs.push(user("original intent"));
/// msgs.push(asst("plan"));
/// for i in 0..STREAM_MIN_MESSAGES + 5 {
///     msgs.push(user(&format!("filler user {i}")));
///     msgs.push(asst(&"A".repeat(200)));
/// }
/// let before = msgs.len();
/// let stats = trim_middle(&mut msgs);
/// assert!(stats.total_bytes_saved > 0);
/// assert!(msgs.len() < before);
/// // Sinks preserved.
/// assert!(matches!(msgs[0].content[0], ContentPart::Text { ref text } if text == "original intent"));
/// ```
pub fn trim_middle(messages: &mut Vec<Message>) -> ExperimentalStats {
    let mut stats = ExperimentalStats::default();
    if messages.len() < STREAM_MIN_MESSAGES {
        return stats;
    }

    let total = messages.len();
    let mut cut_start = SINK_MESSAGES;
    let mut cut_end = total - RECENT_MESSAGES;

    // Never split a tool_call ⇄ tool_result pair across the boundary.
    //
    // The original approach was to *retreat* cut_end past any open tool
    // call, hoping the paired tool result sat at cut_end so both would
    // end up in the preserved tail. That breaks when the call is
    // orphaned at the boundary (no following Tool result in the
    // recency window): retreating leaves the open ToolCall stranded at
    // the head of the preserved tail followed by a User message,
    // which every provider's chat API rejects.
    //
    // Instead we *advance* cut_end forward into what would have been
    // the recent window, absorbing the orphan call (and any immediate
    // Tool results) into the dropped middle. This costs at most a few
    // messages of recency but guarantees the preserved tail starts on
    // a clean boundary — no dangling Tool result, no orphan ToolCall.
    while cut_end < total && has_open_tool_call(&messages[cut_end - 1]) {
        cut_end += 1;
    }
    while cut_end < total && messages[cut_end].role == Role::Tool {
        cut_end += 1;
    }
    while cut_start < cut_end && messages[cut_start].role == Role::Tool {
        cut_start += 1;
    }
    if cut_end <= cut_start + 1 {
        return stats;
    }

    let dropped = cut_end - cut_start;
    let bytes_dropped: usize = messages[cut_start..cut_end].iter().map(approx_bytes).sum();

    let marker = Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: format!(
                "[...streaming-llm trim: {dropped} messages ({bytes_dropped} bytes) elided between sinks and recent window...]"
            ),
        }],
    };
    let marker_bytes = approx_bytes(&marker);

    messages.splice(cut_start..cut_end, std::iter::once(marker));
    stats.snippet_hits += 1;
    stats.total_bytes_saved = bytes_dropped.saturating_sub(marker_bytes);
    stats
}
