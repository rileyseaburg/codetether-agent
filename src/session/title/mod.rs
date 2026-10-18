//! Native session title operations backed by the shared title crate.
mod ai;
mod ai_default;
pub(super) mod display;
mod host;
use super::types::Session;
use anyhow::Result;
pub(crate) use codetether_session_title::is_title_candidate;
impl Session {
    /// Generate a title from the first user message if one is not already
    /// set.
    pub async fn generate_title(&mut self) -> Result<()> {
        codetether_session_title::generate(self).await
    }
    /// Regenerate the title from the first user message, even if already
    /// set.
    pub async fn regenerate_title(&mut self) -> Result<()> {
        codetether_session_title::regenerate(self).await
    }
    /// Set a custom title for the session.
    ///
    /// Whitespace is collapsed before persistence; empty or injected bootstrap
    /// context clears the title so it can be regenerated safely.
    ///
    /// # Arguments
    ///
    /// * `title` - Candidate user-facing title.
    pub fn set_title(&mut self, title: impl Into<String>) {
        codetether_session_title::set(self, title);
    }
    /// Clear the title, allowing it to be regenerated on the next call to
    /// [`Session::generate_title`].
    pub fn clear_title(&mut self) {
        codetether_session_title::clear(self);
    }
    /// React to a context change (directory change, model change, etc.).
    /// Bumps `updated_at` and optionally regenerates the title.
    pub async fn on_context_change(&mut self, regenerate_title: bool) -> Result<()> {
        codetether_session_title::context_change(self, regenerate_title).await
    }
}
