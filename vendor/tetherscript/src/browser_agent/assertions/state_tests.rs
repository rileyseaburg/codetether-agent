use crate::browser_agent::{BrowserPage, Locator};

#[test]
fn expect_count_reports_current_match_count() {
    let mut page = BrowserPage::from_html("mem://count", "<button>A</button><button>B</button>");

    page.expect_count(&Locator::css("button").relaxed(), 2)
        .unwrap();
}

#[test]
fn expect_value_retries_after_timer_mutates_input() {
    let html = "<input id='q' value='start'><script>setTimeout(function(){ document.getElementById('q').value='done'; }, 0);</script>";
    let mut page = BrowserPage::from_html("mem://value", html);
    page.set_default_timeout_ticks(1);

    page.expect_value(&Locator::css("#q"), "done").unwrap();
}

#[test]
fn expect_url_contains_checks_current_session_url() {
    let mut page = BrowserPage::from_html("https://example.test/path?q=1", "<main>ok</main>");

    page.expect_url_contains("/path").unwrap();
}

#[test]
fn expect_count_timeout_reports_last_count() {
    let mut page = BrowserPage::from_html("mem://count-fail", "<button>A</button>");
    page.set_default_timeout_ticks(0);

    let err = page
        .expect_count(&Locator::css("button").relaxed(), 2)
        .unwrap_err();

    assert!(err.contains("expected count 2"));
    assert!(err.contains("last count 1"));
}
