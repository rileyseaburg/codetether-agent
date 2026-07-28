use super::super::*;

#[test]
fn history_forward_go_and_push_truncate_forward_entries() {
    let result = eval_with_dom(
        "<main></main>",
        "let seen='';let hashes=0;\
         window.addEventListener('popstate',function(e){seen=seen+e.state.page+location.pathname+';';});\
         window.addEventListener('hashchange',function(){hashes=hashes+1;});\
         history.pushState({page:1},'', '/one#x');\
         history.pushState({page:2},'', '/two#y');\
         history.back();history.forward();history.go(-1);\
         let before=seen+'|'+hashes+'|'+history.state.page+':'+location.pathname+':'+history.length;\
         history.pushState({page:3},'', '/three');before+'|'+history.length+':'+history.state.page;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("1/one;2/two;1/one;|3|1:/one:3|3:3".into())
    );
}
