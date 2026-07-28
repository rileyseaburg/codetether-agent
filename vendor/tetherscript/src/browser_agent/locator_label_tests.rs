use super::{BrowserPage, Locator};

#[test]
fn label_locator_resolves_associated_input() {
    let mut page = BrowserPage::from_html(
        "mem://labels",
        "<label for='email'>Email address</label><input id='email'>",
    );

    page.fill(&Locator::label("Email address"), "a@example.test")
        .unwrap();

    assert!(page.session.html.contains("value=\"a@example.test\""));
}

#[test]
fn strict_role_diagnostics_include_locator_description() {
    let mut page = BrowserPage::from_html(
        "mem://strict",
        "<button aria-label='Save'></button><button>Save draft</button>",
    );
    let err = page
        .click(&Locator::role_name("button", "Save"))
        .unwrap_err();

    assert!(err.contains("get_by_role"));
    assert!(err.contains("matched 2 elements"));
}
