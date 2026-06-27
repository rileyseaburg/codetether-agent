//! Parse a single SSE frame block into its `id` / `event` / `data` fields.
//!
//! A frame is the text between `\n\n` delimiters. See
//! `docs/transport-phase1-wire-contract.md` section 3.

use super::event_id::EventId;

/// A parsed SSE frame: optional resumable id, event type, and data payload.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::stream::frame::parse_frame;
///
/// let block = "id: ep.7\nevent: progress\ndata: {\"x\":1}";
/// let frame = parse_frame(block).unwrap();
/// assert_eq!(frame.id.unwrap().seq, 7);
/// assert_eq!(frame.event, "progress");
/// assert_eq!(frame.data, "{\"x\":1}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedFrame {
    /// Resumable event id when the server emitted an `id:` line.
    pub id: Option<EventId>,
    /// The `event:` type, defaulting to `message` when absent.
    pub event: String,
    /// The concatenated `data:` payload.
    pub data: String,
}

/// Parse a frame block. Returns `None` when there is no `data:` line.
pub fn parse_frame(block: &str) -> Option<ParsedFrame> {
    let mut id = None;
    let mut event = "message".to_string();
    let mut data: Option<String> = None;
    for line in block.lines() {
        if let Some(v) = line.strip_prefix("id:") {
            id = EventId::parse(v.trim());
        } else if let Some(v) = line.strip_prefix("event:") {
            event = v.trim().to_string();
        } else if let Some(v) = line.strip_prefix("data:") {
            data = Some(v.trim().to_string());
        }
    }
    data.map(|data| ParsedFrame { id, event, data })
}

#[cfg(test)]
mod tests {
    use super::parse_frame;

    #[test]
    fn no_id_defaults_message_type_when_absent() {
        let f = parse_frame("data: hello").unwrap();
        assert!(f.id.is_none());
        assert_eq!(f.event, "message");
        assert_eq!(f.data, "hello");
    }

    #[test]
    fn returns_none_without_data() {
        assert!(parse_frame("event: heartbeat").is_none());
    }
}
