//! collapse duplicate calls.
use super::*;

/// Collapse duplicate tool-call arguments into short back-references.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{ContentPart, Message, Role};
/// use codetether_context_policy::transforms::tool_call_dedup::{
///     collapse_duplicate_calls, KEEP_LAST_MESSAGES,
/// };
///
/// let args = serde_json::json!({
///     "path": "src/very/deeply/nested/module/with/a/long/path/to/the/file.rs",
///     "encoding": "utf-8",
/// }).to_string();
///
/// let call = |id: &str| Message {
///     role: Role::Assistant,
///     content: vec![ContentPart::ToolCall {
///         id: id.into(),
///         name: "read_file".into(),
///         arguments: args.clone(),
///         thought_signature: None,
///     }],
/// };
///
/// let mut msgs = vec![call("first"), call("second")];
/// for i in 0..KEEP_LAST_MESSAGES + 1 {
///     msgs.push(Message {
///         role: Role::User,
///         content: vec![ContentPart::Text { text: format!("q{i}") }],
///     });
/// }
///
/// let stats = collapse_duplicate_calls(&mut msgs);
/// assert!(stats.dedup_hits >= 1);
///
/// // Second call's arguments now reference the first.
/// let ContentPart::ToolCall { arguments, .. } = &msgs[1].content[0] else {
///     panic!();
/// };
/// assert!(arguments.starts_with("[REPEAT"));
/// assert!(arguments.contains("first"));
/// ```
pub fn collapse_duplicate_calls(messages: &mut [Message]) -> ExperimentalStats {
    let mut stats = ExperimentalStats::default();
    let total = messages.len();
    if total <= KEEP_LAST_MESSAGES {
        return stats;
    }
    let eligible = total - KEEP_LAST_MESSAGES;
    let mut seen: HashMap<[u8; 32], String> = HashMap::new();

    for msg in messages[..eligible].iter_mut() {
        for part in msg.content.iter_mut() {
            let ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } = part
            else {
                continue;
            };
            if arguments.len() < MIN_COLLAPSE_BYTES {
                continue;
            }
            if arguments.starts_with("[REPEAT") {
                continue;
            }
            let mut h = Sha256::new();
            h.update(name.as_bytes());
            h.update(b"\0");
            h.update(arguments.as_bytes());
            let key: [u8; 32] = h.finalize().into();
            match seen.get(&key) {
                Some(first_id) => {
                    let marker =
                        format!("[REPEAT of call_id={first_id}, {} bytes]", arguments.len());
                    let saved = arguments.len().saturating_sub(marker.len());
                    *arguments = marker;
                    stats.dedup_hits += 1;
                    stats.total_bytes_saved += saved;
                }
                None => {
                    seen.insert(key, id.clone());
                }
            }
        }
    }

    stats
}
