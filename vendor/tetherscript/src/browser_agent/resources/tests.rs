use crate::browser_agent::{BrowserPage, Locator, ResourceKind};

#[test]
fn external_script_resource_mutates_dom() {
    let html = "<main id='out'></main><script src='/app.js'></script>";
    let mut page = BrowserPage::from_html("https://example.test/index.html", html);

    page.register_script_resource(
        "/app.js",
        "document.getElementById('out').textContent = 'loaded';",
    );
    page.run_scripts().unwrap();

    assert!(page.session.html.contains(">loaded<"));
}

#[test]
fn external_stylesheet_resource_affects_layout() {
    let html = "<link rel='stylesheet' href='/app.css'><main id='box'>x</main>";
    let mut page = BrowserPage::from_html("https://example.test/index.html", html);

    page.register_stylesheet_resource("/app.css", "#box { width: 4px; height: 3px; }");
    page.run_scripts().unwrap();
    let bounds = page.bounding_box(&Locator::css("#box")).unwrap();

    assert_eq!((bounds.width, bounds.height), (4, 3));
}

#[test]
fn image_resource_metadata_is_inspectable() {
    let mut page = BrowserPage::from_html("https://example.test", "<img src='/logo.png'>");

    page.register_image_resource("/logo.png", vec![1, 2, 3, 4]);
    let resources = page.resources();
    let images = page.image_resource_metadata();

    assert_eq!(resources[0].kind, ResourceKind::Image);
    assert_eq!(images[0].url, "/logo.png");
    assert_eq!(images[0].byte_len, 4);
}

#[test]
fn missing_external_script_returns_named_error() {
    let html = "<script src='/missing.js'></script>";
    let mut page = BrowserPage::from_html("https://example.test/index.html", html);

    let err = page.run_scripts().unwrap_err();

    assert!(err.contains("missing external script resource: /missing.js"));
}
