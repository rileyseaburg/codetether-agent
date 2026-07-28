use tetherscript::js;

#[test]
fn json_parse_and_stringify_cover_bundle_state_helpers() {
    let source = "let v=JSON.parse('{\"a\":1,\"b\":[true,null]}');\
        v.b[0]+':'+v.b[1]+':'+JSON.stringify([v.a,'x']);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("true:null:[1,\"x\"]".into())
    );
}
