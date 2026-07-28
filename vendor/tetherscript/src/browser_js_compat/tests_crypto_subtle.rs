use super::super::*;

#[test]
fn crypto_subtle_key_methods_are_rejected_thenables() {
    let result = eval_with_dom(
        "<main></main>",
        "let seen='';let p=crypto.subtle.importKey('raw',Uint8Array(1),{},false,[]);\
         let next=p.catch(function(e){seen=e;return 'handled';});\
         [typeof crypto.subtle.generateKey,p.__promise_state,p.__promise_reason,\
         seen,next.__promise_state,next.__promise_value].join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String(
            "function|rejected|crypto.subtle.importKey: unsupported|crypto.subtle.importKey: \
             unsupported|fulfilled|handled"
                .into()
        )
    );
}

#[test]
fn crypto_subtle_unsupported_then_without_handler_stays_rejected() {
    let result = eval_with_dom(
        "<main></main>",
        "let p=crypto.subtle.sign({}, {}, Uint8Array(1));\
         let next=p.then(null);[p.__promise_state,next.__promise_state,\
         next.__promise_reason].join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String(
            "rejected|rejected|crypto.subtle.sign: unsupported".into()
        )
    );
}
