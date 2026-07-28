use crate::browser_agent::{BrowserPage, Locator};

#[test]
fn expect_text_retries_after_timer_mutates_dom() {
    let html = "<p id='msg'>loading</p><script>setTimeout(function(){ document.getElementById('msg').textContent='ready'; }, 0);</script>";
    let mut page = BrowserPage::from_html("mem://assert-text", html);
    page.set_default_timeout_ticks(1);

    page.expect_text(&Locator::css("#msg"), "ready").unwrap();
    page.expect_text_contains(&Locator::css("#msg"), "read")
        .unwrap();
}

#[test]
fn expect_text_timeout_reports_expected_and_last_text() {
    let mut page = BrowserPage::from_html("mem://assert-timeout", "<p id='msg'>loading</p>");
    page.set_default_timeout_ticks(0);

    let err = page
        .expect_text_contains(&Locator::css("#msg"), "ready")
        .unwrap_err();

    assert!(err.contains("expect_text_contains timed out after 0 ticks"));
    assert!(err.contains("expected text containing \"ready\""));
    assert!(err.contains("last text \"loading\""));
}
