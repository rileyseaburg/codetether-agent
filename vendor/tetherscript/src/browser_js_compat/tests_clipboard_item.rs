use super::super::*;

#[test]
fn clipboard_item_is_global_and_exposes_types() {
    let result = eval_with_dom(
        "<main></main>",
        "let item=ClipboardItem({'text/plain':'p','text/html':'h'}); \
         typeof ClipboardItem+'|'+(ClipboardItem===window.ClipboardItem)+'|'+item.types.join(',');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("function|true|text/html,text/plain".into())
    );
}

#[test]
fn clipboard_item_get_type_rejects_missing_type() {
    let result = eval_with_dom(
        "<main></main>",
        "let out=''; let item=ClipboardItem({'text/plain':'hello'}); \
         item.getType('image/png').catch(function(error){ out=error; }); out;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("ClipboardItem: missing type image/png".into())
    );
}
