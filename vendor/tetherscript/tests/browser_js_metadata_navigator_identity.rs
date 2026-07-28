use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[test]
fn navigator_legacy_identity_fields_are_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "[
            navigator.appCodeName,
            navigator.appName,
            navigator.appVersion,
            navigator.doNotTrack,
            navigator.pdfViewerEnabled,
            navigator.userActivation.isActive,
            navigator.userActivation.hasBeenActive,
            navigator.plugins.length,
            navigator.mimeTypes.length,
            navigator.javaEnabled()
        ].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "Mozilla|Netscape|TetherScript/0.1 BrowserCompat||false|false|false|0|0|false".into()
        )
    );
}
