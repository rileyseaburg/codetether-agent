use tetherscript::js;

#[test]
fn browser_number_predicates_coerce_like_window_globals() {
    let source = "isNaN('x')+':'+isNaN('')+':'+isFinite('4')+':'+isFinite(Infinity);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("true:false:true:false".into())
    );
}

#[test]
fn parse_int_handles_radix_and_stops_at_first_invalid_digit() {
    let source = "parseInt('ff',16)+':'+parseInt('12px',10)+':'+isNaN(parseInt('x',10));";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("255:12:true".into())
    );
}

#[test]
fn parse_float_handles_partial_decimal_tokens() {
    let source = "parseFloat('1.25px')+':'+parseFloat('.5e2')+':'+isNaN(parseFloat('x'));";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("1.25:50:true".into())
    );
}
