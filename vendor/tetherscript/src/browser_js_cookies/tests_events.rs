use super::super::{eval_with_dom_state, BrowserJsState};
use crate::js::JsValue;

#[test]
fn cookie_store_missing_and_event_probe_are_deterministic() {
    let result = eval_with_dom_state(
        "<main></main>",
        "let out='';\
         cookieStore.get('missing').then(function(c){out=typeof c;});\
         cookieStore.addEventListener('change',function(){});\
         cookieStore.removeEventListener('change',function(){});\
         out+':'+(window.cookieStore===cookieStore)+':'\
         +(cookieStore.onchange===null)+':'+cookieStore.dispatchEvent({type:'change'});",
        BrowserJsState {
            cookies: vec![("sid".into(), "abc".into())],
            ..BrowserJsState::default()
        },
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("undefined:true:true:true".into())
    );
}
