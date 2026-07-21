//! User-facing labels for lightweight session listings.

use super::summary::SessionSummary;

impl SessionSummary {
    /// Return the normalized persisted title when it is safe to display.
    ///
    /// # Returns
    ///
    /// `Some` with collapsed whitespace for a usable title, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::SessionSummary;
    /// let encoded = r#"{"id":"s1","title":" Review   storage ","created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z","message_count":1,"agent":"default","directory":null}"#;
    /// let summary: SessionSummary = serde_json::from_str(encoded).unwrap();
    /// assert_eq!(summary.display_title().as_deref(), Some("Review storage"));
    /// ```
    pub fn display_title(&self) -> Option<String> {
        self.title
            .as_deref()
            .and_then(crate::session::title::display::normalize_title)
    }

    /// Return a title or compact identifier suitable for user interfaces.
    ///
    /// # Returns
    ///
    /// A normalized title, the first eight identifier characters as a
    /// fallback, or `new` when neither is available.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::SessionSummary;
    /// let encoded = r#"{"id":"abcdef123","title":null,"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z","message_count":0,"agent":"default","directory":null}"#;
    /// let summary: SessionSummary = serde_json::from_str(encoded).unwrap();
    /// assert_eq!(summary.display_label(), "abcdef12");
    /// ```
    pub fn display_label(&self) -> String {
        crate::session::title::display::label(self.title.as_deref(), &self.id)
    }
}
