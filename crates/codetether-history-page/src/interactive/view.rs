//! Presentation fields consumed when applying completed history pages.
use super::HistoryPageState;
use crate::HistorySource;
use codetether_chat_model::ChatMessage;
/// Bind a native chat view to shared page application and viewport anchoring.
pub trait View<S: HistorySource + 'static> {
    /// Borrow the active pager.
    fn pager(&mut self) -> &mut HistoryPageState<S>;
    /// Whether the chat uses the follow-latest sentinel.
    fn follow_latest(&self) -> bool;
    /// Current manual scroll position.
    fn manual_scroll(&self) -> usize;
    /// Number of cached rendered lines.
    fn cached_lines(&self) -> usize;
    /// Last measured maximum scroll.
    fn last_max_scroll(&self) -> usize;
    /// Borrow the chronological chat messages.
    fn messages(&mut self) -> &mut Vec<ChatMessage>;
    /// Invalidate the rendered transcript cache.
    fn clear_cache(&mut self);
    /// Replace the status line.
    fn set_status(&mut self, status: String);
    /// Set the manual viewport scroll position.
    fn set_scroll(&mut self, scroll: usize);
    /// Enable or disable automatic following of the latest message.
    fn set_auto_follow(&mut self, follow: bool);
}
