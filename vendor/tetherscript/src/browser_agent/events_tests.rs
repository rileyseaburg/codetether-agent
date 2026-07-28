//! Tests for page console and error event capture.

use super::*;

#[test]
fn console_events_are_captured_from_eval_and_scripts() {
    let mut page = BrowserPage::from_html(
        "mem://events",
        "<main><script>console.log('inline')</script></main>",
    );
    page.run_scripts().unwrap();
    page.eval_js("console.log('eval')").unwrap();

    let messages: Vec<_> = page
        .console_events()
        .iter()
        .map(|event| event.message.as_str())
        .collect();
    assert_eq!(messages, vec!["inline", "eval"]);
}

#[test]
fn page_errors_capture_eval_and_script_failures() {
    let mut eval_page = BrowserPage::from_html("mem://events", "<main></main>");
    let eval_error = eval_page.eval_js("missing_call()").unwrap_err();
    assert_eq!(eval_page.page_errors()[0].action, "page.eval_js");
    assert_eq!(eval_page.page_errors()[0].message, eval_error);

    let mut script_page =
        BrowserPage::from_html("mem://events", "<script>missing_script_call()</script>");
    let script_error = script_page.run_scripts().unwrap_err();
    assert_eq!(script_page.page_errors()[0].action, "page.run_scripts");
    assert_eq!(script_page.page_errors()[0].message, script_error);
}

#[test]
fn event_log_order_is_stable() {
    let mut page = BrowserPage::from_html("mem://events", "<main></main>");
    page.eval_js("console.log('a')").unwrap();
    page.eval_js("missing_call()").unwrap_err();
    page.eval_js("console.log('b'); fetch('/api/order');")
        .unwrap();

    let log = page.event_log();
    assert_eq!(
        log.iter().map(|event| event.sequence).collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    assert_eq!(log[0].kind, PageEventKind::Console);
    assert_eq!(log[1].kind, PageEventKind::PageError);
    assert_eq!(log[2].message, "b");
    assert_eq!(log[3].action, "GET");
    assert_eq!(log[3].message, "mem://events/api/order 200");
}
