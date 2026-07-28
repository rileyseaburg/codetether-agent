use crate::browser_agent::{BrowserPage, Locator};

#[test]
fn computed_style_respects_specificity_and_inline() {
    let html = "<style>#go{color:green}button{color:blue}</style>\
                <button id='go' style='color:red'>Save</button>";
    let mut page = BrowserPage::from_html("mem://cssom", html);

    let style = page.computed_style(&Locator::css("#go")).unwrap();

    assert_eq!(style.get("color"), Some("red"));
    assert_eq!(style.get("display"), Some("inline-block"));
}

#[test]
fn later_external_stylesheet_wins_same_specificity() {
    let html = "<style>.box{color:red}</style>\
                <link rel='stylesheet' href='/app.css'>\
                <main id='box' class='box'></main>";
    let mut page = BrowserPage::from_html("https://example.test/app", html);
    page.register_stylesheet_resource("/app.css", ".box{color:green}");

    let value = page.style_property(&Locator::css("#box"), "COLOR").unwrap();

    assert_eq!(value.as_deref(), Some("green"));
}
