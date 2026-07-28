use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[test]
fn document_static_metadata_matches_browser_defaults() {
    let result = eval_with_dom(
        "<main></main>",
        "[
            document.characterSet,
            document.charset,
            document.contentType,
            document.lastModified
        ].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("UTF-8|UTF-8|text/html|01/01/1970 00:00:00".into())
    );
}

#[test]
fn document_location_alias_tracks_window_location() {
    let result = eval_with_dom(
        "<main></main>",
        "document.location===window.location && document.location.href===location.href;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::Bool(true));
}

#[test]
fn document_default_view_points_at_window() {
    let result = eval_with_dom(
        "<main></main>",
        "document.defaultView===window && document.defaultView.document===document;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::Bool(true));
}

#[test]
fn document_visibility_metadata_is_visible_by_default() {
    let result = eval_with_dom(
        "<main></main>",
        "[document.hidden,document.visibilityState,document.prerendering].join('|');",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("false|visible|false".into()));
}
