//! Browser debug report construction.

use crate::browser_agent::events::PageEventKind;
use crate::browser_agent::page::BrowserPage;

use super::types::{BrowserDebugReport, BrowserParityTarget};

pub fn build(page: &BrowserPage) -> BrowserDebugReport {
    let page_errors = page.page_errors();
    let console_errors = page
        .console_events()
        .iter()
        .filter(|event| event.level == "error")
        .map(|event| event.message.clone())
        .collect::<Vec<_>>();
    BrowserDebugReport {
        url: page.session.url.clone(),
        parity: BrowserParityTarget {
            target: "full native browser parity".into(),
            native_engine: true,
        },
        console_errors,
        page_errors: page_errors.clone(),
        mapped_page_errors: super::mapped_errors::collect(page, &page_errors),
        runtime_exceptions: super::exceptions::collect(page, &page_errors),
        failed_requests: failed_requests(page),
        network_har: super::har::entries(page),
        visual_elements: super::visual::collect(page),
        source_maps: super::source_maps::statuses(page),
        frameworks: super::react::frameworks(page),
        react: super::react::summary(page),
    }
}

fn failed_requests(page: &BrowserPage) -> Vec<String> {
    page.event_log()
        .iter()
        .filter(|event| event.kind == PageEventKind::Network)
        .filter(|event| event.message.contains(" 4") || event.message.contains(" aborted"))
        .map(|event| format!("{} {}", event.action, event.message))
        .collect()
}
