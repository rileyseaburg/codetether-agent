//! React and framework diagnostics from native page state.

use crate::browser::query_selector;
use crate::browser_agent::page::BrowserPage;

use super::types::ReactDebugSummary;

pub fn frameworks(page: &BrowserPage) -> Vec<String> {
    let mut out = Vec::new();
    if summary(page).detected {
        out.push("react".into());
    }
    if page.session.html.contains("__NEXT_DATA__") || has(page, "#__next") {
        out.push("next".into());
    }
    if page.session.html.contains("data-vite") || page.session.html.contains("/@vite/") {
        out.push("vite".into());
    }
    out
}

pub fn summary(page: &BrowserPage) -> ReactDebugSummary {
    let roots = roots(page);
    let hydration_warnings = page
        .console_events()
        .iter()
        .filter(|event| warning_text(&event.message))
        .map(|event| event.message.clone())
        .collect::<Vec<_>>();
    ReactDebugSummary {
        detected: !roots.is_empty() || page.session.html.contains("React"),
        roots,
        hydration_warnings,
    }
}

fn roots(page: &BrowserPage) -> Vec<String> {
    ["#root", "#app", "#__next", "[data-reactroot]"]
        .into_iter()
        .filter(|selector| has(page, selector))
        .map(str::to_string)
        .collect()
}

fn has(page: &BrowserPage, selector: &str) -> bool {
    !query_selector(&page.session.document, selector).is_empty()
}

fn warning_text(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("hydration")
        || lower.contains("server rendered")
        || lower.contains("did not match")
}
