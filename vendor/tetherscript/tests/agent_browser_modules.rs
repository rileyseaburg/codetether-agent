use tetherscript::browser_agent::BrowserPage;

#[test]
fn module_script_executes_static_import_graph() {
    let html = concat!(
        "<main id='out'></main>",
        "<script type='module' src='/assets/app.js'></script>",
    );
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);

    page.register_script_resource(
        "/assets/app.js",
        "import { boot as start } from './chunk.js'; start();",
    );
    page.register_script_resource(
        "https://app.test/assets/chunk.js",
        "export function boot(){ document.getElementById('out').textContent = 'booted'; }",
    );
    page.run_scripts().unwrap();

    assert!(page.session.html.contains(">booted<"));
}

#[test]
fn missing_static_import_reports_resolved_chunk() {
    let html = "<script type='module' src='/assets/app.js'></script>";
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);

    page.register_script_resource("/assets/app.js", "import './missing.js';");
    let err = page.run_scripts().unwrap_err();

    assert!(err.contains("missing external script resource: ./missing.js"));
    assert!(err.contains("https://app.test/assets/missing.js"));
}

#[test]
fn resource_validation_checks_static_module_imports() {
    let html = "<script type='module' src='/assets/app.js'></script>";
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);

    page.register_script_resource("/assets/app.js", "import './missing.js';");
    let err = page.validate_external_resources().unwrap_err();

    assert!(err.contains("missing external script resource: ./missing.js"));
    assert!(err.contains("https://app.test/assets/missing.js"));
}
