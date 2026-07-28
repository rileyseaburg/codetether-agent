use tetherscript::js;

#[test]
fn string_code_point_at_supports_unicode_bundle_helpers() {
    let source = "String.fromCodePoint(128187).codePointAt(0);";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(128187.0));
}
