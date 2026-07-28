use super::super::eval_with_dom;
use crate::js::JsValue;

#[test]
fn window_open_returns_deterministic_popup_proxy() {
    let result = eval_with_dom(
        "<main></main>",
        "let p=window.open('/docs','agent','width=1');let before=[p.closed,\
         p.opener===window,p.name,p.url,p.location.href,p.features,typeof p.close,\
         typeof p.focus,typeof p.blur,typeof p.postMessage].join('|');\
         let msg=p.postMessage('x','*');p.blur();let mid=p.__focused;\
         p.focus();p.close();before+'|'+(msg===undefined)+'|'+mid+'|'+\
         p.__focused+'|'+p.closed;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "false|true|agent|/docs|/docs|width=1|function|function|function|function|true|false|true|true"
                .into()
        )
    );
}
