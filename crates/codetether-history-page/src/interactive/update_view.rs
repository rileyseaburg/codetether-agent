//! Borrowed state and initial measurements for one history update operation.
use super::{HistoryPageState, View};
use crate::HistorySource;
use codetether_chat_model::ChatMessage;

/// Inputs for one call to [super::drain] or [super::apply_pending_history_anchor].
///
/// Measurements describe the viewport before the update. Construct a new view
/// for each call so a previous cache invalidation or anchor change cannot leave
/// stale measurements. Mutations use the borrowed slots directly.
///
/// # Examples
/// ```
/// use codetether_history_page::{HistorySource, interactive::{HistoryPageState, UpdateView}};
/// fn example<S: HistorySource + 'static>(pager: &mut HistoryPageState<S>) {
///     let mut messages = Vec::new();
///     let mut status = String::new();
///     let (mut scroll, mut follow) = (0, false);
///     let view = UpdateView {
///         pager, follow_latest: false, manual_scroll: 0,
///         cached_lines: 0, last_max_scroll: 0, messages: &mut messages,
///         clear_cache: || {}, status: &mut status,
///         scroll: &mut scroll, auto_follow: &mut follow,
///     };
///     assert_eq!(view.cached_lines, 0);
/// }
/// ```
pub struct UpdateView<'a, S, C> {
    /// Active backward-history pager.
    pub pager: &'a mut HistoryPageState<S>,
    /// Whether the viewport initially followed the latest output.
    pub follow_latest: bool,
    /// Initial manual scroll position.
    pub manual_scroll: usize,
    /// Number of initially cached rendered lines.
    pub cached_lines: usize,
    /// Last measured maximum scroll position.
    pub last_max_scroll: usize,
    /// Chronological transcript updated when a page arrives.
    pub messages: &'a mut Vec<ChatMessage>,
    /// Invalidate the frontend's rendered transcript cache.
    pub clear_cache: C,
    /// Status line updated after loading or failing a page.
    pub status: &'a mut String,
    /// Current viewport scroll position.
    pub scroll: &'a mut usize,
    /// Current follow-latest flag.
    pub auto_follow: &'a mut bool,
}

impl<S: HistorySource + 'static, C: FnMut()> View<S> for UpdateView<'_, S, C> {
    fn pager(&mut self) -> &mut HistoryPageState<S> {
        self.pager
    }
    fn follow_latest(&self) -> bool {
        self.follow_latest
    }
    fn manual_scroll(&self) -> usize {
        self.manual_scroll
    }
    fn cached_lines(&self) -> usize {
        self.cached_lines
    }
    fn last_max_scroll(&self) -> usize {
        self.last_max_scroll
    }
    fn messages(&mut self) -> &mut Vec<ChatMessage> {
        self.messages
    }
    fn clear_cache(&mut self) {
        (self.clear_cache)();
    }
    fn set_status(&mut self, status: String) {
        *self.status = status;
    }
    fn set_scroll(&mut self, scroll: usize) {
        *self.scroll = scroll;
    }
    fn set_auto_follow(&mut self, follow: bool) {
        *self.auto_follow = follow;
    }
}
