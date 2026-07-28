use super::super::*;

#[test]
fn history_state_reflects_push_and_replace() {
    let result = eval_with_dom(
        "<main></main>",
        "history.pushState({page:1},'', '/one');\
         let first=history.state.page+':' +history.length;\
         history.replaceState({page:2},'', '/two');\
         first+'|' +location.pathname+':' +history.state.page+':' +history.length;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("1:2|/two:2:2".into()));
}

#[test]
fn history_state_url_is_optional() {
    let result = eval_with_dom(
        "<main></main>",
        "let before=location.href; history.pushState({page:1},'');\
         history.replaceState({page:2},'');\
         before+':' +location.href+':' +history.state.page+':' +history.length;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("http://localhost/:http://localhost/:2:2".into())
    );
}

#[test]
fn history_state_is_isolated_from_original_mutation() {
    let result = eval_with_dom(
        "<main></main>",
        "let state={items:[1]}; history.pushState(state,'','/copy');\
         state.items[0]=9; history.state.items[0]+':' +(history.state.items!==state.items);",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("1:true".into()));
}

#[test]
fn popstate_event_exposes_destination_state() {
    let result = eval_with_dom(
        "<main></main>",
        "let seen=''; history.pushState({page:1},'', '/one');\
         history.pushState({page:2},'', '/two');\
         window.addEventListener('popstate',function(e){\
         seen=e.state.page+':' +history.state.page+':' +location.pathname; e.state.page=9; });\
         history.back(); seen+':' +history.state.page;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("1:1:/one:1".into()));
}
