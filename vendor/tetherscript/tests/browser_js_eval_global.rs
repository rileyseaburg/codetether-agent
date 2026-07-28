use tetherscript::js;

#[test]
fn eval_global_runs_in_current_environment() {
    let source = "let x=2; eval('x = x + 3'); x;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Number(5.0));
}
