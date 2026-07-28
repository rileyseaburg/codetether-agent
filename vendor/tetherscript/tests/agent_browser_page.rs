use tetherscript::browser_agent::{BrowserPage, Locator, PageEventKind};

#[test]
fn page_contract_supports_agent_observe_act_assert_loop() {
    let html = "<button id='save'>old</button><script>setTimeout(function(){let b=document.getElementById('save');b.textContent='ready';console.log('ready');},0);</script>";
    let mut page = BrowserPage::from_html("https://agent.test/app", html);
    page.set_default_timeout_ticks(1);

    page.run_scripts().unwrap();
    page.expect_text(&Locator::css("#save"), "ready").unwrap();
    page.click(&Locator::text_exact("ready")).unwrap();
    let image = page.screenshot().unwrap();

    assert_eq!(page.session.url, "https://agent.test/app");
    assert_eq!((image.width, image.height), (80, 24));
    assert_eq!(page.console_events()[0].message, "ready");
    assert!(page
        .session
        .trace
        .iter()
        .any(|entry| entry.action == "click"));
    assert!(page
        .event_log()
        .iter()
        .any(|event| event.kind == PageEventKind::Console));
}
