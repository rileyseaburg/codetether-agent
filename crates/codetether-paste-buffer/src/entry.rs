//! Stored content and visible placeholder for a summarized paste.
use crate::format_size;

/// One sidecar entry for a paste that was summarised in the input
/// buffer. The `id` shows up in the placeholder rendered for the
/// user; expansion at submit time matches by that id's placeholder.
///
/// # Examples
/// ```
/// let paste = codetether_paste_buffer::PendingTextPaste { id: 1, content: "hello".into() };
/// assert_eq!(paste.lines(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct PendingTextPaste {
    /// Identifier embedded in the visible placeholder.
    pub id: u32,
    /// Original text retained for submission.
    pub content: String,
}

impl PendingTextPaste {
    /// Count lines, reporting at least one for empty content.
    ///
    /// # Examples
    /// ```
    /// let paste = codetether_paste_buffer::PendingTextPaste { id: 1, content: "hello".into() };
    /// assert_eq!(paste.lines(), 1);
    /// ```
    pub fn lines(&self) -> usize {
        self.content.lines().count().max(1)
    }

    /// Count the stored UTF-8 bytes.
    ///
    /// # Examples
    /// ```
    /// let paste = codetether_paste_buffer::PendingTextPaste { id: 1, content: "hello".into() };
    /// assert_eq!(paste.bytes(), 5);
    /// ```
    pub fn bytes(&self) -> usize {
        self.content.len()
    }

    /// User-visible compact summary that takes the place of the full
    /// pasted text in the input buffer and in the chat-history user
    /// message. The agent receives the expanded form at submit time.
    /// Build the visible placeholder for this entry.
    ///
    /// # Examples
    /// ```
    /// let paste = codetether_paste_buffer::PendingTextPaste { id: 1, content: "hello".into() };
    /// assert_eq!(paste.placeholder(), "[Pasted text #1: 1 lines, 5 B]");
    /// ```
    pub fn placeholder(&self) -> String {
        format!(
            "[Pasted text #{}: {} lines, {}]",
            self.id,
            self.lines(),
            format_size(self.bytes())
        )
    }
}
