use tetherscript::browser_agent::BrowserPage;

#[test]
fn module_graph_rewrites_named_export_aliases() {
    let html = concat!(
        "<main id='out'></main>",
        "<script type='module' src='/assets/app.js'></script>",
    );
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);

    page.register_script_resource(
        "/assets/app.js",
        "import { R as React } from './vendor.js'; \
         document.getElementById('out').textContent = React.name;",
    );
    page.register_script_resource(
        "https://app.test/assets/vendor.js",
        "const r = { name: 'react' }; export { r as R };",
    );
    page.run_scripts().unwrap();

    assert!(page.session.html.contains(">react<"));
}
