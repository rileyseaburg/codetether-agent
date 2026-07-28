use crate::browser_agent::{BrowserPage, ColorScheme, ForcedColors, Locator};

#[test]
fn media_queries_follow_viewport_and_color_scheme() {
    let html = "<style>\
        #box{color:black;background:white}\
        @media (min-width: 100px){#box{color:purple}}\
        @media (prefers-color-scheme: dark){#box{background:black}}\
        </style><main id='box'></main>";
    let mut page = BrowserPage::from_html("mem://media", html);
    page.set_viewport_size(120, 80).unwrap();
    page.set_color_scheme(ColorScheme::Dark);

    let style = page.computed_style(&Locator::css("#box")).unwrap();

    assert_eq!(style.get("color"), Some("purple"));
    assert_eq!(style.get("background"), Some("black"));
}

#[test]
fn inactive_media_blocks_are_ignored() {
    let html = "<style>\
        #box{outline:solid}\
        @media (max-width: 20px){#box{outline:none}}\
        @media (forced-colors: active){#box{color:CanvasText}}\
        </style><main id='box'></main>";
    let mut page = BrowserPage::from_html("mem://media", html);
    page.set_viewport_size(80, 40).unwrap();

    assert_eq!(
        page.style_property(&Locator::css("#box"), "outline")
            .unwrap(),
        Some("solid".into())
    );
    assert_eq!(
        page.style_property(&Locator::css("#box"), "color").unwrap(),
        Some("black".into())
    );
    page.set_forced_colors(ForcedColors::Active);
    assert_eq!(
        page.style_property(&Locator::css("#box"), "color").unwrap(),
        Some("CanvasText".into())
    );
}
