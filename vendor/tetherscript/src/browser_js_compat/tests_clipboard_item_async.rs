use super::super::*;

#[test]
fn clipboard_item_get_type_returns_blob_text_and_type() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let out=''; let item=ClipboardItem({'text/plain':'hello'}); \
         item.getType('text/plain').then(function(blob){ \
         blob.text().then(function(text){ out=text+'|'+blob.type+'|'+blob.size;\
         document.getElementById('out').textContent=out; }); }); out;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("".into()));
    assert!(result.html.contains("hello|text/plain|5"));
}

#[test]
fn clipboard_item_preserves_blob_bytes_and_type() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let out=''; let source=Blob(['AB'],{type:'text/custom'}); \
         let item=ClipboardItem({'image/png':source}); \
         item.getType('image/png').then(function(blob){ \
         blob.text().then(function(text){ out=text+'|'+blob.type+'|'+blob.size;\
         document.getElementById('out').textContent=out; }); }); out;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("".into()));
    assert!(result.html.contains("AB|text/custom|2"));
}
