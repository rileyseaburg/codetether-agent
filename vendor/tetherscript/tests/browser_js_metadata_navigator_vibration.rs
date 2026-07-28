use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[test]
fn navigator_vibration_records_supported_patterns_only() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=navigator.vibrate(25); let first=navigator.__lastVibration;\
         let b=navigator.vibrate([10,20]); let second=navigator.__lastVibration.join(',');\
         let c=navigator.vibrate(null); let d=navigator.vibrate();\
         [first,a,b,second,c,d,navigator.__lastVibration.join(',')].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("25|true|true|10,20|false|false|10,20".into())
    );
}
