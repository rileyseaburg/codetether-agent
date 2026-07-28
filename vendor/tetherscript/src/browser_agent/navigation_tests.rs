use super::{BrowserPage, PageLoadState};

#[test]
fn page_creation_records_loaded_navigation() {
    let page = BrowserPage::from_html("mem://nav", "<main>ready</main>");
    let navigation = page.navigation();

    assert_eq!(navigation.id, 1);
    assert_eq!(navigation.url, "mem://nav");
    assert_eq!(navigation.action, "goto_html");
    assert_eq!(page.load_state(), PageLoadState::Load);
    assert!(page
        .wait_for_load_state(PageLoadState::DomContentLoaded)
        .is_ok());
    assert!(page
        .wait_for_load_state(PageLoadState::NetworkIdle)
        .is_err());
}

#[test]
fn run_scripts_advances_to_network_idle() {
    let mut page = BrowserPage::from_html(
        "mem://script",
        "<script>window.ready = true; fetch('/ping');</script>",
    );

    page.run_scripts().unwrap();

    assert_eq!(page.load_state(), PageLoadState::NetworkIdle);
    assert!(page.wait_for_load_state(PageLoadState::NetworkIdle).is_ok());
    assert!(page
        .session
        .network
        .iter()
        .any(|event| event.url == "mem://script/ping"));
}

#[test]
fn load_goto_and_reload_replace_navigation_metadata() {
    let mut page = BrowserPage::from_html("mem://one", "<p>one</p>");
    let first = page.navigation().id;

    page.load_html("<p>same url</p>");
    assert_eq!(page.navigation().id, first + 1);
    assert_eq!(page.navigation().url, "mem://one");
    assert_eq!(page.navigation().action, "load_html");

    page.goto_html("mem://two", "<p>two</p>");
    assert_eq!(page.navigation().id, first + 2);
    assert_eq!(page.navigation().url, "mem://two");
    assert_eq!(page.navigation().action, "goto_html");

    page.reload();
    assert_eq!(page.navigation().id, first + 3);
    assert_eq!(page.navigation().url, "mem://two");
    assert_eq!(page.navigation().action, "reload");
    assert_eq!(page.load_state(), PageLoadState::Load);
}
