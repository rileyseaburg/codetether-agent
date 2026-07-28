use tetherscript::browser_agent::BrowserPage;

#[test]
fn validates_registered_production_assets() {
    let html = concat!(
        "<link rel='stylesheet' href='/app.css'>",
        "<link rel='modulepreload' href='/assets/chunk.js'>",
        "<script type='module' src='/assets/app.js'></script>",
        "<img src='logo.png'>",
    );
    let mut page = BrowserPage::from_html("https://app.test/app/index.html", html);

    page.register_stylesheet_resource("/app.css", "body { color: black; }");
    page.register_script_resource("https://app.test/assets/chunk.js", "window.chunk = true;");
    page.register_script_resource("/assets/app.js", "window.app = true;");
    page.register_image_resource("https://app.test/app/logo.png", vec![1, 2, 3]);

    page.validate_external_resources().unwrap();
}

#[test]
fn reports_missing_modulepreload_asset() {
    let html = concat!(
        "<link rel='modulepreload' href='/assets/chunk.js'>",
        "<script type='module' src='/assets/app.js'></script>",
    );
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);

    page.register_script_resource("/assets/app.js", "window.app = true;");
    let err = page.validate_external_resources().unwrap_err();

    assert!(err.contains("script /assets/chunk.js"));
    assert!(err.contains("https://app.test/assets/chunk.js"));
}

#[test]
fn external_module_script_executes_in_document_order() {
    let html = concat!(
        "<main id='out'></main>",
        "<script>window.order = 'a';</script>",
        "<script type='module' src='/assets/app.js'></script>",
        "<script>document.getElementById('out').textContent = window.order;</script>",
    );
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);

    page.register_script_resource("/assets/app.js", "window.order = window.order + 'b';");
    page.run_scripts().unwrap();

    assert!(page.session.html.contains(">ab<"));
}
