//! Read exactly one indexed page, even when the conversation has millions of turns.
use super::super::{
    anchor,
    select::PAGE_MESSAGES,
    types::{Page, Request},
};
use crate::session::Session;
pub(super) async fn page(request: Request, before: usize) -> Result<Page, String> {
    let start = before.saturating_sub(PAGE_MESSAGES);
    let mut messages = Session::read_messages(
        &request.source_id,
        start,
        before - start + request.boundary.len(),
    )
    .await
    .map_err(|error| error.to_string())?;
    let boundary = before - start;
    if messages.len() < boundary || anchor::fingerprints(&messages[boundary..]) != request.boundary
    {
        return Err("history changed at the viewport boundary; reload the session".into());
    }
    messages.truncate(boundary);
    Ok(Page {
        boundary: anchor::fingerprints(&messages),
        exhausted: start == 0,
        depth: request.depth.saturating_add(messages.len()),
        messages,
        before: Some(start),
    })
}
