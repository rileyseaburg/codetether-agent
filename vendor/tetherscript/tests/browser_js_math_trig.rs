use tetherscript::js;

#[test]
fn math_trig_helpers_cover_canvas_geometry_bundles() {
    let source = "Math.sin(Math.PI/2)+':'+Math.cos(0)+':'+(Math.atan2(1,1)>0);";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("1:1:true".into())
    );
}

#[test]
fn math_constants_cover_decimal_library_config() {
    let source = "'2.302585092994046' == Math.LN10 && Math.LOG10E > 0 && Math.SQRT2 > 1;";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::Bool(true));
}

#[test]
fn math_log_helpers_cover_react_scheduler_fallbacks() {
    let source =
        "Math.log(8)/Math.LN2+':'+Math.log2(8)+':'+Math.log10(100)+':'+Math.round(Math.exp(1));";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("3:3:2:3".into())
    );
}
