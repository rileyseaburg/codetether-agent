//! HAR-style network export construction.

use crate::browser_agent::network::NetworkLogEntry;
use crate::browser_agent::page::BrowserPage;
use crate::browser_session::NetworkEvent;

use super::har_types::{BrowserHarEntry, BrowserHarRequest, BrowserHarTimings};

pub fn entries(page: &BrowserPage) -> Vec<BrowserHarEntry> {
    let logs = page.route_log();
    let mut used = vec![false; logs.len()];
    page.network_events()
        .iter()
        .enumerate()
        .map(|(index, event)| {
            let log = super::har_match::take(event, &logs, &mut used);
            entry(index as u64, event, log)
        })
        .collect()
}

fn entry(sequence: u64, event: &NetworkEvent, log: Option<&NetworkLogEntry>) -> BrowserHarEntry {
    BrowserHarEntry {
        sequence,
        started_ms: event.timestamp_ms,
        request: request(event, log),
        response: super::har_status::response(event, log.map(|item| &item.action)),
        timings: BrowserHarTimings::default(),
    }
}

fn request(event: &NetworkEvent, log: Option<&NetworkLogEntry>) -> BrowserHarRequest {
    BrowserHarRequest {
        method: event.method.clone(),
        url: event.url.clone(),
        headers: log.map(|item| item.headers.clone()).unwrap_or_default(),
        post_data: log.and_then(|item| item.body.clone()),
    }
}
