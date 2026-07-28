use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[test]
fn navigator_common_capability_probes_are_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "[navigator.hardwareConcurrency,navigator.deviceMemory,navigator.language,\
         navigator.languages.join(','),navigator.onLine,navigator.cookieEnabled,\
         navigator.platform,navigator.userAgent].join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String(
            "4|8|en-US|en-US,en|true|true|TetherScript|TetherScript/0.1 BrowserCompat".into()
        )
    );
}

#[test]
fn navigator_storage_methods_return_synchronous_thenables() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=''; let b=''; let c=''; let d='';\
         navigator.storage.estimate().then(function(e){ a=e.quota+':'+e.usage; });\
         navigator.storage.persisted().then(function(v){ b=''+v; });\
         navigator.storage.persist().then(function(v){ c=''+v; });\
         navigator.storage.persisted().then(function(v){ d=''+v; });\
         a+'|'+b+'|'+c+'|'+d;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("67108864:0|false|true|true".into())
    );
}

#[test]
fn navigator_locks_request_invokes_callback_and_resolves_result() {
    let result = eval_with_dom(
        "<main></main>",
        "let seen=''; let done='';\
         navigator.locks.request('db',function(lock){ seen=lock.name+':'+lock.mode; return 'ok'; })\
         .then(function(v){ done=v; }); seen+'|'+done;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("db:exclusive|ok".into()));
}
