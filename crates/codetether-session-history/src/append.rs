//! Append messages and keep page classifications aligned.
use crate::History;
use codetether_context_policy::pages::{classify, classify_all};
use codetether_provider_core::Message;
impl History<'_> {
    /// Append a single message to the end of the history.
    ///
    /// The only mutating operation on [`History`] — every other mutation
    /// must be routed through a dedicated API on the owning type.
    /// # Arguments
    /// * `msg` — Message to classify and append at the end of the transcript.
    /// # Examples
    /// ```
    /// use codetether_provider_core::{Message, Role};
    /// let mut messages = Vec::new();
    /// let mut history = codetether_session_history::History::new(&mut messages);
    /// history.append(Message { role: Role::User, content: Vec::new() });
    /// assert_eq!(history.len(), 1);
    /// ```
    pub fn append(&mut self, msg: Message) {
        if let Some(pages) = self.pages.as_deref_mut() {
            if pages.len() != self.buf.len() {
                *pages = classify_all(self.buf);
            }
            pages.push(classify(&msg));
        }
        self.buf.push(msg);
    }
}
