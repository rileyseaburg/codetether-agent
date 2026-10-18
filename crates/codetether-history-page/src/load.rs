//! Background loading and selection of the page before a known boundary.
use crate::indexed;

use crate::HistorySource;

use super::select::Decision;
use super::types::{PageResult, Request};

const SEARCH_MARGIN: usize = 128;

/// Load the page preceding a requested source boundary.
///
/// # Arguments
/// * `request` — Source identifier, generation and current boundary.
///
/// # Returns
/// The original generation and page result.
///
/// # Errors
/// The result contains an error when storage fails, history changes at the boundary, or the search window overflows.
///
/// # Examples
/// ```no_run
/// # async fn sample<S: codetether_history_page::HistorySource>(request: codetether_history_page::Request) {
/// let (_generation, _result) = codetether_history_page::load::<S>(request).await;
/// # }
/// ```
pub async fn run<S: HistorySource>(request: Request) -> PageResult {
    let generation = request.generation;
    (generation, run_page::<S>(request).await)
}

async fn run_page<S: HistorySource>(request: Request) -> Result<super::Page, String> {
    if let Some(before) = request.before {
        return indexed::page::<S>(request, before).await;
    }
    let mut cap = request
        .depth
        .saturating_add(super::select::PAGE_MESSAGES + SEARCH_MARGIN);
    loop {
        let loaded = S::load_tail(&request.source_id, cap)
            .await
            .map_err(|error| error.to_string())?;
        let messages = loaded;
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
