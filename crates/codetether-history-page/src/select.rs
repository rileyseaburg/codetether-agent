//! Select the chronological page immediately before a visible boundary.

use codetether_provider_core::Message;

use super::anchor;
use super::types::{Page, Request};

/// Maximum messages selected by one backward page.
pub const PAGE_MESSAGES: usize = 250;

/// Outcome of searching a loaded message window.
///
/// # Examples
/// ```
/// use codetether_history_page::select::Decision;
/// let decision = Decision::Grow;
/// assert!(matches!(decision, Decision::Grow));
/// ```
pub enum Decision {
    /// A complete preceding page was located.
    Ready(Page),
    /// A larger tail window is needed to locate the boundary.
    Grow,
}

/// Select the page preceding a stable boundary.
///
/// # Arguments
/// * `messages` — Loaded chronological tail window.
/// * `request` — Requested source boundary and depth.
/// * `cap` — Maximum messages requested in the tail load.
///
/// # Returns
/// A ready page or a request to grow the search window.
///
/// # Examples
/// ```
/// use codetether_history_page::{Request, select::{page, Decision}};
/// let request = Request { generation: 0, source_id: "session".into(), boundary: vec![], depth: 0, before: None };
/// assert!(matches!(page(&[], &request, 250), Decision::Grow));
/// ```
pub fn page(messages: &[Message], request: &Request, cap: usize) -> Decision {
    let Some(boundary) = anchor::find(messages, &request.boundary) else {
        return Decision::Grow;
    };
    let full_history = messages.len() < cap;
    if boundary < PAGE_MESSAGES && !full_history {
        return Decision::Grow;
    }
    let start = boundary.saturating_sub(PAGE_MESSAGES);
    let selected = messages[start..boundary].to_vec();
    Decision::Ready(Page {
        boundary: anchor::fingerprints(&selected),
        exhausted: start == 0 && full_history,
        depth: request.depth.saturating_add(selected.len()),
        before: None,
        messages: selected,
    })
}
