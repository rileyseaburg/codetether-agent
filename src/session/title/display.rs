//! User-facing session title normalization and labels.
//!
//! New titles are normalized before persistence, while labels also sanitize
//! legacy titles so display code never renders injected bootstrap context.

use super::super::types::Session;

pub(in crate::session) fn normalize_title(raw: &str) -> Option<String> {
    let mut title = String::with_capacity(raw.len());
    for word in raw.split(|ch: char| ch.is_whitespace() || ch.is_control()) {
        if word.is_empty() {
            continue;
        }
        if !title.is_empty() {
            title.push(' ');
        }
        title.push_str(word);
    }
    super::extract::is_title_candidate(&title).then_some(title)
}

pub(in crate::session) fn label(title: Option<&str>, id: &str) -> String {
    title
        .and_then(normalize_title)
        .unwrap_or_else(|| short_id(id))
}

fn short_id(id: &str) -> String {
    let id = id.trim();
    if id.is_empty() {
        "new".to_string()
    } else {
        id.chars().take(8).collect()
    }
}

impl Session {
    /// Return a normalized title or a compact identifier for display.
    ///
    /// # Returns
    ///
    /// The stored title with whitespace collapsed, the first eight identifier
    /// characters when no safe title exists, or `new` when both are absent.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::session::Session;
    /// let mut session = Session::new().await.unwrap();
    /// session.set_title("  Review   storage ");
    /// assert_eq!(session.display_label(), "Review storage");
    /// # });
    /// ```
    pub fn display_label(&self) -> String {
        label(self.title.as_deref(), &self.id)
    }
}
