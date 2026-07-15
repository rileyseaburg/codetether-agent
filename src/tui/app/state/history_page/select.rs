//! Select the chronological page immediately before a visible boundary.

use crate::provider::Message;

use super::anchor;
use super::types::{Page, Request};

pub(super) const PAGE_MESSAGES: usize = 250;

pub(super) enum Decision {
    Ready(Page),
    Grow,
}

pub(super) fn page(messages: &[Message], request: &Request, cap: usize) -> Decision {
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
        messages: selected,
    })
}
