use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[test]
fn navigator_connection_is_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "let c=navigator.connection;\
         [c.effectiveType,c.type,c.downlink,c.downlinkMax,c.rtt,c.saveData,c.onchange].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("4g|wifi|10|10|50|false|".into())
    );
}

#[test]
fn navigator_connection_event_methods_are_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "let c=navigator.connection;let seen='';\
         function gone(){seen=seen+'gone';}\
         c.addEventListener('change',function(e){seen=seen+'L'+e.type+':'\
         +(e.target===c)+':' +(this===c)+';';});\
         c.addEventListener('change',gone);c.removeEventListener('change',gone);\
         c.onchange=function(e){seen=seen+'H'+(e.currentTarget===c)+';';};\
         let add=c.addEventListener('other',function(){});\
         let rem=c.removeEventListener('other',function(){});\
         [c===navigator.networkInformation,typeof c.addEventListener,\
         typeof c.removeEventListener,typeof c.dispatchEvent,''+add,''+rem,\
         c.dispatchEvent(Event('change')),seen].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "true|function|function|function|undefined|undefined|true|\
             Lchange:true:true;Htrue;"
                .into()
        )
    );
}
