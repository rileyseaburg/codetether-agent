//! State owned by one backward-history paging generation.
use crate::{Fingerprint, PageResult};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
/// Cursor, channel, and viewport state for lazy backward history loading.
///
/// The source type selects the storage backend without storing a backend value.
///
/// # Examples
/// ```
/// let state = codetether_history_page::interactive::HistoryPageState::<()>::default();
/// assert!(!state.expanded());
/// assert_eq!(state.page_step(), 9);
/// ```
pub struct HistoryPageState<S> {
    pub(super) source: std::marker::PhantomData<fn() -> S>,
    pub(super) source_id: Option<String>,
    pub(super) generation: u64,
    pub(super) boundary: Vec<Fingerprint>,
    pub(super) depth: usize,
    pub(super) before: Option<usize>,
    pub(super) loading: bool,
    pub(super) exhausted: bool,
    pub(super) expanded: bool,
    pub(super) viewport_height: usize,
    pub(super) pending_rewind: usize,
    pub(super) pending_old_lines: Option<usize>,
    pub(super) pending_old_scroll: usize,
    pub(super) pending_render_rewind: usize,
    pub(super) tx: UnboundedSender<PageResult>,
    pub(super) rx: UnboundedReceiver<PageResult>,
}
