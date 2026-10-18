//! Borrowed history state and constructors.
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;

/// Append-only handle to a chat-history buffer.
///
/// Wraps `&mut Vec<Message>` but exposes only [`append`](Self::append)
/// and [`view`](Self::view). Nothing can reach the underlying `Vec` to
/// call `pop`, `truncate`, `clear`, `split_off`, `remove`, or `last_mut`
/// while the History handle holds its exclusive borrow.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{ContentPart, Message, Role};
/// use codetether_session_history::History;
///
/// let mut buf: Vec<Message> = Vec::new();
/// let mut history = History::new(&mut buf);
/// assert!(history.view().is_empty());
///
/// history.append(Message {
///     role: Role::Assistant,
///     content: vec![ContentPart::Text {
///         text: "ok".to_string(),
///     }],
/// });
/// assert_eq!(history.view().len(), 1);
/// ```
pub struct History<'a> {
    pub(crate) buf: &'a mut Vec<Message>,
    pub(crate) pages: Option<&'a mut Vec<PageKind>>,
}
impl<'a> History<'a> {
    /// Wrap an existing `Vec<Message>` as an append-only history.
    ///
    /// # Arguments
    ///
    /// * `buf` — The underlying buffer. Ownership is not taken; the
    ///   caller keeps the `Vec` but loses the ability to rewrite it
    ///   while the `History` handle is alive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_provider_core::Message;
    /// use codetether_session_history::History;
    ///
    /// let mut buf: Vec<Message> = Vec::new();
    /// let history = History::new(&mut buf);
    /// assert!(history.view().is_empty());
    /// ```
    pub fn new(buf: &'a mut Vec<Message>) -> Self {
        Self { buf, pages: None }
    }

    /// Wrap a history buffer plus its parallel page sidecar.
    /// # Arguments
    /// * `buf` — Existing message buffer, borrowed exclusively.
    /// * `pages` — Parallel page classifications, repaired on the next append if needed.
    /// # Returns
    /// A handle that updates both buffers when appending.
    /// # Examples
    /// ```
    /// let mut messages = Vec::new();
    /// let mut pages = Vec::new();
    /// let history = codetether_session_history::History::with_pages(&mut messages, &mut pages);
    /// assert!(history.is_empty());
    /// ```
    pub fn with_pages(buf: &'a mut Vec<Message>, pages: &'a mut Vec<PageKind>) -> Self {
        Self {
            buf,
            pages: Some(pages),
        }
    }
}
