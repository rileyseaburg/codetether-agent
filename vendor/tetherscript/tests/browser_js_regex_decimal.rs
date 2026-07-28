use tetherscript::js;

#[test]
fn decimal_validation_regex_supports_library_number_strings() {
    let source = r#"let r=RegExp("^(\\d+(\\.\\d*)?|\\.\\d+)(e[+-]?\\d+)?$","i");
        r.test("2.3025850929940456840179914546843642076")+":"+r.test(".5e-2")+":"+r.test("x");"#;

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("true:true:false".into())
    );
}
