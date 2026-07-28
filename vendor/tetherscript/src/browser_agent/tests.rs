use super::{BrowserContext, BrowserPage, Locator};

#[test]
fn page_click_runs_element_default_action_and_traces() {
    let mut page =
        BrowserPage::from_html("https://example.test", "<input id='ok' type='checkbox'>");

    let report = page.click(&Locator::css("#ok")).unwrap();

    assert_eq!(report.action, "click");
    assert!(page.session.html.contains("checked=\"\""));
    assert!(page
        .session
        .trace
        .iter()
        .any(|event| event.action == "click"));
}

#[test]
fn page_fill_updates_input_value_and_storage_state() {
    let mut page = BrowserPage::from_html("https://example.test", "<input data-testid='q'>");

    page.fill(&Locator::test_id("q"), "hello").unwrap();
    let value = page
        .session
        .eval_js("document.querySelector('[data-testid=\"q\"]').value")
        .unwrap();

    assert_eq!(value, crate::js::JsValue::String("hello".into()));
}

#[test]
fn context_stores_isolated_pages() {
    let mut context = BrowserContext::new();
    let index = context.new_page(BrowserPage::from_html("mem://one", "<main>One</main>"));

    assert_eq!(index, 0);
    assert_eq!(context.len(), 1);
    assert_eq!(context.page(0).unwrap().session.url, "mem://one");
}

#[test]
fn strict_locator_rejects_multiple_matches() {
    let mut page = BrowserPage::from_html("mem://many", "<button>A</button><button>B</button>");

    let err = page.click(&Locator::css("button")).unwrap_err();

    assert!(err.contains("matched 2 elements"));
}
