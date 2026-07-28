use tetherscript::js;

#[test]
fn uri_globals_cover_bundle_url_helpers() {
    let source = "decodeURI('/a%20b?q=1%26x')+':'+\
        decodeURIComponent('q%3D1%26x')+':'+encodeURIComponent('a b&c');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("/a b?q=1%26x:q=1&x:a%20b%26c".into())
    );
}
