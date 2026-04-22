//! StreamingLLM-style middle-drop with attention sinks.
//!
//! Based on *Efficient Streaming Language Models with Attention Sinks*
//! (Xiao et al., 2023; refined 2024). The empirical finding: the first
//! few tokens of a sequence act as "attention sinks" that stabilize
//! later attention distributions, and the *recent* window carries most
//! of the locally-relevant signal. The middle can be dropped with
//! minimal quality loss at long contexts.
//!
//! We apply the same idea at **message granularity** rather than token
//! granularity, which is the right level for an agentic loop:
//!
//! * **Sinks** — the first [`SINK_MESSAGES`] messages (typically the
//!   user's original intent + any system/scaffolding prefix).
//! * **Recent window** — the last [`RECENT_MESSAGES`] messages (active
//!   reasoning the model is building on).
//! * **Middle** — compacted into a single user-visible marker message.
//!
//! # Safety
//!
//! Only activates when history is long enough that the middle is
//! genuinely stale (see [`STREAM_MIN_MESSAGES`]). Never drops content
//! while a tool call is open: we always cut on a boundary where the
//! most recent dropped message is not a `Role::Assistant` with an
//! unresolved tool call. This preserves the tool-call ⇄ tool-result
//! pairing required by every provider's chat API.
//!
//! # Always-on
//!
//! No config flag. If history is short, this is a no-op. If history is
//! long, the trim is beneficial regardless of model — the bytes saved
//! go straight to input-token cost.

use super::ExperimentalStats;
use crate::provider::{ContentPart, Message, Role};

/// Number of leading messages preserved as attention sinks.
pub const SINK_MESSAGES: usize = 2;

/// Number of trailing messages preserved as the recency window.
pub const RECENT_MESSAGES: usize = 16;

/// Below this total message count, streaming trim is a no-op.
pub const STREAM_MIN_MESSAGES: usize = SINK_MESSAGES + RECENT_MESSAGES + 4;

/// Apply sink + recent middle-drop to `messages`.
///
/// Returns a stats struct whose `total_bytes_saved` reflects the bytes
/// removed from the buffer.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{ContentPart, Message, Role};
/// use codetether_agent::session::helper::experimental::streaming_llm::{
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

fn has_open_tool_call(msg: &Message) -> bool {
    msg.role == Role::Assistant
        && msg
            .content
            .iter()
            .any(|p| matches!(p, ContentPart::ToolCall { .. }))
}

fn approx_bytes(msg: &Message) -> usize {
    msg.content
        .iter()
        .map(|p| match p {
            ContentPart::Text { text } => text.len(),
            ContentPart::ToolResult { content, .. } => content.len(),
            ContentPart::ToolCall {
                arguments, name, ..
            } => arguments.len() + name.len(),
            ContentPart::Thinking { .. } => 0,
            ContentPart::Image { .. } | ContentPart::File { .. } => 1024,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u(t: &str) -> Message {
        Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: t.into() }],
        }
    }
    fn a(t: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text { text: t.into() }],
        }
    }

    #[test]
    fn short_history_is_noop() {
        let mut msgs = vec![u("a"), a("b"), u("c"), a("d")];
        let before = msgs.clone();
        let stats = trim_middle(&mut msgs);
        assert_eq!(stats.total_bytes_saved, 0);
        assert_eq!(msgs.len(), before.len());
    }

    #[test]
    fn long_history_preserves_sinks_and_recency() {
        let mut msgs = vec![u("SINK0"), a("SINK1")];
        for i in 0..60 {
            msgs.push(u(&format!("mid-user-{i}")));
            msgs.push(a(&"X".repeat(300)));
        }
        for i in 0..RECENT_MESSAGES {
            msgs.push(a(&format!("recent-{i}")));
        }
        let stats = trim_middle(&mut msgs);
        assert!(stats.total_bytes_saved > 1000);
        // Sink intact.
        let ContentPart::Text { text } = &msgs[0].content[0] else {
            panic!();
        };
        assert_eq!(text, "SINK0");
        // A streaming-llm marker appears somewhere after the sinks.
        assert!(msgs.iter().any(|m| m.content.iter().any(|p| matches!(
            p,
            ContentPart::Text { text } if text.contains("streaming-llm trim")
        ))));
    }

    #[test]
    fn does_not_split_tool_call_pair() {
        // Build a history that would put a ToolCall at cut_end-1 if we
        // weren't careful. Assert it moves the cut to protect the pair.
        let mut msgs = vec![u("sink0"), a("sink1")];
        for _ in 0..40 {
            msgs.push(u("filler"));
            msgs.push(a("filler"));
        }
        // Inject an open tool call right at the boundary.
        let boundary = msgs.len() - RECENT_MESSAGES - 1;
        msgs[boundary] = Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".into(),
                name: "read_file".into(),
                arguments: "{}".into(),
                thought_signature: None,
            }],
        };
        let _ = trim_middle(&mut msgs);
        // Walk: there must not be a ToolCall whose next message isn't a Tool.
        for i in 0..msgs.len().saturating_sub(1) {
            if has_open_tool_call(&msgs[i]) {
                // Next message must be Role::Tool or still Assistant (multi-call).
                let next = &msgs[i + 1];
                assert!(
                    next.role == Role::Tool || next.role == Role::Assistant,
                    "tool call at {i} orphaned; next role {:?}",
                    next.role
                );
            }
        }
    }
}
