use tetherscript::browser_js::run_html_scripts_at_url;
use tetherscript::js::JsValue;

#[test]
fn run_scripts_at_url_seeds_window_location() {
    let result = run_html_scripts_at_url(
        "<main></main><script>location.pathname + location.search + location.hash;</script>",
        "https://agent.test/login?next=%2F#top",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("/login?next=%2F#top".into()));
    assert_eq!(result.state.url, "https://agent.test/login?next=%2F#top");
}
