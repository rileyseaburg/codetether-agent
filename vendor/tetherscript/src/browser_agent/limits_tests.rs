use crate::browser_agent::{BrowserPage, BrowserResourceLimits};

#[test]
fn default_limits_expose_guard_metadata() {
    let page = BrowserPage::from_html("mem://limits", "<main>ok</main>");
    let metadata = page.guard_metadata();

    assert_eq!(metadata.dom_bytes, page.session.html.len());
    assert!(metadata.max_action_attempts > 0);
    assert!(metadata.max_dom_bytes >= metadata.dom_bytes);
}

#[test]
fn custom_limits_are_stored() {
    let mut page = BrowserPage::from_html("mem://limits", "<main>ok</main>");
    let mut limits = BrowserResourceLimits::default();
    limits.max_action_attempts = 2;
    limits.max_action_ticks = 8;
    limits.max_dom_bytes = 64;
    limits.max_trace_entries = 3;

    page.set_resource_limits(limits);

    assert_eq!(page.resource_limits(), limits);
    assert_eq!(page.wait_options.timeout_ticks, 1);
    assert_eq!(page.guard_metadata().max_trace_entries, 3);
}

#[test]
fn eval_fails_when_dom_exceeds_limit() {
    let mut page = BrowserPage::from_html("mem://limits", "<main>too large</main>");
    let mut limits = page.resource_limits();
    limits.max_dom_bytes = 4;
    page.set_resource_limits(limits);

    let error = page.eval_js("1").unwrap_err();

    assert!(error.contains("page.eval_js blocked"));
    assert!(error.contains("max DOM bytes 4"));
}
