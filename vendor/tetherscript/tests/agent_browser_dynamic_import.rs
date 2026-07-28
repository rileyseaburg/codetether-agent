use tetherscript::browser_agent::BrowserPage;

#[test]
fn dynamic_import_executes_registered_module_namespace() {
    let html = concat!(
        "<main id='out'></main>",
        "<script type='module' src='/assets/app.js'></script>",
    );
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);

    page.register_script_resource(
        "/assets/app.js",
        "import('./lazy.js').then((mod) => { mod.boot(); });",
    );
    page.register_script_resource(
        "https://app.test/assets/lazy.js",
        "export const boot = () => { document.getElementById('out').textContent = 'lazy'; };",
    );
    page.run_scripts().unwrap();

    assert!(page.session.html.contains(">lazy<"));
}

#[test]
fn validation_reports_missing_dynamic_import_chunk() {
    let html = "<script type='module' src='/assets/app.js'></script>";
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);

    page.register_script_resource("/assets/app.js", "import('./missing.js');");
    let err = page.validate_external_resources().unwrap_err();

    assert!(err.contains("missing external script resource: ./missing.js"));
    assert!(err.contains("https://app.test/assets/missing.js"));
}
