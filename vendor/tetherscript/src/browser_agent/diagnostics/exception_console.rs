//! Console-error exception projection.

use crate::browser_agent::page::BrowserPage;

use super::exception_kind::RuntimeExceptionKind;
use super::exception_types::RuntimeException;

pub fn collect(page: &BrowserPage) -> Vec<RuntimeException> {
    page.console_events()
        .iter()
        .filter(|event| event.level == "error")
        .filter_map(|event| exception(&event.message))
        .collect()
}

fn exception(message: &str) -> Option<RuntimeException> {
    let kind = super::exception_classify::kind(message);
    (kind != RuntimeExceptionKind::Other).then(|| RuntimeException {
        action: "console.error".into(),
        message: message.into(),
        kind,
    })
}
