use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[test]
fn navigator_share_accepts_and_records_payload() {
    let result = eval_with_dom(
        "<main></main>",
        "let data={text:'hello',url:'https://example.test'}; let seen='';\
         let p=navigator.share(data); p.then(function(v){ seen=''+v; });\
         [navigator.canShare(data),navigator.canShare({files:[]}),navigator.canShare({}),\
         navigator.canShare('x'),navigator.canShare(),navigator.__lastShare.text,navigator.__lastShare.url,\
         p.__promise_state,seen].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "true|true|false|false|false|hello|https://example.test|fulfilled|undefined".into()
        )
    );
}

#[test]
fn navigator_share_rejects_and_is_catchable_when_unshareable() {
    let result = eval_with_dom(
        "<main></main>",
        "let seen=''; let p=navigator.share({});\
         let next=p.catch(function(e){ seen=e; return 'handled'; });\
         [p.__promise_state,p.__promise_reason,seen,next.__promise_state,\
         next.__promise_value].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "rejected|navigator.share: data is not shareable|navigator.share: data is not \
             shareable|fulfilled|handled"
                .into()
        )
    );
}
