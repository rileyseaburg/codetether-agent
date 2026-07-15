//! Background backward pagination state for the chat transcript.

#[path = "history_page/anchor.rs"]
mod anchor;
#[path = "history_page/apply.rs"]
mod apply;
#[path = "history_page/default.rs"]
mod default;
#[path = "history_page/drain.rs"]
mod drain;
#[path = "history_page/load.rs"]
mod load;
#[path = "history_page/render_anchor.rs"]
mod render_anchor;
#[path = "history_page/request.rs"]
mod request;
#[path = "history_page/select.rs"]
mod select;
#[path = "history_page/types.rs"]
mod types;
#[path = "history_page/viewport.rs"]
mod viewport;

#[cfg(test)]
#[path = "history_page/race_tests.rs"]
mod race_tests;
#[cfg(test)]
#[path = "history_page/tests.rs"]
mod tests;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub(crate) use drain::drain;
use types::Page;
use types::{Fingerprint, PageResult};

/// Cursor, channel, and viewport state for lazy backward history loading.
pub(crate) struct HistoryPageState {
    source_id: Option<String>,
    generation: u64,
    boundary: Vec<Fingerprint>,
    depth: usize,
    loading: bool,
    exhausted: bool,
    expanded: bool,
    viewport_height: usize,
    pending_rewind: usize,
    pending_old_lines: Option<usize>,
    pending_old_scroll: usize,
    pending_render_rewind: usize,
    tx: UnboundedSender<PageResult>,
    rx: UnboundedReceiver<PageResult>,
}
