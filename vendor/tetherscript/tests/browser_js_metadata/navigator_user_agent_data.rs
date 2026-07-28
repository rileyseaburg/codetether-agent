use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[test]
fn navigator_user_agent_data_is_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "let ua=navigator.userAgentData; let out='';\
         ua.getHighEntropyValues(['architecture','uaFullVersion','wow64'])\
         .then(function(v){ out=[ua.brands[0].brand,ua.brands[0].version,\
         ua.mobile,ua.platform,v.architecture,v.uaFullVersion,v.wow64].join('|'); });\
         out;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("TetherScript|0.1|false|TetherScript|x86|0.1|false".into())
    );
}

#[test]
fn navigator_user_agent_data_to_json_returns_base_fields() {
    let result = eval_with_dom(
        "<main></main>",
        "let v=navigator.userAgentData.toJSON();\
         [v.brands.length,v.brands[1].brand,v.mobile,v.platform].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("2|BrowserCompat|false|TetherScript".into())
    );
}
