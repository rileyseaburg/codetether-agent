//! Deterministic navigation lifecycle log entries.

use crate::browser_agent::events::PageEventKind;
use crate::browser_agent::navigation::result::NavigationKind;
use crate::browser_agent::page::BrowserPage;

pub(crate) fn event(
    page: &mut BrowserPage,
    event: &str,
    action: &str,
    kind: NavigationKind,
    from: &str,
    to: &str,
) {
    let message = detail(page, action, kind, from, to);
    page.push_event(PageEventKind::Navigation, event, &message);
}

pub(crate) fn no_entry(page: &mut BrowserPage, action: &str, url: &str) {
    let message = format!(
        "navigation={} action={} status=NoEntry url={}",
        page.navigation.id, action, url
    );
    page.push_event(PageEventKind::Navigation, "noentry", &message);
}

fn detail(page: &BrowserPage, action: &str, kind: NavigationKind, from: &str, to: &str) -> String {
    format!(
        "navigation={} action={} kind={:?} from={} to={}",
        page.navigation.id, action, kind, from, to
    )
}
