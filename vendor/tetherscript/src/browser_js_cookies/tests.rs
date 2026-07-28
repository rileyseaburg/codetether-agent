use super::super::{eval_with_dom_state, BrowserJsState};
use crate::js::JsValue;

fn seeded(script: &str) -> super::super::BrowserJsResult {
    eval_with_dom_state(
        "<main></main>",
        script,
        BrowserJsState {
            cookies: vec![
                ("sid".into(), "abc".into()),
                ("theme".into(), "dark".into()),
            ],
            ..BrowserJsState::default()
        },
    )
    .unwrap()
}

#[test]
fn cookie_store_reads_seeded_visible_cookies() {
    let result = seeded(
        "let out='';\
         cookieStore.get('sid').then(function(c){out=c.name+'='+c.value;});\
         cookieStore.getAll().then(function(a){out=out+':'+a.length+':'+a[1].name;});\
         out;",
    );
    assert_eq!(result.value, JsValue::String("sid=abc:2:theme".into()));
}

#[test]
fn cookie_store_writes_refresh_document_cookie_projection() {
    let result = seeded(
        "let out='';\
         window.cookieStore.set('sid','new').then(function(v){out=typeof v;});\
         cookieStore.set({name:'theme',value:'light'});\
         cookieStore.delete({name:'sid'});\
         out+':'+document.cookie;",
    );
    assert_eq!(
        result.value,
        JsValue::String("undefined:theme=light".into())
    );
    assert_eq!(result.state.cookies, vec![("theme".into(), "light".into())]);
}
