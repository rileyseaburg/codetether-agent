//! Background loading and selection of the page before a known boundary.

use crate::session::Session;

use super::select::Decision;
use super::types::{PageResult, Request};

const SEARCH_MARGIN: usize = 128;

pub(super) async fn run(request: Request) -> PageResult {
    let generation = request.generation;
    (generation, run_page(request).await)
}

async fn run_page(request: Request) -> Result<super::Page, String> {
    let mut cap = request
        .depth
        .saturating_add(super::select::PAGE_MESSAGES + SEARCH_MARGIN);
    loop {
        let loaded = Session::load_tail(&request.source_id, cap)
            .await
            .map_err(|error| error.to_string())?;
        let messages = loaded.session.messages;
        match super::select::page(&messages, &request, cap) {
            Decision::Ready(page) => return Ok(page),
            Decision::Grow if messages.len() < cap => {
                return Err("visible history boundary no longer exists in source session".into());
            }
            Decision::Grow => cap = grow(cap)?,
        }
    }
}

fn grow(cap: usize) -> Result<usize, String> {
    cap.checked_mul(2)
        .filter(|next| *next > cap)
        .ok_or_else(|| "history page search exceeded addressable memory".to_string())
}
