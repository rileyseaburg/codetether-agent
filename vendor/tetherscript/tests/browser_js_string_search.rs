use tetherscript::js;

#[test]
fn string_search_supports_regex_and_string_needles() {
    let source = "'abc123'.search(/\\d+/)+':'+'abc'.search('b')+':'+'abc'.search(/z/);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("3:1:-1".into())
    );
}
