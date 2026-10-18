//! Types exchanged by the asynchronous history-page loader.

use codetether_provider_core::Message;

/// Stable hash of one serialized message.
pub type Fingerprint = u64;
/// Generation-tagged result so callers can discard obsolete responses.
pub type PageResult = (u64, Result<Page, String>);

/// Chronological messages preceding the requested visible boundary.
///
/// # Examples
/// ```
/// let page = codetether_history_page::Page { messages: vec![], boundary: vec![], exhausted: true, depth: 0, before: None };
/// assert!(page.exhausted);
/// ```
pub struct Page {
    /// Selected messages in chronological order.
    pub messages: Vec<Message>,
    /// Fingerprints anchoring the beginning of the visible window.
    pub boundary: Vec<Fingerprint>,
    /// Whether the beginning of the source has been reached.
    pub exhausted: bool,
    /// Number of messages loaded at and after the current boundary.
    pub depth: usize,
    /// Optional absolute storage index before the visible window.
    pub before: Option<usize>,
}

/// Source and boundary identifying the next backward page.
///
/// # Examples
/// ```
/// let request = codetether_history_page::Request { generation: 0, source_id: "session".into(), boundary: vec![], depth: 0, before: None };
/// assert_eq!(request.generation, 0);
/// ```
pub struct Request {
    /// Caller generation echoed with the load result.
    pub generation: u64,
    /// Identifier understood by the storage backend.
    pub source_id: String,
    /// Fingerprints anchoring the beginning of the visible window.
    pub boundary: Vec<Fingerprint>,
    /// Number of messages loaded at and after the current boundary.
    pub depth: usize,
    /// Optional absolute storage index before the visible window.
    pub before: Option<usize>,
}
