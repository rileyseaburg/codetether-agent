//! dedup tool outputs.
use super::*;

/// Replace duplicate tool-result contents with a back-reference marker.
///
/// Walks `messages` in order. The first tool result for a given content
/// hash is kept verbatim; every subsequent identical result is replaced
/// with a marker of the form:
///
/// ```text
/// [DEDUP] identical to tool_call_id=<first-id> (<N> bytes)
/// ```
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{ContentPart, Message, Role};
/// use codetether_context_policy::transforms::dedup::{
///     dedup_tool_outputs, KEEP_LAST_MESSAGES,
/// };
///
/// let big = "x".repeat(1024);
/// let mut msgs = vec![
///     Message {
///         role: Role::Tool,
///         content: vec![ContentPart::ToolResult {
///             tool_call_id: "a".into(),
///             content: big.clone(),
///         }],
///     },
///     Message {
///         role: Role::Tool,
///         content: vec![ContentPart::ToolResult {
///             tool_call_id: "b".into(),
///             content: big,
///         }],
///     },
/// ];
/// // Push enough padding so both tool results fall outside the
/// // trailing keep-window and are eligible for dedup.
/// for _ in 0..KEEP_LAST_MESSAGES {
///     msgs.push(Message {
///         role: Role::User,
///         content: vec![ContentPart::Text { text: "...".into() }],
///     });
/// }
///
/// let stats = dedup_tool_outputs(&mut msgs);
/// assert_eq!(stats.dedup_hits, 1);
/// assert!(stats.total_bytes_saved > 512);
///
/// // First copy is preserved intact.
/// if let ContentPart::ToolResult { content, .. } = &msgs[0].content[0] {
///     assert_eq!(content.len(), 1024);
/// } else {
///     panic!("expected tool result");
/// }
///
/// // Second copy is now a back-reference pointing at call_a.
/// if let ContentPart::ToolResult { content, .. } = &msgs[1].content[0] {
///     assert!(content.starts_with("[DEDUP]"));
///     assert!(content.contains("tool_call_id=a"));
/// } else {
///     panic!("expected tool result");
/// }
/// ```
pub fn dedup_tool_outputs(messages: &mut [Message]) -> ExperimentalStats {
    let mut seen: HashMap<[u8; 32], String> = HashMap::new();
    let mut stats = ExperimentalStats::default();

    let total = messages.len();
    let eligible = total.saturating_sub(KEEP_LAST_MESSAGES);
    if eligible == 0 {
        return stats;
    }

    for msg in messages[..eligible].iter_mut() {
        for part in msg.content.iter_mut() {
            let ContentPart::ToolResult {
                tool_call_id,
                content,
            } = part
            else {
                continue;
            };
            if content.len() < MIN_DEDUP_BYTES {
                continue;
            }
            let hash = Sha256::digest(content.as_bytes()).into();
            match seen.get(&hash) {
                Some(first_id) => {
                    let marker = format!(
                        "[DEDUP] identical to tool_call_id={first_id} ({} bytes)",
                        content.len()
                    );
                    let saved = content.len().saturating_sub(marker.len());
                    *content = marker;
                    stats.dedup_hits += 1;
                    stats.total_bytes_saved += saved;
                }
                None => {
                    seen.insert(hash, tool_call_id.clone());
                }
            }
        }
    }

    stats
}
